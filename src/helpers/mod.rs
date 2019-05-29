use std::env;
use std::fs;
use std::path::Path;
use std::process;

pub fn get_output_name(file_path: &str) -> String {
    let path = Path::new(file_path);

    let os_str = match path.file_name() {
        None => panic!("cannot find a filename to output from {}", file_path),
        Some(s) => s,
    };

    let mut file_name = match os_str.to_str() {
        None => panic!("cannot convert file path to string from {}", file_path),
        Some(s) => String::from(s),
    };

    file_name.push_str(".png");
    file_name
}

pub fn get_args() -> (String, String) {
    let arg_vec: Vec<String> = env::args().collect();

    if arg_vec.len() < 2 {
        println!("Must provide a file to read");
        process::exit(1);
    }

    let file_path = arg_vec[1].clone();
    let output_name = get_output_name(&file_path[..]);

    (file_path, output_name)
}

pub fn get_file_as_binary(file_path: &str) -> Vec<u8> {
    let contents = match fs::read(file_path) {
        Ok(d) => d,
        Err(e) => panic!("Failed to read file: {} \n Error: \n {}", file_path, e),
    };

    contents
}

pub fn get_dimensions_from_len(len: f64) -> (u32, u32) {
    let sqrt = len.sqrt();
    let mut upper_limit = sqrt as u32;

    if upper_limit * upper_limit < len as u32 {
        upper_limit += 1;
    }

    (upper_limit, upper_limit)
}
