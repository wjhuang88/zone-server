FROM alpine:3.12

WORKDIR /app
COPY ./target/release/zone-server ./zone-server

EXPOSE 8080

RUN ["apk add", "--no-cache", "musl-dev"]
ENTRYPOINT ["/app/zone-server"]