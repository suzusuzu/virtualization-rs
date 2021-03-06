# virtualization-rs

[![Crates](https://img.shields.io/crates/v/virtualization-rs.svg)](https://crates.io/crates/virtualization-rs)
[![Document](https://docs.rs/virtualization-rs/badge.svg)](https://docs.rs/virtualization-rs) 

## Rust bindings for [Virtualization.framework](https://developer.apple.com/documentation/virtualization?language=objc)

virtualization-rs provides the API of the Apple Virtualization.framework in Rust language.

## Requirements

macOS Big Sur

## Usage

```
[dependencies]
virtualization-rs = "0.1.2"
```

## Example

The [example](https://github.com/suzusuzu/virtualization-rs/blob/main/examples/simplevm.rs) is inspired from [SimpleVM](https://github.com/KhaosT/SimpleVM).

```sh
make release
./target/release/examples/simplevm --kernel ubuntu/vmlinuz --initrd ubuntu/initrd --disk ubuntu/ubuntu.iso
```

![simplevm](./img/simplevm.gif)