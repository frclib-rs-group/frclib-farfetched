use std::{env, io::{Read, Write}};

use serde_json::{Map, Value};

#[cfg(target_os = "windows")]
const NPM_PATH: &str = "C:\\Program Files\\nodejs\\npm.cmd";
#[cfg(not(target_os = "windows"))]
const NPM_PATH: &str = "npm";

fn main() {
    //cd into ../shiitake-webpage and run `npm run build`
    //compress ../shiitake-webpage/dist/index.html as gzip using flate2
    //put the output intot the output directory

    let output_path = format!("{}/index.html.gz", env::var("OUT_DIR").unwrap());

    //check if NPM_PATH is set, if it is, use that, otherwise use the default
    let npm_path = match env::var("NPM_PATH") {
        Ok(path) => path,
        Err(_) => NPM_PATH.to_string(),
    };

    let build_cmd = std::process::Command::new(npm_path)
        .args(&["run", "build"])
        .current_dir("../shiitake-webpage")
        .status()
        .expect("Failed to run npm build");

    if !build_cmd.success() {
        panic!("Failed to run npm build");
    }

    let mut file = std::fs::File::open("../shiitake-webpage/dist/index.html").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&buffer).unwrap();
    let compressed = encoder.finish().unwrap();

    // println!("cargo:warning={}", output_path);
    std::fs::write(output_path, compressed).unwrap();

    //open the package.json file and get the version
    let mut package_json = std::fs::File::open("../shiitake-webpage/package.json").unwrap();
    let json: Map<String, Value> = serde_json::from_reader(&mut package_json).unwrap();
    let version = json.get("version").unwrap().as_str().unwrap();
    //set the version as an environment variable
    println!("cargo:rustc-env=SHIITAKE_WEBPAGE_VERSION={}", version);

    //tell cargo to watch for changes in every file in ../shiitake-webpage/src
    println!("cargo:rerun-if-changed=../shiitake-webpage/src");
}