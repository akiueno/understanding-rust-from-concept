use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut f = File::open("input.txt")?;

    let mut lines = Vec::new();
    let mut linebuf = String::new();

    let mut buf = Vec::new();
    let read_size = f.read_to_end(&mut buf)?;

    println!("read_size: {}", read_size);

    for cc in &buf[..read_size] {
        if *cc == b'\n' {
            lines.push(linebuf.clone());
            linebuf = String::new();
        } else {
            linebuf.push(*cc as char);
        }
    }
    println!("lines: {:?}", lines);

    Ok(())
}
