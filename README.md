## How to run
### Docker:
```bash
docker build -t backend . && docker run --workdir=/app --runtime=runc -p 1337:1337 -d backend:latest
```

OR

### Rust
```bash
cargo run
```
& built-in rust test cases
```bash
cargo test
```

## Docs
Check out <a href="/docs">/docs</a> for API Documentation

## Endpoint
URL: `http://localhost:1337`
