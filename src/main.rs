use std::process::exit;

use clap::Parser;

use crate::doi::Reference;

mod doi;

#[derive(Parser)]
#[clap(version)]
/// A tool to generate Hayagriva citations from DOI URLs
struct Args {
    /// The DOI identifier or URL to fetch data from
    url: String,
}

fn main() {
    let args = Args::parse();

    let doi = format!(
        "10.{}",
        if let Some((_, id)) = args.url.split_once("10.") {
            id
        } else {
            eprintln!("Invalid DOI. DOI identifiers begin with `10.`");
            exit(1)
        }
    );

    let reference: Reference =
        match minreq::get(format!("https://citation.doi.org/metadata?doi={doi}")).send() {
            Ok(res) if res.status_code != 200 => {
                eprintln!("DOI not found: {doi}");
                exit(1);
            }
            Ok(res) => match res.json() {
                Ok(json) => json,
                Err(err) => match err {
                    minreq::Error::SerdeJsonError(err) => {
                        eprintln!("Error parsing JSON body: {err}");
                        eprintln!("URL: https://citation.doi.org/metadata?doi={doi}");
                        eprintln!("Response: {}", res.as_str().unwrap());
                        exit(1)
                    }
                    minreq::Error::InvalidUtf8InBody(err) => {
                        eprintln!("URL: https://citation.doi.org/metadata?doi={doi}");
                        eprintln!("Invalid UTF-8 in response: {err}");
                        exit(1);
                    }
                    _ => unreachable!(),
                },
            },
            Err(err) => {
                eprintln!("Error during request: {err}");
                exit(1);
            }
        };

    println!("{reference}");
}
