use std::env;
use std::fs::File;
use std::path::*;
use std::io::Write;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("webgl_exts.rs")).unwrap();

    let root = env::current_dir().unwrap().join("api_webgl/extensions");

    writeln!(file, "&[").unwrap();
    for entry in root.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let ext_name = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() && ext_name != "template" {
            let ext_path = path.join("extension.xml");
            if ext_path.is_file() {
                writeln!(file, "&*include_bytes!({:?}),", ext_path.to_str().unwrap()).unwrap();
            }
        }
    }
    writeln!(file, "]").unwrap();
}
