use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{
    fs::read_dir,
    io::{self, Read},
    net::TcpStream,
    string::FromUtf8Error,
    time::Duration,
};

const PATH: &str = "/home/mateus/Games/";

pub mod read {
    use super::*;

    pub fn read_exact(mut stream: &TcpStream, limit: usize) -> io::Result<Vec<u8>> {
        stream.set_read_timeout(Some(Duration::from_secs_f64(2.5)))?;

        let mut bytes: Vec<u8> = vec![0; limit];
        stream.read_exact(&mut bytes)?;

        Ok(bytes)
    }
}

pub mod write {
    const KIB64: usize = 1024 * 64;
    use std::io::Write;

    use super::*;

    pub fn write_file(mut stream: TcpStream, Path { folder, file }: Path) -> io::Result<()> {
        let path = format!("{PATH}{}/compressed/{}", folder, file);
        let mut count: f64 = 0.0;

        println!("{path}");
        let mut file = File::open(path)?;
        let mut buffer = [0u8; write::KIB64];

        loop {
            let n = file.read(&mut buffer)?;
            count += n as f64 / 1_000_000.0;
            println!("{count} MiB");

            if n == 0 {
                break;
            }

            stream.write_all(&buffer[..n])?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Path {
    pub folder: String,
    pub file: String,
}

impl Path {
    pub fn try_new(folder: Vec<u8>, file: Vec<u8>) -> Result<Path, FromUtf8Error> {
        let folder = String::from_utf8(folder)?;
        let file = String::from_utf8(file)?;

        Ok(Path::new(folder, file))
    }

    fn new(folder: String, file: String) -> Path {
        Path { folder, file }
    }
}

#[derive(Debug)]
pub struct Games {
    games: HashMap<String, HashSet<String>>,
}

impl Games {
    pub fn try_new() -> io::Result<Games> {
        let dirs = read_dir(PATH)
            .expect("Should be a accessible dir")
            .flatten()
            .filter(|d| d.path().is_dir());

        let mut games = HashMap::new();

        for dir in dirs {
            let Ok(dir) = dir.file_name().into_string() else {
                continue;
            };

            let Some(list) = Games::dir_games(&dir) else {
                continue;
            };

            games.insert(dir, list);
        }

        Ok(Games { games })
    }

    fn dir_games(dir: &str) -> Option<HashSet<String>> {
        let mut setlist = HashSet::new();

        let dir = format!("{PATH}{dir}/compressed/");

        let Ok(list) = read_dir(&dir) else {
            eprintln!("Error reading {dir}");
            return None;
        };

        for game in list.flatten() {
            let Ok(game_name) = game.file_name().into_string() else {
                continue;
            };

            if game_name.ends_with(".tar") {
                setlist.insert(game_name);
            }
        }

        if setlist.is_empty() {
            None
        } else {
            Some(setlist)
        }
    }

    pub fn search(&self, path: &Path) -> bool {
        let Some((_, list)) = self.games.get_key_value(&path.folder) else {
            return false;
        };

        list.contains(&path.file)
    }
}
