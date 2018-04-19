use std::fs::File;
use std::fs;
use std::io::Read;
use std::io::{Seek,SeekFrom};
use std::io;

fn check_bom(file_name :&str) -> bool {
    let mut f = File::open(file_name).unwrap();
    let mut buf = vec![0;3];
    f.read(&mut buf).unwrap();
    if (buf[0] == 0xef) && (buf[1] == 0xbb) && (buf[2] == 0xbf) {
        return true;
    }
    false
}

fn remove_bom_impl(file_name :&str) -> bool {
    let tmpname = file_name.to_owned() + ".tmp";
    {
        fs::rename(file_name, tmpname.as_str());

        let mut old_file = File::open(tmpname.as_str()).unwrap();
        let mut file = File::create(file_name).unwrap();
        old_file.seek(SeekFrom::Start(3));
        io::copy(&mut old_file, &mut file);
    }
    fs::remove_file(tmpname.as_str());
    true
}

pub fn remove_bom(file_name: &str) ->bool {
    match check_bom(file_name) {
        true => remove_bom_impl(file_name),
        false => true
    }
}