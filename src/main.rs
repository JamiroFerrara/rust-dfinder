use clap::Parser;
use std::fs;
use std::fs::metadata;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Number of times to greet
   #[clap(short, long, value_parser)]
    s: String
}

fn main() {
    let args = Args::parse();

    let dirs = get_directories("./");
    let w_dirs = walk_directories(dirs);
    filter_directories(args.s, w_dirs)
}

fn filter_directories(search_term: String, dirs: Vec<Vec<String>>) {
    for dir in dirs {
        for file in dir {
            if file.contains(&search_term) {
                println!("{}", file);
            }
        }
    }
}

fn check_directory(path: &str) -> bool {
    let metadata = metadata(path).unwrap();
    return metadata.is_dir();
}

fn walk_directories(dirs: Vec<String>) -> Vec<Vec<String>>{
    let mut result = Vec::new();
    for dir in &dirs {
        let is_dir = check_directory(dir);
        if is_dir {
            let sub_dirs = get_directories(&dir);
            result.push(sub_dirs);
        }
    }

    return result;
}

fn get_directories(path: &str) -> Vec<String> {
    let mut directories = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        directories.push(path.unwrap().path().to_str().unwrap().to_string());
    }

    return directories;
}
