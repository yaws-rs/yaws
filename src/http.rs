use tokio_uring::net::TcpListener;

pub mod client;
pub mod parser;

use self::client::Reader as HttpReader;

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:9999".parse().unwrap()).unwrap();
        
        Self { listener }
    }
    pub async fn next(self: &Self) {
        let (client, addr) = self.listener.accept().await.unwrap();

        let mut reader = HttpReader::new(client);
        reader = reader.read_next().await;

        reader.status();
    }
}
