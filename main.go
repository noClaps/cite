package main

import (
	"flag"
	"fmt"
	"log"
	"strings"
)

func main() {
	flag.Parse()

	url := flag.Arg(0)

	doi := "10." + strings.SplitN(url, "10.", 2)[1]
	reference, err := getReference(doi)
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println(reference)
}
