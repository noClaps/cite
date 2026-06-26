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

    let reference: Reference = minreq::get(format!("https://citation.doi.org/metadata?doi={doi}"))
        .send()?
        .json()?;

    println!("{reference}");
    Ok(())
}
