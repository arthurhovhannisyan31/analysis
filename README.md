<div style="display: flex; flex-direction: column; justify-content: center; align-items: center;" align="center">
    <h1><code>analysis</code></h1>
    <h4>Built with <a href="https://rust-lang.org/">🦀</a></h4>
</div>

[![main](https://github.com/arthurhovhannisyan31/analysis/actions/workflows/code-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/analysis/actions/workflows/code-validation.yml)
[![main](https://github.com/arthurhovhannisyan31/analysis/actions/workflows/packages-validation.yml/badge.svg?branch=main)](https://github.com/arthurhovhannisyan31/analysis/actions/workflows/packages-validation.yml)

## Overview

This is a demo crate which shows code refactoring results.

The main purpose of the crate is text parsing using specific format. Please see [main](./src/main.rs) for details.

## Description

The main part of code refactor covers code generation for functions with different number of arguments. Please
see [declarative](./src/macros.rs) and [procedural](./src/perm_macro) macros implementation for details.
Rest of code refactor covers performance, safety and code readability issues.

## Usage

Please run following commands for test purposes:

```bash
cargo test -- --nocapture
cargo test test_all -- --nocapture
cargo run example.log 
```

## Stack

- [Rust](https://rust-lang.org/)
- [itertools](https://crates.io/crates/itertools)
- [proc-macro2](https://crates.io/crates/proc-macro2)
- [quote](https://crates.io/crates/quote)
- [syn](https://crates.io/crates/syn)
- [tightness](https://crates.io/crates/tightness)

## Credits

Crate implemented as part of the [Yandex practicum](https://practicum.yandex.ru/) course.

## License

Licensed under either of your options.

* Apache License, Version 2.0, [LICENSE-APACHE](./LICENSE_APACHE) or http://www.apache.org/licenses/LICENSE-2.0
* MIT license [LICENSE-MIT](./LICENSE_MIT) or http://opensource.org/licenses/MIT
