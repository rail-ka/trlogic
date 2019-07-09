FROM ubuntu:18.04
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
ARG BUILD_DATE
LABEL tags="latest 0.1" \
      build_date=$BUILD_DATE \
      maintainer="Rail Khusnutdinov <mail@rail-ka.ru>"
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install curl gcc musl-tools libssl-dev -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh \
    -s -- -y --default-toolchain 1.36.0-x86_64-unknown-linux-gnu
RUN /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl
VOLUME /trlogic
WORKDIR /trlogic
CMD RUSTFLAGS=-Clinker=musl-gcc /root/.cargo/bin/cargo build --release --target=x86_64-unknown-linux-musl
