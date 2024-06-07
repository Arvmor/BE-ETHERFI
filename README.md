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

## .ENV SETUP
```
ENDPOINT_MONGODB = "TO_BE_FILLED"
```