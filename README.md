# ptest

prettify the output of `cargo test`

By default, the testing command run is just `cargo test`.

## Command Args
To pass arguments to cargo test put them after a `--`. For example, 
```bash 
cargo ptest --no-color -- --tests --no-fail-fast -- --color=always
```
would run `cargo test --tests --no-fail-fast -- --color=always` and the --no-color argument would be consumed by ptest.

--no-capture get filtered out of the arguments as it interferes with parsing the command output as it provided unpredictable output.

## TODO
 - [x] merge_outputs needs logic specifically for Doc-tests which can have multiple running x test blocks per Running Doc-tests block
 - [x] test new merge_outputs function
 - [x] wrong line being passed in to Summary::new()
 - [ ] add custom parsing for when the group is a Doc group
   - [ ] add custom parsing in ParsedTest for a doc test
 - [x] rewrite ParseTest::new() to use regex capture groups