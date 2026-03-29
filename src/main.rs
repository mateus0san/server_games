use std::net::{TcpListener, TcpStream};

use server::Path;
use server::write::write_file;
use server::{Games, read};

const ADDR: &str = "0.0.0.0:21280";

fn main() {
    let game_list = Games::try_new().unwrap();

    start_server(ADDR, game_list);
}

fn start_server(addr: &str, game_list: Games) {
    let listener = TcpListener::bind(addr).expect("Could not bind the server");
    eprintln!("Server listening: {addr}");

    for stream in listener.incoming().flatten() {
        let Some(path) = handle_client(&stream) else {
            continue;
        };

        if game_list.search(&path) {
            eprintln!("Client {addr:#?} requested: {}/{}", path.folder, path.file);
            eprintln!("{:#?}", write_file(stream, path));
        }
    }
}

fn handle_client(stream: &TcpStream) -> Option<Path> {
    let Ok(folder) = read::read_exact(stream, 3) else {
        eprintln!("Error reading folder");
        return None;
    };

    let Ok(file_name_len) = read::read_exact(stream, 1) else {
        eprintln!("Error reading filename len");
        return None;
    };

    if file_name_len[0] > 42 {
        eprintln!("Filename too big");
        return None;
    }

    let Ok(file_name) = read::read_exact(stream, file_name_len[0] as usize) else {
        eprintln!("Error reading filename");
        return None;
    };

    let Ok(path) = Path::try_new(folder, file_name) else {
        eprintln!("Invalid request");
        return None;
    };

    Some(path)
}
