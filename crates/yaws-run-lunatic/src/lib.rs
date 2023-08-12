#[cfg(yaws_tcp = "1")]
mod tcp;
    
#[cfg(yaws_webrtc = "1")]
mod webrtc;

pub fn run() {
    #[cfg(yaws_webrtc = "1")]
    webrtc::WebRTCServer::new("192.168.88.252:9998").spawn();
    #[cfg(yaws_webrtc = "1")]
    webrtc::WebRTCServer::new("127.0.0.1:9998").spawn();
    
    #[cfg(yaws_tcp = "1")]
    tcp::TcpServer::new("127.0.0.1:9999").spawn();

}
