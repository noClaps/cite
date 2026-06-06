# Cite

A tool to generate [Hayagriva](https://github.com/typst/hayagriva) citations from DOI URLs.

## Usage

To use `cite`, you need to pass in a DOI URL to fetch:

```sh
cite 10.5555/12345678                    # using DOI identifier
cite https://doi.org/10.5555/12345678    # using DOI URL
```

This will print the Hayagriva YAML object (with 2 space indentation) for you to paste into your references file.

```yaml
  type: "article"
  title: "Toward a Unified Theory of High-Energy Metaphysics: Silly String Theory"
  author:
    - "Carberry, Josiah"
  date: "2008-02-29"
  serial-number:
    doi: "10.5555/12345678"
  parent:
    publisher: "Test accounts"
    title: "Journal of Psychoceramics"
    volume: "1"
    issue: "1"
```

## Build instructions

You can build it from source using [Rust](https://rust-lang.org):

```sh
cargo install --git https://github.com/noClaps/cite
```
