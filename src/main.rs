use std::process::exit;

use argparse::ArgParse;

use crate::doi::Doi;

mod doi;

fn main() -> Result<(), ureq::Error> {
    let args = ArgParse::new()
        .positional("url", "The DOI identifier or URL to fetch data from")
        .parse();
    let url: String = args.positional("url").unwrap();

    let doi = format!(
        "10.{}",
        match url.split_once("10.") {
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
