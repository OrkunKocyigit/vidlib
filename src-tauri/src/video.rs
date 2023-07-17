use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn is_video<P: AsRef<Path>>(path: P) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        _ => return false,
    };
    let mut buf = vec![0; 1024];
    let result = file.read_exact(&mut buf);
    if result.is_err() {
        false
    } else {
        infer::is_video(&*buf)
    }
}
