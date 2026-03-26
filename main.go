package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	flag.Usage = func() {
		w := flag.CommandLine.Output()
		fmt.Fprintf(w, `Usage: %s <url>

Arguments:
  url
        The DOI identifier or URL to fetch data from.
`, filepath.Base(os.Args[0]))
	}

	flag.Parse()

	url := flag.Arg(0)

	doi := "10." + strings.SplitN(url, "10.", 2)[1]
	if strings.Contains(doi, "arXiv") {
		arxiv, err := getArxiv(doi)
		if err != nil {
			log.Fatalln(err)
		}
		fmt.Println(arxiv)
		return
	}

	crossref, err := getCrossref(doi)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(crossref)
}
