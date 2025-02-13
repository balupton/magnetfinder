use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

pub struct Torrent {
    pub title: String,
    pub magnet: String,
    pub size: String,
    pub seeders: String,
}

impl Torrent {
    pub fn get_size_as_i64(&self) -> i64 {
        let split: Vec<&str> = self.size.split(' ').collect();

        if split.len() < 2 {
            let float: f32 = split[0].parse().unwrap_or(0.0);
            let int_approximation = (float * 100.0) as i64;
            return int_approximation;
        }

        let base: i64 = 2;
        let byte_modifier = match split[1] {
            "TiB" | "TB" => base.pow(40),
            "GiB" | "GB" => base.pow(30),
            "MiB" | "MB" => base.pow(20),
            "KiB" | "KB" => base.pow(10),
            "Bytes" => 1,
            _ => 1,
        };

        let float: f32 = split[0].parse().unwrap_or(0.0);
        let int_approximation = (float * 100.0) as i64;

        int_approximation * byte_modifier
    }
}

pub enum Website {
    Nyaa,
    Piratebay,
    YTS,
}

pub enum Media {
    Anime,
    Movie,
    TVShow,
}

pub enum Sort {
    Size,
    Seeds,
}

pub enum TorrentClient {
    Deluge,
    Transmission,
    Unknown,
}

pub struct Settings {
    pub anime_dir: Rc<PathBuf>,
    pub tvshow_dir: Rc<PathBuf>,
    pub movie_dir: Rc<PathBuf>,
    pub default_directory: Rc<PathBuf>,
    pub default_proxy: String,
    pub autodownload: bool,
    pub torrent_client: String,
}

pub struct UserParameters {
    pub websites: Vec<Website>,
    pub directory: Rc<PathBuf>,
    pub search_query: Arc<String>,
    pub search_depth: u32,
    pub sort_preference: Sort,
    pub num_torrents_shown: usize,
    pub proxy: Arc<String>,
    pub autodownload: bool,
    pub torrent_client: TorrentClient,
    pub no_interactive: bool,
}
