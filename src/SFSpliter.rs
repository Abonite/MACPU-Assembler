use std::{fs::File, io::Read};

pub fn SourceFileSpliter(assembly_file_path: &str) -> Vec<(usize, String)> {
    let mut sf = File::open(assembly_file_path).unwrap();
    let mut sf_data = String::new();
    sf.read_to_string(&mut sf_data).unwrap();

    sf_data.trim().lines().enumerate().map(|x| (x.0, String::from(x.1.trim()))).collect::<Vec<_>>()
    // TODO: remove comments
}