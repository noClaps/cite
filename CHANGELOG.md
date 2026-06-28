# Changelog

## v0.1.2

- Update dependencies
- Switch to using [clap](https://github.com/clap-rs/clap) for argument parsing
- Improve error handling and error messages

## v0.1.0

The initial release of Cite!

To use Cite, you can run:

```sh
cite 10.5555/12345678 # Replace with your DOI
cite https://doi.org/10.5555/12345678 # You can also use a DOI URL
```

This will return

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
