//! This crate provides types and functions useful to use FBX data.

extern crate fbx_binary_reader;
extern crate fnv;
#[macro_use]
extern crate log;

pub use node_loader::FormatConvert;
pub use scene::FbxScene;

use std::io::Read;
use std::path::Path;

pub mod connections;
pub mod definitions;
pub mod error;
pub mod fbx_header_extension;
pub mod objects;
pub mod property;
pub mod scene;

mod node_loader;


/// Load FBX from the given path.
pub fn load_from_file<P: AsRef<Path>, C: FormatConvert>(path: P, converter: C) -> error::Result<FbxScene<C::ImageResult>> {
    use std::fs::File;
    use std::io::BufReader;

    let file = try!(File::open(path));
    load_from_stream(&mut BufReader::new(file), converter)
}

/// Load FBX from the given stream.
pub fn load_from_stream<R: Read, C: FormatConvert>(source: &mut R, converter: C) -> error::Result<FbxScene<C::ImageResult>> {
    use fbx_binary_reader::{FbxEvent, FbxHeaderInfo};

    let reader = &mut fbx_binary_reader::EventReader::new(source);
    let fbx_version = match try!(reader.next()) {
        FbxEvent::StartFbx(FbxHeaderInfo { version }) => version,
        _ => unreachable!(),
    };

    scene::load_scene(reader, fbx_version, converter)
}

/// Returns `Option<(name: &'a str, class: &'a str)>`
fn separate_name_class<'a>(name_class: &'a str) -> Option<(&'a str, &'a str)> {
    name_class.find("\u{0}\u{1}").map(|sep_pos| (&name_class[0..sep_pos], &name_class[sep_pos+2..]))
}
