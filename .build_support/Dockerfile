FROM rust:1.47-alpine

RUN apk update \
    & apk add --no-cache ca-certificates wget openssl-dev make cmake gcc g++ \
    & wget -q -O /etc/apk/keys/sgerrand.rsa.pub https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub \
    & wget https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.28-r0/glibc-2.28-r0.apk \
    & apk add glibc-2.28-r0.apk