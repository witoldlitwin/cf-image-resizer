// This is a placeholder worker that will be replaced by the Rust-compiled worker
export default {
  async fetch(request, env, ctx) {
    return new Response("Worker is being built...", { status: 200 });
  }
};