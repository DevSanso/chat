use std::{env, fs, path::Path};
use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    static ref PROJECT_PATH : String = String::from(env!("CARGO_MANIFEST_DIR"));
    static ref PROTOBUF_PATH : String = (|| {
        let mut buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        buf.pop();
        buf.pop();
        buf.pop();
        buf.push("proto");
        let s = String::from(buf.as_os_str().to_str().unwrap());
        return s;
    })();
}

fn main() {
    let out_dir = "src/protos";
    fs::create_dir_all(out_dir).unwrap();

    let proto_root = Path::new(PROTOBUF_PATH.as_str());
    let dbconn_dir = proto_root.join("dbconn");
    let core_dir = proto_root.join("core");

    let mut all_protos = vec![];
    all_protos.extend(collect_proto_files(&dbconn_dir));
    all_protos.extend(collect_proto_files(&core_dir)); 

    prost_build::Config::new()
        .out_dir(out_dir)
        .compile_protos(&all_protos, &[proto_root.to_path_buf()])
        .expect("Failed to compile protos");
}

fn collect_proto_files(dir: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension().unwrap() == "proto" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}
