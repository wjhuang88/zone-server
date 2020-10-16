FROM alpine:3.12

WORKDIR /app
COPY ./target/release/zone-server /app/zone-server

EXPOSE 8080

CMD ["ls", "/app"]
ENTRYPOINT ["/app/zone-server"]