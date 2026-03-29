use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("192.168.0.179:21280").unwrap();

    let file = "silent_hill_2.tar";
    let mut data: Vec<u8> = "ps2".as_bytes().to_owned();

    data.push(u8::try_from(file.len()).unwrap());
    for byte in file.as_bytes() {
        data.push(*byte);
    }
    let _ = stream.write_all(&data);

    const KIB64: usize = 1024 * 64;

    let mut file = File::create("/home/mateus/silent_hill_2.tar").unwrap();
    let mut buffer = [0u8; KIB64];

    loop {
        let n = stream.read(&mut buffer).unwrap();

        if n == 0 {
            break;
        }

        file.write_all(&buffer[..n]).unwrap();
    }
}
