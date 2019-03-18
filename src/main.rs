use std::io;

fn walk(path: &str) -> io::Result<()> {
    for entry in std::fs::read_dir(path)? {
        let file = entry?;
        println!("{:?}", file.path());
    }
    Ok(())
}

fn main() {
    println!("Hello, world! Let's see what we have...");
    walk(".").expect("Failed to walk directory");
}
