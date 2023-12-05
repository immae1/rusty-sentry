# rusty-sentry

```bash
docker build -t rusty-sentry .

docker run --rm rusty-sentry
```

1. Install direnv
2. Add .envrc with export SENTRY_URL=\<replace with Project DSN\>
3. Execute `direnv allow` and `cargo run` for local build/execution
