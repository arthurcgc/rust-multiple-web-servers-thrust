mod server;

#[tokio::main]
async fn main() {
    let hostname = "0.0.0.0";
    let base_port: u16 = 8000;

    for i in 0..3{
        server::new_server(&hostname, base_port+i as u16)
    }

    futures::future::pending().await
}