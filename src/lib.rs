pub mod day1;

#[macro_export]
macro_rules! day_file {
    ($name:expr) => {{
        use std::fs::read_to_string;
        use std::path::Path;

        let p = Path::new(file!())
            .parent()
            .and_then(|p| Some(p.join($name)))
            .unwrap();
        read_to_string(p).unwrap()
    }};
}
