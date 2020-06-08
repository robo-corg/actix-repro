use actix_web::{self, web, App, Responder, HttpServer};
use futures::{future, StreamExt};
use std::env;

async fn test_view(body: web::Payload) -> impl Responder {
    // Consuming the stream here seems necessary for things to bloat up
    body.for_each(|_chunk|  future::ready(())).await;
    "test"
}

#[actix_rt::main]
async fn main() {
    let server = HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/test").to(test_view)
            )
    });

    let http_server = server.bind("127.0.0.1:8000").unwrap();
    let server = http_server.run();

    {
        use tokio::process::Command;

        let uploader_path = env::var("UPLOADER_PATH").ok().unwrap_or("target/debug/uploader".to_owned());

        let child = Command::new(&uploader_path)
            .spawn();

        let future = child.expect("failed to spawn");

        let status = future.await.expect("failed to run");
        println!("the command exited with: {}", status);
    }

    server.stop(true).await;
}
