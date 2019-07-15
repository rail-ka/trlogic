# TR Logic homework

## Проверка задания

1. Необходимо запустить контейнер командой `docker-compose up .`
2. Далее запустится контейнер с http сервером на `http://localhost:8080/`
3. Необходимо открыть страницу по ссылке выше и попробовать отправить три формы на странице.

- Первая форма - необходимо выбрать 2 изображения на компьютере и отправить.
- Вторая форма - просто нажать кнопку и форма сама отправит json данные с закодированным base64 изображением.
- Третья форма - поле уже заполнено по умолчанию, так же просто нажать кнопку отправить.

Все три формы создают две версии изображений в папке, к которой был примонтирован докер контейнер.

Данная реализация тестового задания была проверена на Rust версии 1.35, 1.36.

## Задание

Реализовать простое REST API с одним единственным методом, который загружает изображения.

Требования:
- [x] Возможность загружать несколько файлов.
- [x] Возможность принимать multipart/form-data запросы.
- [x] Возможность принимать JSON запросы с BASE64 закодированными изображениями.
- [x] Возможность загружать изображения по заданному URL (изображение размещено где-то в интернете).
- [x] Создание квадратного превью изображения размером 100px на 100px.
- [ ] (частично) Наличие модульных/интеграционных тестов.

Следующее будет плюсом:
- [x] Корректное завершение приложения при получении сигнала ОС (graceful shutdown).
- [x] Dockerfile и docker-compose.yml, которые позволяют поднять приложение единой docker-compose up командой.
- [x] CI интеграция (Travis CI, Circle CI, другие).

## Полезные ссылки

- https://docs.travis-ci.com/user/languages/rust/
- https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
- https://blog.filippo.io/easy-windows-and-linux-cross-compilers-for-macos/
- https://chr4.org/blog/2017/03/15/cross-compile-and-link-a-static-binary-on-macos-for-linux-with-cargo-and-rust/
- https://rurust.github.io/cargo-docs-ru/config.html
- https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/
- https://github.com/emk/rust-musl-builder
- https://github.com/openssl/openssl/issues/7207

## Build
```bash
docker build -t trlogic .
docker run --rm -it -p 8080:8080 -v $(pwd)/images:/usr/local/bin/images trlogic
# OR
docker-compose up
```

## Others
```bash
cargo test -- --nocapture
```

SpawnError { is_shutdown: true }
- https://stackoverflow.com/questions/54503625/why-does-calling-tokiospawn-result-in-the-panic-spawnerror-is-shutdown-tru
- https://github.com/actix/actix-web/issues/593
