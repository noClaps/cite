use std::process::exit;

use crate::{args::Args, doi::Reference};

mod args;
mod doi;

fn main() -> Result<(), minreq::Error> {
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

    let reference: Reference = minreq::get(format!("http://citation.doi.org/metadata?doi={doi}"))
        .send()?
        .json()?;

    println!("{reference}");
    Ok(())
}
