use std::collections::HashMap;

use image::imageops::FilterType;
use image_util::{read_image, write_image, OutputFormat};
use worker::*;
mod fetcher;
mod image_util;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

struct RouterContext {
    pub worker_context: Context,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, worker_ctx: worker::Context) -> Result<Response> {
    log_request(&req);
    utils::set_panic_hook();

    let router = Router::with_data(RouterContext {
        worker_context: worker_ctx,
    });

    router
        .get("/", |_, _| Response::ok("Hello Image Resizer!"))
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .get_async("/image", |req, ctx| async move {
            let url = req.url().unwrap();
            let query: HashMap<_, _> = url.query_pairs().collect();

            let target_url = match query.get("url") {
                Some(target_url) => target_url,
                None => return Response::error("'url' parameter is not provided.", 403),
            };

            let w = match query.get("w") {
                Some(res) => match res.parse::<u32>() {
                    Ok(width) => width,
                    Err(_) => return Response::error("'w' parameter must be a valid number.", 400),
                },
                None => return Response::error("'w' parameter not provided.", 403),
            };

            // Parse format parameter (default to PNG for backward compatibility)
            let format_str = query.get("format").map(|s| s.as_ref()).unwrap_or("png");

            // Parse quality parameter for JPEG
            let quality = query.get("quality").and_then(|q| q.parse::<u8>().ok());

            let output_format = match OutputFormat::from_string(format_str, quality) {
                Ok(format) => format,
                Err(err) => return Response::error(&format!("Invalid format: {}", err), 400),
            };

            let cache = Cache::open("cache:image_proxy".to_string()).await;

            if let Ok(Some(response)) = cache.get(&req, false).await {
                return Ok(response);
            }

            let bytes = match fetcher::fetch_image_from_url(target_url).await {
                Ok(bytes) => bytes,
                Err(_) => return Response::error("failed to fetch image.", 403),
            };

            let img = read_image(bytes);
            let resized = img.resize(w, img.height(), FilterType::Nearest);
            let output_image = match write_image(resized, output_format.clone()) {
                Ok(image) => image,
                Err(err) => return Response::error(&format!("failed to write image: {}", err), 500),
            };

            let mut header = Headers::new();
            header.append("Accept-Ranges", "bytes").unwrap();
            header.append("Content-Type", output_format.content_type()).unwrap();
            header
                .append("Content-Length", output_image.len().to_string().as_ref())
                .unwrap();
            header
                .append("Cache-Control", "public, s-maxage=2592000")
                .unwrap();

            let mut response = Response::from_bytes(output_image)?.with_headers(header);
            let cloned_response: Response = response.cloned()?;

            ctx.data.worker_context.wait_until(async move {
                let _ = cache.put(&req, cloned_response).await;
            });

            Ok(response)
        })
        .run(req, env)
        .await
}
