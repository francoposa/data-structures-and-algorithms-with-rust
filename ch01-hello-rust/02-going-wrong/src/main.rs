use std::{fs, io};

fn main() {}

fn find(needle: u16, haystack: &Vec<u16>) -> Option<usize> {
    haystack.iter().position(|&x| x == needle)
}

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let vec = vec![0, 1, 2, 3];
        assert!(find(2, &vec).is_some());
        assert!(find(4, &vec).is_none());
    }

    #[test]
    fn test_read_file() {
        assert!(read_file("/tmp/not/a/file").is_err())
    }
}
