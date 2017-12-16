use std::io::{BufReader, Read, Result, Seek, SeekFrom};
use std::fs::File;
use std::slice;
use std::time::Instant;

pub fn read_file(file_name: &str) -> Result<()> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            println!("failed to open file '{}' with {:?}", file_name, err);
            return Err(err);
        }
    };

    let mut numbers = vec![0.0f64; 100 * 1024];
    let mut buffer = unsafe {
        slice::from_raw_parts_mut(numbers.as_mut_ptr() as *mut u8, numbers.len() * 8)
    };


    file.seek(SeekFrom::Start(0))?;
    let now = Instant::now();

    BufReader::with_capacity(100 * 1024, file).read_exact(&mut buffer)?;

    let elapsed = now.elapsed();

    println!("read in {:?}: {:?}", elapsed, &numbers[10 * 1024..10 * 1024 + 20]);

    Ok(())
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_file_read(b: &mut Bencher) {
        let file_name = "/Users/Shared/data/dsj/test.data";

        let mut file = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => {
                assert!(false, "failed to open file '{}' with {:?}", file_name, err);
                return;
            }
        };

        let mut numbers = vec![0.0f64; 100 * 1024];
        let mut buffer = unsafe {
            slice::from_raw_parts_mut(numbers.as_mut_ptr() as *mut u8, numbers.len() * 8)
        };
        let mut reader = BufReader::new(&mut file);

        b.iter(move || {
            reader.seek(SeekFrom::Start(0));
            reader.read_exact(&mut buffer)
        });
    }
}
