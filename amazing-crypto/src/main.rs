use std::env;

fn main() {
    match env::current_dir() {
        Ok(path) => println!("当前工作目录是: {}", path.display()),
        Err(e) => eprintln!("无法获取当前工作目录: {}", e),
    }
}