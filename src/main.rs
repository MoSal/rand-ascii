use std::env;
use std::io::{self, Write};
use fastrand::char as rand_ch;

fn parse_size(n_str: &str) -> Option<usize> {
    let n = match n_str.as_bytes().last() {
        Some(b'g') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024 * 1024,
        Some(b'm') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024,
        Some(b'k') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024,
        _    => n_str.parse().ok()?,
    };
    Some(n)
}

fn main() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();
    let n_str_opt = args.next();
    let trailing_arg = args.next();
    const SZ_MSG: &str = "expected a number optionally followed by 'k' or 'm' or 'g'";

    if let (Some(n_str), None) = (n_str_opt, trailing_arg) {
        let n = parse_size(&n_str).expect(SZ_MSG);

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        let mut buf = [0u8; 1];
        for _ in 0..n {
            let _ = stdout.write(rand_ch(' '..='~').encode_utf8(&mut buf).as_bytes());
        }
        let _ = stdout.write(b"\n");
    } else {
        eprintln!("USAGE {arg0} <n[b|m|g]>");
    }
}
