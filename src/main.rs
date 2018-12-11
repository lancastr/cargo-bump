extern crate clap;
extern crate semver;
extern crate toml_edit;

mod config;
mod version;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use toml_edit::{value, Document};

use semver::Version;

fn main() {
    let conf = config::get_config();
    let raw_data = read_file(&conf.manifest);
    let mut doc = raw_data
        .parse::<Document>()
        .expect("couldn't parse Cargo.toml");

    let table = doc.as_table_mut();

    let raw_value = table
        .get("package")
        .and_then(|r| r.as_table().and_then(|r| r.get("version")))
        .expect("could not find version");

    let mut version = if raw_value.is_str() {
        Version::parse(raw_value.as_str().unwrap()).expect("bad version format")
    } else {
        panic!("version not a string");
    };

    let old_version = version.clone();
    version::update_version(&mut version, conf.version);
    println!("Version {} -> {}", old_version, version);

    table["package"]["version"] = value(version.to_string());

    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&conf.manifest)
        .unwrap();
    f.write_all(doc.to_string().as_bytes()).unwrap();
}

fn read_file(file: &Path) -> String {
    let mut file = File::open(file).unwrap();
    let mut raw_data = String::new();
    file.read_to_string(&mut raw_data).unwrap();
    raw_data
}
