use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;

    let git_path: &str = "/.git";
    let mut pwd: String =  path.display().to_string().to_owned();

    pwd.push_str(git_path);
    println!("The current directory is {}", pwd);


    let contents = fs::read_to_string(git_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    Ok(())
}