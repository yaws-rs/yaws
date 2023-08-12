use lunatic::{net, spawn_link, Mailbox};

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct WebRTCServer {
    local_address: String,
}

impl WebRTCServer {
    pub(crate) fn new(local_address: &str) -> Self {
        WebRTCServer {
            local_address: local_address.to_owned(),
        }
    }
    pub(crate) fn spawn(self) {
        spawn_link!(|me = self| bind(me));
    }
}

// Binds the WebRTCServer
fn bind(input: WebRTCServer) {
    let bound_srv = net::UdpSocket::bind(input.local_address.clone()).unwrap();
    println!("Bound on addr: {}", bound_srv.local_addr().unwrap());

    let mut in_buf = [0; 9100];

    while let Ok((bytec, peer)) = bound_srv.recv_from(&mut in_buf) {
        println!(
            "Received [{}] from peer {} on addr: {}",
            bytec, peer, &input.local_address
        );

        let recvd_buf = &mut in_buf[..bytec];

        let dbg_buf = String::from_utf8_lossy(recvd_buf);

        dbg!(dbg_buf);
    }
}
