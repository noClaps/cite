# Cite

A tool to generate [Hayagriva](https://github.com/typst/hayagriva) citations from DOI URLs.

## Build instructions

You'll need [Swift](https://swift.org).

Clone the repository:

```sh
git clone https://codeberg.org/noClaps/cite.git
cd cite
```

Build the project:

```sh
swift build
swift build --configuration release # to build in release mode
```

Run the executable:

```sh
swift run cite --help
```

## Usage

```
USAGE: cite <url>

ARGUMENTS:
  <url>                   The DOI identifier or URL to fetch data from.

OPTIONS:
  -h, --help              Show help information.
```

To use `cite`, you need to pass in a DOI URL to fetch:

```sh
cite 10.5555/12345678
```

This will print the Hayagriva YAML object (with 2 space indentation) for you to paste into your references file.

You can view the help for the command using `--help` or `-h`:

```sh
cite --help
cite -h
```
