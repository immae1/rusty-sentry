FROM rust:alpine3.18 as builder
WORKDIR /usr/src/
COPY . .
RUN apk add --no-cache pkgconfig openssl-dev musl-dev && cargo install --path .

FROM alpine:3.18.4
ENV RUST_BACKTRACE=1
COPY --from=builder /usr/src/target/release/rusty-sentry /
CMD ["./rusty-sentry"]