# ptest

prettify the output of `cargo test`

By default, the testing command run is `cargo test --no-fail-fast` meaning all the tests (library unit tests, binary unit tests, integration tests and docs tests will be run).

## Command Args
takes all the same arguments that cargo test does e.g. all args before the -- go to cargo and all the args after -- go to the test binary 