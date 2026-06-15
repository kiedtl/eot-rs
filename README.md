# EOT-rs

A parser for the Microsoft font "standard", Embeddable OpenType. This font
format was proposed as a web font standard in 2007 before being discontinued in
favor of WOFF.

Unfortunately in the meantime, Microsoft added support for it in PowerPoint
(among other products). While PowerPoint no longer embeds fonts in this format
as far as I can tell, Google Slides did not get the memo, and so you will still
find this format in the wild today, tucked inside a `.pptx`.

Based on [libeot](https://github.com/umanwizard/libeot). Similarities are not a
coincidence.

- https://en.wikipedia.org/wiki/Embedded_OpenType
- https://www.w3.org/submissions/EOT/
- https://www.w3.org/submissions/MTX/

## Usage

Conversion to TTF:

```rust
let data = std::fs::read(inp).unwrap();
let (metadata, out) = eot::eot_to_ttf(&data).unwrap();
```

Checking the EOT metadata:

```rust
let metadata = eot::metadata::read_metadata(&data).unwrap();
```

## Status

This crate faithfully reproduces most of the features and all of the bugs from
the original libeot. Tread lightly.

If you find an EOT that this library can't parse, please report it **here** and
not upstream.

NOTE: this crate was originally transpiled from the C library via `c2rust`. All
instances of raw pointers and `unsafe` has been refactored away, but the code is
still un-idiomatic in many cases. Further cleanups are needed.

## License

Mozilla Public License 2.0, in compliance with the original license.
