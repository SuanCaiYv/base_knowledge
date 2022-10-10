fn main() {
    let socket = std::net::UdpSocket::bind("127.0.0.1:11120").unwrap();
    let stream = std::net::UdpSocket::bind("127.0.0.1:8190").unwrap();
    std::thread::spawn(move || {
        stream.connect("127.0.0.1:11120").unwrap();
        stream.send("hello".as_bytes()).unwrap();
        println!("{}", stream.local_addr().unwrap().to_string())
    });
    std::thread::spawn(move || {
        let buf = &mut [0; 1024];
        socket.recv(buf).unwrap();
        println!("received: {}", String::from_utf8_lossy(buf));
    });
    std::thread::sleep(std::time::Duration::from_secs(1));
}
