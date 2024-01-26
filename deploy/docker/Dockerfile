FROM rust:1.75.0-alpine3.19 as builder
RUN mkdir /src
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.19.0
RUN mkdir /app
COPY --from=builder /src/target/release/yt-parallel /app/yt-parallel
WORKDIR /app
CMD ["./yt-parallel", "-V"]

