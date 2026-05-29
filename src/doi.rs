use std::fmt::{Display, Write};

use serde::Deserialize;

#[derive(Deserialize)]
struct Author {
    family: String,
    given: String,
}

#[derive(Deserialize, Clone, Copy)]
struct DateParts {
    #[serde(rename(deserialize = "date-parts"))]
    date_parts: ((Option<i64>, Option<i64>, Option<i64>),),
}

#[derive(Deserialize)]
pub struct Reference {
    author: Vec<Author>,
    issued: DateParts,
    #[serde(rename(deserialize = "DOI"))]
    doi: String,
    title: String,
    publisher: String,
    issue: Option<String>,
    page: Option<String>,
    volume: Option<String>,
    #[serde(rename(deserialize = "container-title"))]
    container_title: Option<String>,
    published: Option<DateParts>,
}

impl Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  type: \"article\"")?;
        writeln!(f, "  title: \"{}\"", self.title)?;
        writeln!(f, "  author:")?;
        for author in &self.author {
            writeln!(f, "    - \"{}, {}\"", author.family, author.given)?;
        }

        let date_parts = self.published.unwrap_or(self.issued).date_parts.0;
        if let Some(year) = date_parts.0 {
            let mut date = format!("{year:04}");
            if let Some(month) = date_parts.1 {
                write!(date, "-{month:02}")?;
                if let Some(day) = date_parts.2 {
                    write!(date, "-{day:02}")?;
                }
            }
            writeln!(f, "  date: \"{date}\"")?;
        }

        if let Some(page) = &self.page {
            writeln!(f, "  page-range: \"{page}\"")?;
        }
        writeln!(f, "  serial-number:")?;
        writeln!(f, "    doi: \"{}\"", self.doi)?;
        writeln!(f, "  parent:")?;
        writeln!(f, "    publisher: \"{}\"", self.publisher)?;
        if let Some(container_title) = &self.container_title {
            writeln!(f, "    title: \"{container_title}\"")?;
        }
        if let Some(volume) = &self.volume {
            writeln!(f, "    volume: \"{volume}\"")?;
        }
        if let Some(issue) = &self.issue {
            write!(f, "    issue: \"{issue}\"")?;
        }

        Ok(())
    }
}
