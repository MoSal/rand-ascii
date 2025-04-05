# `rand-ascii`: A trivial and fast random ASCII chars generator

Just a quick tool to generate single-line `<n>` random ASCII chars and print
them to stdout. Or optionally output `<n>` lines with a random length in
the range of `<short>-<long>`.

The range of characters used is `' '-'~'`, which obviously includes
spaces, meaning the output contains "words", which can be relevant in some
test cases.

The full list of characters is:

```
 !"#$%&'()*+,./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
```

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
rand-ascii <n> [<short>-<long>]
```

If `n` is the only argument passed, it denotes the number of random chars to be
generated and printed.

If a second argument `<short>-<long>` is passed, then `<n>` becomes the number
of lines generated, with each having a length in the range of `<short>-<long>`.

All values accept `g`/`m`/`k` suffices, which correspond to GiB/MiB/KiB values.

### Example

```
rand-ascii 1g > /dev/null
rand-ascii 100k 30-150 > /dev/null
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
