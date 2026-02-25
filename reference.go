package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
)

type Reference struct {
	ContainerTitle []string `json:"container-title"`
	Issue          *string  `json:"issue"`
	Doi            string   `json:"DOI"`
	Published      struct {
		DateParts [][]*int `json:"date-parts"`
	} `json:"published"`
	Page   *string  `json:"page"`
	Title  []string `json:"title"`
	Volume string   `json:"volume"`
	Author []struct {
		Given  *string `json:"given"`
		Family *string `json:"family"`
	} `json:"author"`
}

func (ref Reference) String() string {
	authors := make([]string, 0, len(ref.Author))
	for _, author := range ref.Author {
		if author.Given == nil || author.Family == nil {
			continue
		}
		authors = append(authors, fmt.Sprintf("  - %s, %s", *author.Family, *author.Given))
	}
	published := ""
	if year := ref.Published.DateParts[0][0]; year != nil {
		published += fmt.Sprintf("%04d", *year)
		if month := ref.Published.DateParts[0][1]; month != nil {
			published += fmt.Sprintf("-%02d", *month)
			if day := ref.Published.DateParts[0][2]; day != nil {
				published += fmt.Sprintf("-%02d", *day)
			}
		}
	}
	pageRange := ""
	if ref.Page != nil {
		pageRange = "\n  page-range: " + *ref.Page
	}
	issue := ""
	if ref.Issue != nil {
		issue = "\n    issue: " + *ref.Issue
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
    title: "%s"
    volume: %s%s`,
		ref.Title[0], strings.Join(authors, "  -\n"), published, pageRange, ref.Doi,
		ref.ContainerTitle[0], ref.Volume, issue)

}

type CrossRef struct {
	Message Reference `json:"message"`
}

func getReference(doi string) (Reference, error) {
	resp, err := http.Get("https://api.crossref.org/works/" + doi)
	if err != nil {
		return Reference{}, err
	}
	reference := new(CrossRef)
	if err := json.NewDecoder(resp.Body).Decode(reference); err != nil {
		return Reference{}, err
	}

	return reference.Message, nil
}
