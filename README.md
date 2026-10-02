# modelname

[![crates.io](https://img.shields.io/crates/v/modelname.svg)](https://crates.io/crates/modelname)
[![docs.rs](https://docs.rs/modelname/badge.svg)](https://docs.rs/modelname)

Parse AI model names into something you can sort.

```rust
let m = modelname::parse("claude-opus-4-1-20250805");

m.vendor;  // Some("anthropic")
m.family;  // "claude-opus"
m.version; // Some("4.1")
m.date;    // Some(2025-08-05)
```

Rust port of my Go package [modelname](https://github.com/BoNour/modelname).
Same rules, same test cases, no dependencies.

It reads API IDs (`gpt-4.1-mini`, `meta-llama/llama-3.3-70b-instruct:free`)
and display names (`Gemini 3.1 Pro Preview`) the same way. Old and new
Claude names land in the same family, so `claude-3-5-sonnet` and
`claude-sonnet-4-5` can be compared.

## Newest in a family

```rust
let ids = ["claude-sonnet-4-5", "claude-3-7-sonnet-20250219", "claude-sonnet-4-6"];
let id = modelname::latest(ids, "claude-sonnet"); // Some("claude-sonnet-4-6")
```

`modelname::compare(&a, &b)` gives you the ordering if you want to sort a
whole list.

## What it won't do

It doesn't know about every model and it doesn't call any API. It's string
parsing. If a name is weird enough it'll put everything in `family` and
leave `version` empty, which is usually the right call.

Most of the test cases are real names from [whizi.io/models](https://whizi.io/models/).
If you find one it gets wrong, open an issue with the name.

## License

MIT
