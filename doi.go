package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
)

type DOI struct {
	Author []struct {
		Family string
		Given  string
	}
	Issued struct {
		DateParts [1][3]*int `json:"date-parts"`
	}
	DOI            string
	Title          string
	Publisher      string
	Issue          *string
	Page           *string
	Volume         *string
	ContainerTitle *string `json:"container-title"`
	Published      *struct {
		DateParts [1][3]*int `json:"date-parts"`
	}
}

func (d *DOI) String() string {
	dateParts := d.Issued.DateParts
	if d.Published != nil {
		dateParts = d.Published.DateParts
	}
	date := ""
	if year := dateParts[0][0]; year != nil {
		date += fmt.Sprintf("%04d", *year)
		if month := dateParts[0][1]; month != nil {
			date += fmt.Sprintf("-%02d", *month)
			if day := dateParts[0][2]; day != nil {
				date += fmt.Sprintf("-%02d", *day)
			}
		}
	}

	authors := make([]string, 0, len(d.Author))
	for _, author := range d.Author {
		authors = append(authors, fmt.Sprintf("  - %s, %s", author.Family, author.Given))
	}

	page := ""
	if d.Page != nil {
		page = fmt.Sprintf(`
  page-range: "%s"`, *d.Page)
	}
	containerTitle := ""
	if d.ContainerTitle != nil {
		containerTitle = fmt.Sprintf(`
    title: "%s"`, *d.ContainerTitle)
	}
	volume := ""
	if d.Volume != nil {
		volume = fmt.Sprintf(`
    volume: "%s"`, *d.Volume)
	}
	issue := ""
	if d.Issue != nil {
		issue = fmt.Sprintf(`
    issue: "%s"`, *d.Issue)
	}

	return fmt.Sprintf(
		`  type: "article"
  title: "%s"
  author:
  %s
  date: "%s"%s
  serial-number:
    doi: "%s"
  parent:
    publisher: "%s"%s%s%s`,
		d.Title, strings.Join(authors, "\n  "), date, page, d.DOI, d.Publisher,
		containerTitle, volume, issue)
}

func getDoi(doi string) (*DOI, error) {
	resp, err := http.Get("https://citation.doi.org/metadata?doi=" + doi)
	if err != nil {
		return nil, err
	}

	if resp.StatusCode != http.StatusOK {
		body, err := io.ReadAll(resp.Body)
		if err != nil {
			return nil, err
		}
		return nil, fmt.Errorf("HTTP status %d: %s", resp.StatusCode, body)
	}

	reference := new(DOI)
	if err := json.NewDecoder(resp.Body).Decode(reference); err != nil {
		return nil, err
	}

	return reference, nil
}
