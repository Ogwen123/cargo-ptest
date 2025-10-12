# ptest

Prettify the output of `cargo test`

If installed using cargo install it acts as a command line tool, if used as a package provides methods for running and parsing the output of cargo test.

## Command Args
To pass arguments to cargo test put them after a `--`. For example, 
```bash 
cargo ptest --no-color -- --tests --no-fail-fast -- --color=always
```
would run `cargo test --tests --no-fail-fast -- --color=always` and the --no-color argument would be consumed by ptest.

### Filtered Commands
The following commands are filtered out before running `cargo test` as they add extra formatting that the parser cannot handle.
```txt
   --nocapture
   -v
   --verbose
   --color=always
   --color=auto
   --color=never
```
