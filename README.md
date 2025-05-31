# Cloudflare Workers Image Resize Proxy

An image resize proxy built with Rust and designed to work with Cloudflare Workers.

It provides a way to resize images on the fly, making it useful for web apps that require image optimazation.

## Usage

To use this image resize proxy, simply construct a URL in the following format:

```
https://<your-worker-url>/image?url=<image_url>&w=<resized_width_in_pixel>&format=<output_format>&quality=<jpeg_quality>
```

### Parameters

- `url` (required): The URL of the image you want to resize
- `w` (required): The desired width of the resized image, in pixels
- `format` (optional): The output format. Supported values:
  - `png` (default) - PNG format
  - `jpeg` or `jpg` - JPEG format
  - `webp` - WebP format
- `quality` (optional): Image quality (0-100). Applies to:
  - JPEG: Controls compression quality (default: 85)
  - WebP: Accepted for API consistency but currently uses default encoding (default: 80)
  - PNG: Ignored (lossless format)

### Examples

Basic resize to PNG (default format):
```
https://<your-worker-url>/image?url=https://loremflickr.com/1000&w=200
```

Resize to JPEG with custom quality:
```
https://<your-worker-url>/image?url=https://loremflickr.com/1000&w=200&format=jpeg&quality=90
```

Resize to WebP:
```
https://<your-worker-url>/image?url=https://loremflickr.com/1000&w=200&format=webp
```

Resize to WebP with quality parameter (currently uses default encoding):
```
https://<your-worker-url>/image?url=https://loremflickr.com/1000&w=200&format=webp&quality=90
```

## License

This project is licensed under the [MIT License](LICENSE).
