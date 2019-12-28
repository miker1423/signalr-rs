use signalr_rs;

fn main() {
    let url = "http://localhost:5000/chat".to_owned();
    let connection = signalr_rs::HubConnectionBuilder::new().with_url(url).build();
    println!("{:?}", connection);
}
