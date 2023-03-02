use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Result};
use std::path::PathBuf;

async fn index(_: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from("static/file"))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use std::io::Read;

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        assert!(resp.is_ok());
    }

    #[actix_web::test]
    async fn test_index_returns_file_contents() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index(req).await;
        let mut contents = String::new();
        // TODO: we can't assume this resp is Ok -- is there an idiomatic way
        //       to do this in tests in Rust?
        resp.unwrap().file().read_to_string(&mut contents);
        assert_eq!(contents, "Hello, Static File!\n")
    }
}
