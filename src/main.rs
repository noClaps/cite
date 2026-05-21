use std::process::exit;

use clap::Parser;

use crate::doi::Doi;

mod doi;

#[derive(Parser)]
struct Args {
    /// The DOI identifier or URL to fetch data from
    url: String,
}

fn main() -> Result<(), ureq::Error> {
    let args = Args::parse();

    let doi = format!(
        "10.{}",
        match args.url.split_once("10.") {
            Some((_, id)) => id,
            None => {
                eprintln!("Invalid DOI. DOI identifiers begin with `10.`");
                exit(1)
            }
        }
    );

    let reference: Doi = ureq::get(format!("https://citation.doi.org/metadata?doi={}", doi))
        .call()?
        .body_mut()
        .read_json()?;

    println!("{}", reference);
    Ok(())
}
