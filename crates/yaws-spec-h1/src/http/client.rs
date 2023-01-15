use tokio_uring::net::TcpStream;
use tokio_uring::buf::IoBufMut;

pub struct Reader {
    buf_in: Vec<u8>,
    client: TcpStream,
}

impl Reader {
    pub fn new(client: TcpStream) -> Self {
        let buf_in = vec![0; 8192];
        Self { buf_in, client }
    }
    pub async fn read_next(mut self) -> Self {
        let (res, buf) = self.client.read(self.buf_in).await;
        let n = res.unwrap();
        self.buf_in = buf;
        
        println!("n: {:?}", n);

        self
    }
    /*
    pub fn status(&self) {

        // I look dodgy..
        crate::http::parser::status(&self.buf_in)
    }
     */
}
