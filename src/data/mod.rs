pub mod file;
pub mod mem;

pub fn test(file_name: &str) {
    mem::read_mem();

    if let Err(err) = file::read_file(file_name) {
        println!("failed to read data file {:?}", err);
    }
}