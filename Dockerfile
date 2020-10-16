FROM alpine:3.12

COPY ./target/x86_64-unknown-linux-musl/release/zone-server /app/zone-server

EXPOSE 8080

WORKDIR /app
CMD ["ls", "-a"]
ENTRYPOINT ["./zone-server"]