extern crate embed_resource;
extern crate rustfmt;
extern crate peg;

use std::env;
use std::fs::{self, File};
use std::path::{PathBuf, Path};
use std::collections::BTreeMap;
use std::io::{BufReader, BufRead, Write};


fn main() {
    let doc_correction_map = {
        let mut m = BTreeMap::new();
        m.insert("pub struct ParseError {", "OpenAlias parsing error");
        m.insert("    pub line: usize,", "1-based line # of error");
        m.insert("    pub column: usize,", "1-based column # of error");
        m.insert("    pub offset: usize,", "Byte offset of error");
        m.insert("    pub expected: ::std::collections::HashSet<&'static str>,",
                 "Set of expected but unmatched rules");
        m
    };


    embed_resource::compile("openalias.rs-manifest.rc");

    let grammar_rs = PathBuf::from(format!("{}/grammar.rs", env::var("OUT_DIR").expect("OUT_DIR env var nonexistant/non-Unicode")));
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
