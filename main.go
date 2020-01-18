package main

import (
	"io"
	"os"
	"regexp"

	log "github.com/sirupsen/logrus"
	"go.starlark.net/starlark"
)

var attrReplace = regexp.MustCompile(`@(\w+)\s?`)

func main() {
	src := os.Args[1]
	res, err := Eval(os.Stdin, src)
	if err != nil {
		log.Fatal(err)
	}
	log.Infof("%+v", res)
}

func Eval(reader io.Reader, src string) (interface{}, error) {
	src = attrReplace.ReplaceAllString(src, `attr("$1")`)
	log.Infof(src)
	thread := &starlark.Thread{}
	res, err := starlark.Eval(thread, "", src, nil)
	if err != nil {
		return nil, err
	}
	log.Infof("%+v", res)
	return nil, nil
}
