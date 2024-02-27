# `rand-ascii`: A trivial and fast random ASCII chars generator

Just a quick tool to generate `<n>` random ASCII chars and print them to stdout.

## Build

```
git clone https://github.com/MoSal/rand-ascii.git
cargo build --release
```

Requires cargo (and `rustc` of course).

Check out `rustup` if you never built Rust code before.

Binary can be found at `target/release/rand-ascii`.

## Usage

```
rand-ascii <n>
```

`n` is the number of random chars to be generated and printed. Values
accept `g`/`m`/`k` suffices, which correspond to GiB/MiB/KiB values.

### Example

```
rand-ascii 1g > /dev/null
```

Or if you want to trivially test the "*performance*" of some terminal emulators:

```
time rand-ascii 1g
```

Limiting printed output to ASCII (unlike let's say, `cat /dev/urandom`) may minimize
variation of results stemming from a variation in sophistication of the text stacks
utilized by each terminal.

Besides, call me paranoid, but I wouldn't feel fully confident running `cat /dev/urandom`
in terminal emulators implemented in unsafe languages.
