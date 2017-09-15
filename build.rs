extern crate embed_resource;
extern crate rustfmt;
extern crate peg;

use std::env;
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::collections::BTreeMap;
use std::io::{BufReader, BufRead, Write, Read};


fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR env var nonexistant/non-Unicode");
    let doc_correction_map = {
        let mut m = BTreeMap::new();
        m.insert("pub struct ParseError {", "OpenAlias record parsing error");
        m.insert("    pub line: usize,", "1-based line # of error");
        m.insert("    pub column: usize,", "1-based column # of error");
        m.insert("    pub offset: usize,", "Byte offset of error");
        m.insert("    pub expected: ::std::collections::HashSet<&'static str>,",
                 "Set of expected but unmatched rules");
        m
    };


    let mut resource_template = String::new();
    File::open("openalias.rs-manifest.rc").expect("opening resource file").read_to_string(&mut resource_template).expect("reading resource file");

    write!(File::create(format!("{}/openalias.rs-manifest.rc", out_dir)).expect("creating resource file"),
           "#define FILE_VERSION_COMMAS {}\n\
            #define FILE_VERSION_STRING \"{}\"\n\
            \n\
            {}",
           env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION env var nonexistant/non-Unicode").replace('.', ",") + ",0",
           env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION env var nonexistant/non-Unicode"),
           resource_template)
        .expect("writing resource file");
    embed_resource::compile(format!("{}/openalias.rs-manifest.rc", out_dir));


    let grammar_rs = PathBuf::from(format!("{}/grammar.rs", out_dir));
    peg::cargo_build("src/grammar.rustpeg");
    let _ = rustfmt::run(rustfmt::Input::File(grammar_rs.clone()),
                         &rustfmt::config::Config::from_toml_path(Path::new("rustfmt.toml")).expect("constructing rustfmt::Config from rustfmt.toml"));

    fs::copy(&grammar_rs, grammar_rs.with_extension("rs.nodoc")).expect("Backing up formatted grammar.rs");
    let mut out_f = File::create(&grammar_rs).expect("Creating documented grammar.rs");
    for line in BufReader::new(File::open(grammar_rs.with_extension("rs.nodoc")).expect("Opening formatted grammar.rs")).lines() {
        let line = line.expect("Reading line of formatted grammar.rs");
        if let Some(doc) = doc_correction_map.get(&line[..]) {
            writeln!(out_f, "#[doc = \"{}\"] ", doc).expect("Adding documentation to documented grammar.rs");
        }
        out_f.write_all(line.as_bytes()).expect("Copying line to documented grammar.rs");
        out_f.write_all(b"\n").expect("Writing newline to documented grammar.rs");
    }
}
