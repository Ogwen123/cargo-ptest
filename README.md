# ptest

prettify the output of `cargo test`

By default, the testing command run is `cargo test --no-fail-fast` meaning all the tests (library unit tests, binary unit tests, integration tests and docs tests will be run).

## Command Args
All arguments passed to `cargo ptest` are passed on to cargo test.

--no-capture get filtered out of the arguments as it interferes with parsing the command output as it provided unpredictable output.