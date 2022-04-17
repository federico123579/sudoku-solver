use std::path::{Path, PathBuf};

fn get_board_dir() -> std::io::Result<PathBuf> {
    // take the path pointing to this file, go up two level and then go to the board directory
    Path::new(file!())
        .ancestors()
        .nth(2)
        .unwrap()
        .join("boards")
        .canonicalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_board_dir() {
        let board_dir = get_board_dir().unwrap();
        assert!(board_dir.is_dir());
        assert!(board_dir.join("empty_board.txt").exists());
    }
}
