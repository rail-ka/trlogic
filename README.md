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
# docker build -t trlogic-build docker/build
# docker run --rm -it -v $(pwd):/trlogic trlogic-build
# docker build -t trlogic docker
# docker run --rm -it -p 8080:8080 -v $(pwd):/usr/src/trlogic trlogic

docker build -t trlogic .
docker run --rm -it -p 8080:8080 -v $(pwd):/usr/src/trlogic trlogic
# OR
docker-compose up
```

## Проверка задания

1. Необходимо запустить контейнер командой `docker-compose up .`
2. Далее запустится контейнер с http сервером на `http://localhost:8080/`
3. Необходимо открыть страницу по ссылке выше и попробовать отправить три формы на странице.

- Первая форма - необходимо выбрать 2 изображения на компьютере и отправить.
- Вторая форма - просто нажать кнопку и форма сама отправит json данные с закодированным base64 изображением.
- Третья форма - поле уже заполнено по умолчанию, так же просто нажать кнопку отправить.

Все три формы создают две версии изображений в папке, к которой был примонтирован докер контейнер.

Данная реализация тестового задания была проверена на Rust версии 1.35, 1.36.

## P.S.

В папке docker - файлы для сборки под target x86_64-unknown-linux-musl и для запуска под alpine linux,
но его завершить так и не удалось - для работы с https нужен openssl, и он еще не корректно собирается под musl.
Нашел https://github.com/emk/rust-musl-builder, но решил что в рамках тестового задания это уже слишком.
Так же обнаружил что rustls еще не внедрен для клиентской библиотеки actix-web/awc и даже начал писать реализацию сам.
Но думаю это так же не входит в рамки тестового задания.

## Others
```bash
cargo test -- --nocapture
```
Добрый день, может ли кто-нибудь помочь с ошибкой в тестах: actix-web проект
```rust
#[test]
fn upload_from_url_test() {
    let body = BodyWithUrl {
        url: "http://diagramcenter.org/wp-content/uploads/2016/03/image.png".to_string(),
    };
    println!("step 1");
    let form = Form(body);
    println!("step 2");
    let fut = upload_from_url(form);
    println!("step 3");
    let resp = test::block_on(fut).unwrap();
    println!("step 4");
    assert_eq!(resp.status(), http::StatusCode::OK);
    }
```
вот такой простой тест, ошибка выходит на строке `let fut = upload_from_url(form);`:
```log
thread 'tests::upload_from_url_test' panicked at 'called `Result::unwrap()` on an `Err` value: SpawnError { is_shutdown: true }', src/libcore/result.rs:999:5
```

Функция `upload_from_url`:
```rust
fn upload_from_url(req: Form<BodyWithUrl>) -> impl Future<Item = HttpResponse, Error = Error> {
    let url = &req.url[..];
    let connector = Connector::new().timeout(Duration::new(1000, 0)).finish();
    let client = ClientBuilder::new()
        .connector(connector)
        .timeout(Duration::new(1000, 0))
        .finish();
    // TODO: тут тесты обрубаются зачем то((
    client
        .get(url)
        .send()
        .map_err(|err| error::ErrorInternalServerError(err))
        .and_then(|mut res| {
            println!("image getted");
            let body = res.body().limit(10000000);
            println!("body created");
            body.map_err(|err| error::ErrorInternalServerError(err))
                .and_then(|data| match image::load_from_memory(&data[..]) {
                    Ok(img) => {
                        println!("img created");
                        resize_and_save(img, "img".to_string());
                        println!("img resized");
                        Ok(HttpResponse::Ok().finish())
                    }
                    Err(e) => {
                        error!(target: "upload_from_multipart", "{}", e);
                        Ok(HttpResponse::BadRequest().finish())
                    }
                })
        })
}
```
