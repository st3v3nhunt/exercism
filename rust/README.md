# Rust track

> Exercism [Rust track](https://exercism.org/tracks/rust)

## Testing

Use [cargo-watch](https://crates.io/crates/cargo-watch) to continually run
tests. After installing, run `cargo watch -x test` in the directory of the
exercise for continually running tests.

If the tests attempt to output to stdout e.g. `print!() or println!()`, by
default nothing will be displayed,
[source](https://doc.rust-lang.org/cargo/commands/cargo-test.html#display-options).
The output can be turned on by starting the test runner with the option
`--nocapture` e.g.

```sh
cargo watch -x "test -- --nocapture
```

Rather than having to change dir to run the command, the working directory can
be passed as an option. When in the rust dir run this:

```sh
cargo watch -x "test -- --nocapture" --workdir <path_to_dir>
```
