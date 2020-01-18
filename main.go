package main

import (
	"encoding/xml"
	"io"
	"os"
	"regexp"

	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
	"go.starlark.net/resolve"
	"go.starlark.net/starlark"
	"go.starlark.net/starlarkstruct"
)

var attrReplace = regexp.MustCompile(`@(\w+)\s?`)

func init() {
	resolve.AllowNestedDef = true
	resolve.AllowLambda = true
}

func main() {
	src := os.Args[1]
	res, err := Eval(os.Stdin, src)
	if err != nil {
		log.Fatal(err)
	}
	log.Infof("%+v", res)
}

type Node struct {
	XMLName xml.Name
	Attrs   []xml.Attr `xml:"-"`
	Content []byte     `xml:",innerxml"`
	Nodes   []Node     `xml:",any"`
}

func (n *Node) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	n.Attrs = start.Attr
	type node Node

	return d.DecodeElement((*node)(n), &start)
}

func Eval(reader io.Reader, src string) (interface{}, error) {
	src = attrReplace.ReplaceAllString(src, `attr("$1")`)
	log.Infof(src)
	thread := &starlark.Thread{}
	globals, err := starlark.ExecFile(thread, "filters.star", nil, starlark.StringDict{
		"struct": starlark.NewBuiltin("struct", starlarkstruct.Make),
	})
	res, err := starlark.Eval(thread, "", src, globals)
	if err != nil {
		return nil, err
	}

	dec := xml.NewDecoder(reader)
	node := Node{}
	err = dec.Decode(&node)
	if err != nil {
		return nil, errors.Wrap(err, "Failed to parse XML")
	}

	attrDict := starlark.NewDict(1)
	attrDict.SetKey(starlark.String("answer"), starlark.MakeInt(42))
	if res, ok := res.(starlark.Callable); ok {
		return starlark.Call(
			thread,
			res,
			[]starlark.Value{
				starlarkstruct.FromStringDict(
					starlarkstruct.Default,
					starlark.StringDict{
						"attr": attrDict,
					},
				),
			},
			nil,
		)
	}
	return nil, errors.New("Result is not callable")
}
