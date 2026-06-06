use std::{env, process::exit};

pub struct Args {
    pub url: String,
}

impl Args {
    pub fn parse() -> Self {
        let args: Vec<_> = env::args().collect();

        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            println!(
                r#"
A tool to generate Hayagriva citations from DOI URLs.

Usage: cite <url>

Arguments:
  <url>         The DOI identifier or URL to fetch data from.

Options:
  -h, --help    Show this help and exit.

To use cite, you need to pass in a DOI URL to fetch:

  $ cite 10.5555/12345678                    # using DOI identifier
  $ cite https://doi.org/10.5555/12345678    # using DOI URL

This will print the Hayagriva YAML object (with two space indentation) for you
to paste into your references file.

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
"#
            );
            exit(0);
        }

        if args.len() < 2 {
            println!(
                "
Missing required argument: <url>

Usage: cite <url>

See `cite --help` for the full documentation.
"
            );
            exit(1);
        }

        if args.len() > 2 {
            println!(
                r"
Extra argument provided: {}

Usage: cite <url>

See `cite --help` for the full documentation.
",
                args[2]
            );
            exit(1);
        }

        Args {
            url: args[1].clone(),
        }
    }
}
