use std::{env, iter};
use std::io::{self, Write};
use std::process;

fn parse_size(n_str: &str) -> Option<usize> {
    let n = match n_str.as_bytes().last() {
        Some(b'g') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024 * 1024,
        Some(b'm') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024,
        Some(b'k') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024,
        _    => n_str.parse().ok()?,
    };
    Some(n)
}

fn usage(arg0: &str, exit: i32) {
    eprintln!("USAGE {arg0} <n[b|m|g]>[-n2[b|m|g]]");
    process::exit(exit);
}

fn main() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();

    let Some(n_str) = args.next() else {
        return usage(&arg0, 1);
    };

    if args.next().is_some() {
        usage(&arg0, 2);
    }

    const SZ_MSG: &str = "expected a number optionally followed by 'k' or 'm' or 'g'";

    let mut n_str_split = n_str.split('-');
    let n1_str = n_str_split.next().expect("impossible");
    let n2_str_opt = n_str_split.next();

    if n_str_split.next().is_some() {
        usage(&arg0, 3);
    }

    let n1 = parse_size(&n1_str).expect(SZ_MSG);

    if n1 == 0 {
        eprintln!("n must be > 0");
        usage(&arg0, 4);
    }

    if let Some(n2_str) = n2_str_opt {
    } else {
    }

    let ascii_range = b' '..=b'~';

    let ascii_vec = iter::repeat_with(|| fastrand::choice(ascii_range.clone()).unwrap())
        .take(n1)
        .collect::<Vec<_>>();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    stdout.write_all(&ascii_vec)
        .expect("failed to write output");
    let _ = stdout.write(b"\n")
        .expect("failed to write final \\n");
    stdout.flush()
        .expect("failed to flush output");
}
