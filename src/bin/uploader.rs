const PAYLOAD_SIZE: usize = 1024 * 1024 * 512;
const REQUEST_COUNT: i32 = 100;

#[actix_rt::main]
async fn main() {
    let data: Vec<u8> = vec![0; PAYLOAD_SIZE];

    let url = "http://127.0.0.1:8000/test";

    for n in 0..REQUEST_COUNT {
        let client = reqwest::Client::new();

        println!("starting upload {}/{}", n, REQUEST_COUNT);

        let data_len = data.len();

        client
            .post(url)
            .header("Content-type", "application/octet-stream")
            .header("Content-Length", data_len)
            .body(data.clone())
            .send()
            .await
            .expect("Failed to send data")
            .error_for_status()
            .expect("Failed to send data");
    }
}
