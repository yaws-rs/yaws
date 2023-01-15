use lunatic::{net, spawn_link, Mailbox};
use std::io::{BufRead, BufReader, Write};

pub fn run() {
    TcpServer::new("127.0.0.1:9999").spawn();
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TcpServer {
    local_address: String,
}

impl TcpServer {
    fn new(local_address: &str) -> Self {
        TcpServer {
            local_address: local_address.to_owned(),
        }
    }
    fn spawn(self) {
        spawn_link!(|input = self| listen(input));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TcpPeer {
    tcp_stream: lunatic::net::TcpStream,
    peer: std::net::SocketAddr,
}

// Creates a TcpServer Listener that spawns TcpPeers upon accept
fn listen(input: TcpServer) {
    let listener = net::TcpListener::bind(input.local_address.clone()).unwrap();
    println!("Listening on addr: {}", listener.local_addr().unwrap());

    while let Ok((tcp_stream, peer)) = listener.accept() {
        println!("Accepted peer {} on addr: {}", peer, &input.local_address);

        let tcp_peer = TcpPeer { tcp_stream, peer };

        spawn_link!(|input = tcp_peer | respond(input));
    }
}

// Respond back to Line buffered input
fn respond(mut peer: TcpPeer) {
    let mut buf_reader = BufReader::new(peer.tcp_stream.clone());
    loop {
        let mut buffer = String::new();
        let read = buf_reader.read_line(&mut buffer).unwrap();
        if buffer.contains("exit") || read == 0 {
            return;
        }
        peer.tcp_stream.write_all(buffer.as_bytes()).unwrap();
    }
}
