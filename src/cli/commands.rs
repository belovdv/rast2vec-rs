use std::{collections::HashMap, path::Path};

use crate::rast2vec::ImageRGBA;

use super::Config;

use rusteval::{InteractiveRoot, Methods};

#[derive(InteractiveRoot)]
pub struct Manager {
    config: Config,

    data: HashMap<String, ImageRGBA>,

    status: Status,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Normal,

    Exit,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            data: Default::default(),
            status: Status::Normal,
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    fn img_load(&mut self, path: &Path, id: String) -> String {
        let img = match image::open(&path) {
            Ok(img) => img.into_rgba8(),
            Err(e) => return format!("cannot load image: {e}"),
        };
        let nd = crate::utility::image2ndarray(&img, 1);
        self.data.insert(id.clone(), nd);

        format!("image `{:?}` loaded to `{id}`", path)
    }
    fn img_store(&self, id: &str, path: &Path) -> String {
        let nd = match self.data.get(id) {
            Some(nd) => nd,
            None => return format!("cannot find image `{id}`"),
        };
        let img = crate::utility::ndarray2image(&nd);

        match img.save(path) {
            Ok(_) => format!("image `{id}` stored in `{:?}`", path),
            Err(e) => format!("cannot store image: {e}"),
        }
    }
}

#[Methods]
impl Manager {
    fn src_load_img(&mut self, filename: &str, id: String) -> String {
        let path = match &self.config.dir_source {
            Some(s) => s.join(filename),
            None => return format!("cannot open file {filename}"),
        };
        self.img_load(&path, id)
    }

    fn ws_load_img(&mut self, filename: &str, id: String) -> String {
        self.img_load(&self.config.workspace.join(filename), id)
    }
    fn ws_store_img(&self, id: &str, filename: &str) -> String {
        self.img_store(id, &self.config.workspace.join(filename))
    }

    fn exit(&mut self) {
        self.status = Status::Exit;
    }
    fn quit(&mut self) {
        self.status = Status::Exit;
    }
    fn q(&mut self) {
        self.status = Status::Exit;
    }
}

/* Example load-store

let source = config.dir_source.unwrap().join(name);
let img = image::open(source).unwrap().into_rgba8();
let nd = rast2vec::image2ndarray(&img);
let ser = serde_json::to_string(&nd).expect("couldn't serialize");
std::fs::File::create(config.workspace.join(format!("{name}.ser.json")))
                .unwrap().write(ser.as_bytes()).unwrap();

let out = config.workspace.join(name);
let ser = std::fs::read_to_string(config.workspace.join(format!("{name}.ser.json"))).unwrap();
let nd: Array<[u8; 4], Dim<[usize; 2]>> = serde_json::from_str(&ser).unwrap();
let img = rast2vec::ndarray2image(&nd);
img.save(out).unwrap();
 */

// #[derive(Debug, Clone, Copy, strum_macros::EnumString)]
// pub enum Commands {
//     #[strum(ascii_case_insensitive)]
//     Load,
//     #[strum(ascii_case_insensitive)]
//     Store,
// }
