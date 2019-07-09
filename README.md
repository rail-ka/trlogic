# TR Logic homework

Репозиторий: 

## Задание

Реализовать простое REST API с одним единственным методом, который загружает изображения.

Требования:
- [x] Возможность загружать несколько файлов.
- [x] Возможность принимать multipart/form-data запросы.
- [x] Возможность принимать JSON запросы с BASE64 закодированными изображениями.
- [x] Возможность загружать изображения по заданному URL (изображение размещено где-то в интернете).
- [x] Создание квадратного превью изображения размером 100px на 100px.
- [x] Наличие модульных/интеграционных тестов.

Следующее будет плюсом:
- [x] Корректное завершение приложения при получении сигнала ОС (graceful shutdown).
- [ ] Dockerfile и docker-compose.yml, которые позволяют поднять приложение единой docker-compose up командой.
- [ ] CI интеграция (Travis CI, Circle CI, другие).

Тестовое задание должно быть предоставлено в виде ссылки на публичный репозиторий (GitHub, BitBucket, GitLab), содержащий исходный код приложения и необходимые инструкции по сборке и запуску.

git push -u origin master
https://docs.travis-ci.com/user/languages/rust/

## TODO

- написать тесты
- https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
- https://blog.filippo.io/easy-windows-and-linux-cross-compilers-for-macos/
- https://chr4.org/blog/2017/03/15/cross-compile-and-link-a-static-binary-on-macos-for-linux-with-cargo-and-rust/
- https://rurust.github.io/cargo-docs-ru/config.html
- https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/
- https://github.com/emk/rust-musl-builder
- https://github.com/openssl/openssl/issues/7207

```toml
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

## Testing

- http://localhost:8080/

## Build
```bash
docker build -t trlogic-build docker/build
docker run --rm -it -v $(pwd):/trlogic trlogic-build

docker build -t trlogic docker
docker run --rm -it trlogic
```
