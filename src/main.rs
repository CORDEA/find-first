extern crate walkdir;
extern crate getopts;

use walkdir::WalkDir;
use getopts::Options;
use std::ffi::OsStr;
use std::fs::FileType;
use std::path::Path;
use std::process;
use std::env;

enum Type {
    All,
    File,
    Dir,
}

impl Type {
    pub fn from_str(sr: &str) -> Type {
        match sr {
            "f" => Type::File,
            "d" => Type::Dir,
            _ => Type::All
        }
    }
}

fn find<'a>(dir: &str, path: &'a Path) -> Option<&'a str> {
    if let Some(name) = path.file_name() {
        if name == OsStr::new(dir) {
            if let Some(path) = path.to_str() {
                return Some(path)
            }
        }
    }
    return None
}

fn is_valid(ft: &FileType, ty: &Type) -> bool {
    match ty {
        &Type::All => true,
        &Type::File => ft.is_file(),
        &Type::Dir => ft.is_dir()
    }
}

fn walk(base: &str, name: &str, ty: &Type) {
    let mut is_find = false;
    for f in WalkDir::new(base) {
        let child = f.unwrap();
        if is_valid(&child.file_type(), ty) {
            if let Some(path) = find(name, child.path()) {
                println!("{}", path);
                is_find = true;
                break;
            }
        }
    }
    if !is_find {
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("t", "type", "", "TYPE");
    opts.optopt("n", "name", "", "NAME");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f)
    };

    if !matches.free.is_empty() {
        let base = matches.free[0].clone();
        if let Some(name) = matches.opt_str("name") {
            let mut ty = Type::All;
            if let Some(t) = matches.opt_str("type") {
                ty = Type::from_str(&t);
            }
            walk(&base, &name, &ty);
        } else {
            panic!();
        }
    }
}
