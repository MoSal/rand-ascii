use std::{env, iter};
use std::io::{self, Write};
use std::process;

fn parse_size(arg0: &str, n_str: &str) -> usize {
    let ret = || {
        let n = match n_str.as_bytes().last() {
            Some(b'g') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024 * 1024,
            Some(b'm') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024 * 1024,
            Some(b'k') => n_str[0..n_str.len()-1].parse::<usize>().ok()? * 1024,
            _    => n_str.parse().ok()?,
        };
        Some(n)
    };
    match ret() {
        None => usage(arg0, 1),
        Some(ret) => ret,
    }
}

fn usage(arg0: &str, exit: i32) -> ! {
    eprintln!("USAGE {arg0} <n> [<short>-<long>]");
    eprintln!("Dump random output to stdout comprised of printable ascii characters only.");
    eprintln!("USAGE {arg0} <n>");
    eprintln!(" <n>: the length of single-line output dumped.");
    eprintln!("USAGE {arg0} <n> <short>-<long>");
    eprintln!(" <n>: the number of lines dumped.");
    eprintln!(" <short>-<long>: range for the random length of each line dumped.");
    process::exit(exit);
}

fn main() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();

    let Some(n_str) = args.next() else {
        usage(&arg0, 1);
    };

    let n_range_str_opt = args.next();

    if args.next().is_some() {
        usage(&arg0, 2);
    }

    let n = parse_size(&arg0, &n_str);

    if n == 0 {
        eprintln!("n({n}) must be > 0");
        usage(&arg0, 3);
    }

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let ascii_range = b' '..=b'~';

    if let Some(n_range_str) = n_range_str_opt {
        let mut n_range_str_split = n_range_str.split('-');
        let short_str = n_range_str_split.next().expect("impossible");

        let Some(long_str) = n_range_str_split.next() else {
            usage(&arg0, 4);
        };

        if n_range_str_split.next().is_some() {
            usage(&arg0, 5);
        }

        let short = parse_size(&arg0, short_str);
        let long = parse_size(&arg0, long_str);

        if long < short {
            eprintln!("long({long}) must be >= short({short})");
            usage(&arg0, 5);
        }

        if long == usize::MAX {
            eprintln!("long({long}) must be < {}", usize::MAX);
            usage(&arg0, 5);
        }

        let line_length_range = short..long+1;

        let line_lengths = iter::repeat_with(|| fastrand::choice(line_length_range.clone()).unwrap())
            .take(n);

        for ll in line_lengths {
            let ascii_vec = iter::repeat_with(|| fastrand::choice(ascii_range.clone()).unwrap())
                .take(ll)
                .chain([b'\n'])
                .collect::<Vec<_>>();

            stdout.write_all(&ascii_vec)
                .expect("failed to write line output");
        }
    } else {
        let ascii_vec = iter::repeat_with(|| fastrand::choice(ascii_range.clone()).unwrap())
            .take(n)
            .chain([b'\n'])
            .collect::<Vec<_>>();

        stdout.write_all(&ascii_vec)
            .expect("failed to write output");
    }

    stdout.flush()
        .expect("failed to flush output");
}
