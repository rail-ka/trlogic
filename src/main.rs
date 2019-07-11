#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

use actix_multipart::Multipart;
use actix_web::client::ClientBuilder;
use actix_web::client::Connector;
use actix_web::http::header::{DispositionParam, DispositionType::FormData};
use actix_web::web::{get, post, Form, Json};
use actix_web::{error, App, Error, HttpResponse, HttpServer, Responder};
use futures::{Future, Stream};
use image::{DynamicImage, FilterType::Nearest};
use std::time::Duration;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct BodyWithUrl {
    url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct ImageInJson {
    image: String,
}

/// Отдает index.html файл
fn index() -> impl Responder {
    actix_files::NamedFile::open("./index.html")
}

fn json_img() -> impl Responder {
    actix_files::NamedFile::open("./image.json")
}

/// resize image to 100x100 and save both
fn resize_and_save(img: DynamicImage, name: String) -> (bool, bool) {
    let small = img.resize(100, 100, Nearest);
    let save_res = img.save(format!("{}.png", name));
    let mut a = (false, false);
    match save_res {
        Ok(_) => a.0 = true,
        Err(e) => error!(target: "resize_and_save", "{}", e.to_string()),
    }
    let save_small_res = small.save(format!("{}_small.png", name));
    match save_small_res {
        Ok(_) => a.1 = true,
        Err(e) => error!(target: "resize_and_save", "{}", e.to_string()),
    }
    a
}

/// get from json request, base64, example: {"image": "base64"}
fn upload_from_json(req: Json<ImageInJson>) -> HttpResponse {
    // decode from json {"image": "base64"}
    let decoded = base64::decode(&req.image);
    match decoded {
        Ok(vec) => match image::load_from_memory(&vec) {
            Ok(img) => {
                resize_and_save(img, "img".to_string());
                HttpResponse::Ok().finish()
            }
            Err(e) => {
                error!(target: "upload_from_json", "{}", e);
                HttpResponse::BadRequest().finish()
            }
        },
        Err(e) => {
            error!(target: "upload_from_json", "{}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}

/// get from multipart request
fn upload_from_multipart(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .and_then(|field| {
            // trace!("{}", field);
            //            Ok(HttpResponse::Ok())
            let mut name = "img".to_string();
            if let Some(content_disposition) = field.content_disposition() {
                match content_disposition.disposition {
                    FormData => {
                        for x in content_disposition.parameters {
                            match x {
                                DispositionParam::Name(param_name) => {
                                    name = param_name;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            field
                .map_err(error::ErrorInternalServerError)
                .concat2()
                .and_then(|data| match image::load_from_memory(&data[..]) {
                    Ok(img) => {
                        resize_and_save(img, name);
                        Ok(true)
                    }
                    Err(e) => {
                        error!(target: "upload_from_multipart", "{}", e);
                        Ok(false)
                    }
                })
        })
        .collect()
        .map(|res| {
            dbg!(res);
            HttpResponse::Ok().finish()
        })
}

/// upload from url
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
            let body = res.body().limit(10_000_000);
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

fn main() {
    env_logger::init();

    let server = HttpServer::new(|| {
        App::new()
            .route("/", get().to(index))
            .route("/json_img", get().to(json_img))
            .route("/upload", post().to_async(upload_from_multipart))
            .route("/upload-from-json", post().to(upload_from_json))
            .route("/upload-from-url", post().to_async(upload_from_url))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run();
    match server {
        Ok(_) => {}
        Err(e) => error!(target: "main", "{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;
    use actix_web::test;
    use actix_web::FromRequest;
    use futures;
    use std::fs;
    use tokio;

    //    #[test]
    //    fn resize_and_save_test() {
    //        match fs::read("./img.jpg") {
    //            Ok(file) => match image::load_from_memory(&file) {
    //                Ok(img) => {
    //                    assert_eq!(resize_and_save(img, "img".to_string()), (true, true));
    //                    fs::remove_file("./img.png").unwrap();
    //                    fs::remove_file("./img_small.png").unwrap();
    //                }
    //                Err(e) => panic!("{}", e),
    //            },
    //            Err(e) => panic!("{}", e),
    //        }
    //    }

    //    #[test]
    //    fn upload_from_json_test() {
    //        // TODO: не работает
    //        let file = fs::read("./image.json").unwrap();
    //        let (req, mut payload) = test::TestRequest::post().set_json(&file).to_http_parts();
    //        println!("request created");
    //        let res = test::block_on(Json::<ImageInJson>::from_request(&req, &mut payload)).unwrap();
    //        println!("res created");
    //        //        let json = Json::<ImageInJson>::from_request(&req, &mut actix_web::dev::Payload::None)
    //        //            .wait()
    //        //            .unwrap();
    //        println!("json correct");
    //        let resp = upload_from_json(res);
    //        assert_eq!(resp.status(), http::StatusCode::OK);
    //        // fs::remove_file("./img.png").unwrap();
    //        // fs::remove_file("./img_small.png").unwrap();
    //    }

    //    #[test]
    //    // TODO: add body
    //    fn upload_from_multipart_test() {
    //        let req = test::TestRequest::post().to_http_request();
    //        let result = Multipart::from_request(&req, &mut actix_web::dev::Payload::None).unwrap();
    //        let resp = upload_from_multipart(result).wait().unwrap();
    //        assert_eq!(resp.status(), http::StatusCode::OK);
    //        fs::remove_file("./img.png").unwrap();
    //        fs::remove_file("./img_small.png").unwrap();
    //        fs::remove_file("./img2.png").unwrap();
    //        fs::remove_file("./img2_small.png").unwrap();
    //    }

    #[test]
    fn upload_from_url_test() {
        let body = BodyWithUrl {
            // url: "https://itprojects.management/img/leading-team.jpg".to_string(),
            url: "http://passtransit.ru/upload/logo.png".to_string(),
        };
        println!("step 1");
        let form = Form(body);
        println!("step 2");
        let fut = upload_from_url(form);
        println!("step 3");
        let resp = test::block_on(fut).unwrap();
        println!("step 4");
        assert_eq!(resp.status(), http::StatusCode::OK);
        //        fs::remove_file("./img.png").unwrap();
        //        fs::remove_file("./img_small.png").unwrap();
    }
}
