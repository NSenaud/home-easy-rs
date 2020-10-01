FROM alpine:latest

COPY ./target/armv7-unknown-linux-musleabihf/release/dios .

CMD ["./dios"]
