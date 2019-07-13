FROM ekidd/rust-musl-builder:1.36.0-openssl11 AS builder
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
ARG BUILD_DATE
LABEL tags="latest 0.1" \
      build_date=$BUILD_DATE \
      maintainer="Rail Khusnutdinov <mail@rail-ka.ru>"

COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY index.html ./index.html
COPY image.json ./image.json
COPY src ./src

RUN sudo chown -R rust:rust /home/rust

RUN cargo build --release

FROM alpine:3.10
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
ARG BUILD_DATE
LABEL tags="latest 0.1" \
      build_date=$BUILD_DATE \
      maintainer="Rail Khusnutdinov <mail@rail-ka.ru>"
RUN apk update && apk upgrade
RUN apk --no-cache add ca-certificates
EXPOSE 8080
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/trlogic \
    /usr/local/bin/
CMD /usr/local/bin/trlogic
