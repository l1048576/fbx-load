//! This crate provides types and functions useful to use FBX data.

#[macro_use]
extern crate log;
extern crate fbx_binary_reader;

pub use scene::FbxScene;

use std::io::Read;
use std::path::Path;

pub mod definitions;
pub mod error;
pub mod fbx_header_extension;
pub mod property;
pub mod scene;

mod node_loader;


/// Load FBX from the given path.
pub fn load_from_file<P: AsRef<Path>>(path: P) -> error::Result<FbxScene> {
    use std::fs::File;
    use std::io::BufReader;

    let file = try!(File::open(path));
    load_from_stream(&mut BufReader::new(file))
}

/// Load FBX from the given stream.
pub fn load_from_stream<R: Read>(source: &mut R) -> error::Result<FbxScene> {
    use fbx_binary_reader::{FbxEvent, FbxHeaderInfo};

    let reader = &mut fbx_binary_reader::EventReader::new(source);
    let fbx_version = match try!(reader.next()) {
        FbxEvent::StartFbx(FbxHeaderInfo { version }) => version,
        _ => unreachable!(),
    };

    scene::load_scene(reader, fbx_version)
}
