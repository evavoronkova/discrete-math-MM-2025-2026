# Rust Project Rules

## Standards

- Use stable Rust unless explicitly needed
- Always run cargo fmt
- Always run cargo clippy
- Prefer idiomatic Rust
- Avoid unwrap in production
- Use anyhow for app errors
- Use thiserror for library errors

## Architecture

- Keep modules small
- Prefer composition over complex abstractions
- Keep async boundaries explicit

## Testing

- Add tests for public APIs
- Prefer integration tests for major flows

## Performance

- Avoid unnecessary allocations
- Avoid cloning unless required
- Prefer &str over String when possible
