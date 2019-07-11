FROM rust:1.36
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
ARG BUILD_DATE
LABEL tags="latest 0.1" \
      build_date=$BUILD_DATE \
      maintainer="Rail Khusnutdinov <mail@rail-ka.ru>"

WORKDIR /usr/src/trlogic

COPY src ./src
COPY Cargo.toml ./src/Cargo.toml
COPY Cargo.lock ./src/Cargo.lock
COPY index.html ./src/index.html
COPY image.json ./src/image.json

RUN cargo install --path .

CMD ["trlogic"]
