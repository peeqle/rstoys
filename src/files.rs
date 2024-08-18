#[macro_export]
macro_rules! read_file_buffer {
    ($path:expr) => {{
        use io::File;
        let file = File::open($path)?;
        let mut buffer = Vec::new();
        match file {
            Ok() => {
                f.read_to_end(&mut buffer)?;
            }
            Err(_) => {
                panic!("cannot read file on: {}", $path);
            }
        }

        buffer;
    }};
}

#[macro_export]
macro_rules! read_file_content {
    ($path:expr) => {{
        use std::fs::File;
        use std::io::prelude::*;

        let file = File::open($path);
        let mut content = String::new();
        if file.is_ok() {
            file.expect("File reading error {}")
                .read_to_string(&mut content)
                .unwrap();
        } else {
            panic!("Cannot read file: {}", $path.display());
        }
        content
    }};
}
