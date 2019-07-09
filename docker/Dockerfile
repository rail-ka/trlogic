FROM alpine:3.10
MAINTAINER Rail Khusnutdinov <mail@rail-ka.ru>
ARG BUILD_DATE
LABEL tags="latest 0.1" \
      build_date=$BUILD_DATE \
      maintainer="Rail Khusnutdinov <mail@rail-ka.ru>"
# TODO: why?
#RUN apk add --update alpine-sdk
#RUN addgroup -g 1000 myapp
#RUN adduser -D -s /bin/sh -u 1000 -G myapp myapp
EXPOSE 8080
#VOLUME /trlogic
WORKDIR /trlogic
COPY ./target/x86_64-unknown-linux-musl/release/ /trlogic/
#RUN chown myapp:myapp myapp
#USER myapp
CMD ["trlogic"]
