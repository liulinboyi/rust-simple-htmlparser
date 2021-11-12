use std::env::current_dir;
use std::fs;
use std::time::SystemTime;

pub mod html2ast;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    // let path = String::from(&args[1]);

    let paths = fs::read_dir("file")?;
    let mut count = 0; // 文件数量
    let mut time_count = 0; // 总时间
                            // let sy_time = SystemTime::now();

    for entry in paths {
        let entry = entry?;
        let path = entry.path();
        // println!("{:?}", path);
        let data = std::fs::read(path)?;
        let file = std::str::from_utf8(&data)?;
        // println!("{:?}", file);
        let mut files: Vec<String> = vec![];
        for item in file.chars() {
            files.push(String::from(item));
        }
        // html2ast::run(&String::from(file))?;

        let start_sy_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        html2ast::run(&files)?;

        let end_sy_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        time_count += end_sy_time - start_sy_time;

        // println!("Hello, world!");
        count += 1;
    }

    println!(
        "File count is {} ,Average parsing time per file is {}ms,All time is {}ms",
        count,
        time_count as f64 / count as f64,
        time_count
    );

    /*
    let path = String::from("html/index3.html");
    let data = std::fs::read(path)?;
    let file = std::str::from_utf8(&data)?;
    // println!("{:?}", file);
    html2ast::run(&String::from(file))?;
    // println!("Hello, world!");
    */
    Ok(())
}
