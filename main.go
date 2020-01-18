package main

import (
	"fmt"
	"os"

	"github.com/antchfx/xmlquery"
	"github.com/antchfx/xpath"
	log "github.com/sirupsen/logrus"
)

func main() {
	doc, err := xmlquery.Parse(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}
	expr, err := xpath.Compile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	res := expr.Evaluate(xmlquery.CreateXPathNavigator(doc))
	switch v := res.(type) {
	case bool:
		fmt.Println(v)
	case float64:
		fmt.Println(v)
	case string:
		fmt.Println(v)
	default:
		fmt.Printf("No pretty-print supported: %+v\n", v)
	}
}
