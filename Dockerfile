FROM alpine:3.12

WORKDIR /app
COPY ./target/release/zone-server /app/zone-server

EXPOSE 8080

ENTRYPOINT ["/app/zone-server"]