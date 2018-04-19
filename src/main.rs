#[macro_use]
extern crate structopt;

extern crate walkdir;
extern crate remove_bom;
extern crate regex;

use structopt::StructOpt;
use walkdir::{WalkDir, DirEntry};
use regex::Regex;

#[derive(StructOpt, Debug)]
#[structopt(name = "remove-bom")]
struct Opt {
    #[structopt(short = "e", long = "exts", help = "file extensions seperated by comma(,)")]
    extensions: String
}

fn split_by_comma(s: &String) -> Vec<&str> {
    let _s = s;
    _s.split(",").map(|x| x).collect()
}


fn check_file(re: &Regex, x: &DirEntry) -> bool {
    match x.file_type().is_dir() {
        true => true,
        false => x.file_name().to_str().map(|x| {
            re.is_match(x)
        }).unwrap_or(false)
    }
}

fn build_regex(a: &Vec<&str>) -> Regex {
    let mut restr = a.into_iter().fold(String::new(), |acc, &x| acc + r"\." + x + "$|");
    restr.pop();
    Regex::new(restr.as_str()).unwrap()
}

fn main() {
    let opt = Opt::from_args();
    let extensions = split_by_comma(&opt.extensions);
    let re = build_regex(&extensions);
    let iter = WalkDir::new(".").into_iter();
    for entry in iter.filter_entry(|x| check_file(&re, x) ) {
        let entry = entry.unwrap();
        match entry.file_type().is_file() {

            
            true => {
                remove_bom::remove_bom(entry.path().to_str().unwrap());
                println!("{}", entry.path().display())
            },
            _ => ()
        }
    }

}
