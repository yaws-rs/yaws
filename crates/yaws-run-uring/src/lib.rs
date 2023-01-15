use tokio_uring::net::TcpListener;

pub fn run() {
    tokio_uring::start(async move {
        let listener = crate::Listener::new();
        listener.next().await;
    });
}

struct Listener {
    listener: TcpListener,
}

impl Listener {
    fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:9999".parse().unwrap()).unwrap();

        Self { listener }
    }
    async fn next(self: &Self) {
        let (client, addr) = self.listener.accept().await.unwrap();

        let mut reader = Reader::new(client);
        reader = reader.read_next().await;

        //reader.status();
    }
}

use tokio_uring::buf::IoBufMut;
use tokio_uring::net::TcpStream;

struct Reader {
    buf_in: Vec<u8>,
    client: TcpStream,
}

impl Reader {
    fn new(client: TcpStream) -> Self {
        let buf_in = vec![0; 8192];
        Self { buf_in, client }
    }
    async fn read_next(mut self) -> Self {
        let (res, buf) = self.client.read(self.buf_in).await;
        let n = res.unwrap();
        self.buf_in = buf;

        println!("n: {:?}", n);

        self
    }
}
