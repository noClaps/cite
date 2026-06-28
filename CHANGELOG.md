# Changelog

## v0.1.2

- Update dependencies (9dfc2d9f25a9d5badf6ddcc131d2e7a59e0e7abc)
- Switch to using [clap](https://github.com/clap-rs/clap) for argument parsing (f918d8353ec13b3e3cf001c6d08d7ad47948d905)
- Improve error handling and error messages (f69c6301f0831f3d667ad34631ed6b1197fd3e23)

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
