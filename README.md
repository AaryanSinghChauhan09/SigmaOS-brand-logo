# SigmaOS Brand Logo

This crate exports a `Branding` data model for product name, version, tagline,
copyright, support URL, and caller-provided logo lines. Setters validate and
store metadata, and `about()` formats it as text. The crate does not load image
assets or render a logo, wallpaper, or screensaver.

The crate depends on the sibling `SigmaOS-kernel` checkout through a local
path.

## License

GPL-3.0-or-later. See [`LICENSE`](LICENSE) and [`Cargo.toml`](Cargo.toml).
