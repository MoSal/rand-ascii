use std::env;
use std::io::{self, Write};
use fastrand::char as rand_ch;

fn main() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();
    let n_str_opt = args.next();
    let trailing_arg = args.next();

    if let (Some(n_str), None) = (n_str_opt, trailing_arg) {
        let n = match n_str.as_bytes().last() {
            Some(b'g') => n_str[0..n_str.len()-1].parse::<usize>().unwrap() * 1024 * 1024 * 1024,
            Some(b'm') => n_str[0..n_str.len()-1].parse::<usize>().unwrap() * 1024 * 1024,
            Some(b'k') => n_str[0..n_str.len()-1].parse::<usize>().unwrap() * 1024,
            _    => n_str.parse().unwrap(),
        };

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        //let mut buf = [0u8; 1];
        let mut buf = [0u8; 1];
        for _ in 0..(n.max(1)-1) {
            let _ = stdout.write(rand_ch(' '..='~').encode_utf8(&mut buf).as_bytes());
        }
        let _ = stdout.write(b"\n");
    } else {
        eprintln!("USAGE {arg0} <n[b|m|g]>");
    }
}
