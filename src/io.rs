use std::io::BufRead;

pub struct Pieces<T>
    where T: BufRead + Sized
{
    reader: T,
    buf: Vec<u8>,
    delimiter: u8,
}

impl<T: 'static> Pieces<T>
    where T: BufRead + Sized
{
    pub fn new(reader: T, delimiter: u8) -> Pieces<T> {
        Pieces {
            reader,
            delimiter,
            buf: Vec::new(),
        }
    }

    fn read_line(&mut self) -> Option<(usize, String)> {
        let mut buf = &mut self.buf;

        buf.clear();

        match self.reader.read_until(self.delimiter, &mut buf) {
            Ok(0) => None,
            Ok(delta) => {
                if buf[buf.len() - 1] == self.delimiter {
                    buf.pop();
                }

                let line = String::from_utf8_lossy(&mut buf);

                Some((delta, line.to_string()))
            }
            Err(_) => None
        }
    }
}


impl<T: 'static> Iterator for Pieces<T>
    where T: BufRead + Sized
{
    type Item = (usize, String);

    fn next(&mut self) -> Option<(usize, String)> {
        self.read_line()
    }
}
