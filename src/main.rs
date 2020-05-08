use chrono::DateTime;
use chrono::Utc;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use time;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FileStruct {
    name: String,
    created_date: SystemTime,
}

fn main() {
    let start = time::Instant::now();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let param = args.remove(1);
        println!("Param : {:?}", param);
        if path_exists(&param) {
            match list_dir(&param) {
                Err(e) => println!("Error {}", e),
                Ok(_) => {}
            }
        } else {
            println!("Path not found!!");
        }
        println!("{:?} seconds for whatever you did.", start.elapsed());
    } else {
        println!("The remove_image_file command used for keep last 5 file  ");
        println!("Example : ./remove_image_file <path>");
    }
}

fn list_dir(param: &String) -> std::io::Result<()> {
    let paths = fs::read_dir(param).unwrap();
    for path in paths {
        let path_result = path.unwrap().path();
        let metadata = path_result.metadata();
        if path_result.is_dir() {
            let dir_name = path_result.display();
            let count_file = get_file_inside(dir_name.to_string());
            match list_dir(&dir_name.to_string()) {
                Err(e) => print!("Break Error {}", e),
                Ok(_) => {
                    if let Ok(_) = metadata {
                        let total_file = count_file.0;
                        //println!("{:?}, TotalFile: {:?}", path_result.display(), total_file);
                        let mut file_result: Vec<FileStruct> = count_file.1;
                        file_result.sort_by_key(|f| f.created_date);
                        let max_file: usize = 5;
                        if total_file > max_file as i32 {
                            slice_files(file_result, max_file);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn slice_files(files: Vec<FileStruct>, number: usize) {
    let total_file = files.len();
    println!("Length: {:?}", total_file);
    let files_local = &files[0..files.len()-number];
    // let files_local = &files[1..files.len()];
    for file in files_local {
        println!(
            "{:?} Date: {:?}",
            file.name,
            system_time_to_date_time(file.created_date)
        );
        if let Ok(_) = fs::remove_file(file.name.to_string()) {
            println!(" was delete.");
        }
    }
    println!("--------------------------------------------------------------");
}

fn system_time_to_date_time(t: SystemTime) -> String {
    let datetime: DateTime<Utc> = t.into();
    return datetime.format("%d/%m/%Y %T").to_string();
}

fn get_file_inside(s: String) -> (i32, Vec<FileStruct>) {
    let paths = fs::read_dir(s).unwrap();
    let mut file_list: Vec<FileStruct> = Vec::new();
    let mut i: i32 = 0;
    for path in paths {
        let path_result = path.unwrap().path();
        match path_result.is_file() {
            true => {
                let metadata = path_result.metadata().unwrap();
                let dateinfo = metadata.created();
                if let Ok(v) = dateinfo {
                    let file_result = FileStruct {
                        name: path_result.display().to_string(),
                        created_date: v,
                    };
                    file_list.push(file_result);
                }
                i = i + 1;
            }
            false => {}
        }
    }
    return (i, file_list);
}

pub fn path_exists(path: &String) -> bool {
    Path::new(path).exists()
}
