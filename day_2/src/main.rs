
fn get_file_path(file: &str) -> std::path::PathBuf {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    exe_path.parent().unwrap().join(file)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

    }
}