FROM alpine:3.12

WORKDIR /app
COPY ./target/release/zone-server ./zone-server

EXPOSE 8080

ENTRYPOINT ["/app/zone-server"]