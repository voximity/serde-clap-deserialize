# serde-clap-deserialize

Small proc macro to add both default values to [clap] and [serde]. Inspired by
[serde-inline-default].

```rs
#[serde_clap_deserialize]
#[derive(Deserialize, Parser)]
struct MyArgs {
    #[serde_clap_deserialize(8)]
    foo: u32,
}

// MyArgs can now be serde-deserialized or clap-parsed
// and `foo` will have a default value of 8
```

[clap]: https://crates.io/crates/clap
[serde]: https://crates.io/crates/serde
[serde-inline-default]: https://crates.io/crates/serde-inline-default
