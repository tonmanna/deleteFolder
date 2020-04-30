use std::fs;
fn main() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path_result = path.unwrap();
        if path_result.path().is_dir() {
            println!("Directory {}", path_result.path().display())
        } else {
            println!("File {}", path_result.path().display());
        }
    }
}
