pub mod html2ast;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = String::from(&args[1]);
    // let path = String::from("html/index2.html");
    let data = std::fs::read(path)?;
    let file = std::str::from_utf8(&data)?;
    // println!("{:?}", file);
    html2ast::run(&String::from(file))?;
    // println!("Hello, world!");
    Ok(())
}
