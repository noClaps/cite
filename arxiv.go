package main

import (
	"encoding/xml"
	"fmt"
	"net/http"
	"strings"
)

type Arxiv struct {
	Doi     *string
	Title   string   `xml:"entry>title"`
	Updated string   `xml:"entry>updated"`
	Author  []string `xml:"entry>author>name"`
}

func (a Arxiv) String() string {
	authors := make([]string, 0, len(a.Author))
	for _, author := range a.Author {
		nameParts := strings.Split(author, " ")
		last := nameParts[len(nameParts)-1]
		rest := strings.Join(nameParts[:len(nameParts)-1], " ")
		authors = append(authors, fmt.Sprintf("  - %s, %s", last, rest))
	}

	return fmt.Sprintf(`  type: "article"
  title: "%s"
  author:
  %s
  date: %s
  serial-number:
    doi: %s`,
		a.Title, strings.Join(authors, "\n  "), strings.SplitN(a.Updated, "T", 2)[0], *a.Doi)
}

func getArxiv(doi string) (Arxiv, error) {
	arxivId := strings.SplitN(doi, "arXiv.", 2)[1]
	resp, err := http.Get("https://export.arxiv.org/api/query?id_list=" + arxivId)
	if err != nil {
		return Arxiv{}, err
	}

	arxiv := new(Arxiv)
	if err := xml.NewDecoder(resp.Body).Decode(arxiv); err != nil {
		return Arxiv{}, err
	}
	arxiv.Doi = &doi
	return *arxiv, nil
}
