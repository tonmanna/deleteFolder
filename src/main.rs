use chrono::DateTime;
use chrono::Utc;
use std::fs;
use std::time::SystemTime;
use time;
static mut COUNTER: i32 = 0;

static mut filestruct: Vec<FileStruct> = Vec::new();

struct FileStruct {
    name: String,
    date_time: SystemTime,
}

fn main() {
    let start = time::Instant::now();

    match list_dir("./".to_string()) {
        Err(e) => println!("Error {}", e),
        Ok(_) => {}
    }
    unsafe {
        println!("Total File : {:?}", COUNTER.to_string());
        // filestruct.push(FileStruct {
        //     name: "a".to_string(),
        //     date_time: DateTime.now,
        // });
        // filestruct.sort_by_key(|d| d.date_time); // error[E0507]: cannot move out of borrowed content
    }
    println!("{:?} seconds for whatever you did.", start.elapsed());
}

fn list_dir(param: String) -> std::io::Result<()> {
    let paths = fs::read_dir(param).unwrap();
    for path in paths {
        let path_result = path.unwrap().path();
        let metadata = path_result.metadata();
        if path_result.is_dir() {
            let dir_name = path_result.display();
            match list_dir(dir_name.to_string()) {
                Err(e) => print!("Break Error {}", e),
                Ok(_) => {
                    if let Ok(v) = metadata {
                        println!("{:?}", path_result.display());
                    }
                }
            }
        }
        // } else {
        //     if let Ok(v) = metadata {
        //         match v.created() {
        //             Ok(time) => {
        //                 // println!(
        //                 //     "{:?} {:?}",
        //                 //     system_time_to_date_time(time),
        //                 //     path_result.display()
        //                 // );
        //             }
        //             Err(e) => println!("{:?}", e),
        //         }
        //     } else {
        //         println!("Not supported on this platform or filesystem");
        //     }
        // }
    }
    Ok(())
}

fn system_time_to_date_time(t: SystemTime) -> String {
    let datetime: DateTime<Utc> = t.into();
    return datetime.format("%d/%m/%Y %T").to_string();
}
