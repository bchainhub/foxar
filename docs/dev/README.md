# Contributing Quick Start

The orbitalis Rust project is organized as a regular [Cargo workspace][cargo-workspace].

Simply running

```
$ cargo test
```

should be enough to get you started!

To learn more about how orbitalis's tools works, see [./architecture.md](./architecture.md).
It also explains the high-level layout of some aspects of the source code.
To read more about how to use it, see [📖 Orbitalis Book][orbitalis-book]
Note though, that the internal documentation is very incomplete.

# Getting in Touch

See also [Getting Help](../../README.md#getting-help)

# Issue Labels

-   [good-first-issue](https://github.com/orbitalis-rs/orbitalis/labels/good%20first%20issue)
    are good issues to get into the project.
-   [D-easy](https://github.com/orbitalis-rs/orbitalis/issues?q=is%3Aopen+is%3Aissue+label%3AD-easy),
    [D-average](https://github.com/orbitalis-rs/orbitalis/issues?q=is%3Aopen+is%3Aissue+label%3AD-medium),
    [D-hard](https://github.com/orbitalis-rs/orbitalis/issues?q=is%3Aopen+is%3Aissue+label%3AD-hard),
    [D-chore](https://github.com/orbitalis-rs/orbitalis/issues?q=is%3Aopen+is%3Aissue+label%3AD-chore),
    labels indicate how hard it would be to write a fix or add a feature.

# CI

We use GitHub Actions for CI.
We use [cargo-nextest][nextest] as the test runner
If `cargo test` passes locally, that's a good sign that CI will be green as well.
We also have tests that make use of forking mode which can be long running if the required state is not already cached locally.
Forking-related tests are executed exclusively in a separate CI job, they are identified by `fork` in their name.
So all of them can be easily skipped by `cargo t -- --skip fork`

[orbitalis-book]: https://book.getorbitalis.sh
[cargo-workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[nextest]: https://nexte.st/
