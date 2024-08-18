#[macro_export]
macro_rules! read_line {
    () => {{
        use std::io::stdin;

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        input.trim().to_string()
    }};

    ($val:expr) => {{
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => *$val = input.trim().to_owned(),
            Err(_) => {}
        }
        $val
    }};
}