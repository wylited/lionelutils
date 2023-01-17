use lionel;

#[tokio::main]
async fn main() {
    let username = std::env::args().nth(1).expect("no username provided");
    let password = std::env::args().nth(2).expect("no password provided");

    let client = lionel::Client::default();
    let cookies = client.get_cookies(&username, &password).await;

    println!("{:?}", cookies);
}