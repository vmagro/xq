package main

import (
	"encoding/xml"
	"fmt"
	"io"
	"os"
	"regexp"

	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
	"go.starlark.net/resolve"
	"go.starlark.net/starlark"
	"go.starlark.net/starlarkstruct"
)

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

func (n *Node) String() string {
	bytes, err := xml.Marshal(n)
	if err != nil {
		if n.XMLName.Space != "" {
			return fmt.Sprintf("<%s:%s />", n.XMLName.Space, n.XMLName.Local)
		}
		return fmt.Sprintf("<%s />", n.XMLName.Local)
	}
	return string(bytes)
}

func (n *Node) Type() string {
	return "Node"
}

func (n *Node) Freeze() {

}

func (n *Node) Truth() starlark.Bool {
	return starlark.True
}

func (n *Node) Hash() (uint32, error) {
	// TODO
	return 0, nil
}

func (n *Node) Attr(name string) (starlark.Value, error) {
	switch name {
	case "attrs":
		d := starlark.NewDict(len(n.Attrs))
		for _, attr := range n.Attrs {
			key := attr.Name.Local
			if attr.Name.Space != "" {
				key = attr.Name.Space + ":" + key
			}
			d.SetKey(starlark.String(key), starlark.String(attr.Value))
		}
		return d, nil
	}
	return nil, errors.Errorf("No attr '%s'", name)
}

func (n *Node) AttrNames() []string {
	return []string{"attrs"}
}

func (n *Node) UnmarshalXML(d *xml.Decoder, start xml.StartElement) error {
	n.Attrs = start.Attr
	type node Node

	return d.DecodeElement((*node)(n), &start)
}

var attrReplace = regexp.MustCompile(`@(\w+)\b`)
var childReplace = regexp.MustCompile(`/(\w+)\b`)

func Eval(reader io.Reader, src string) (interface{}, error) {
	src = attrReplace.ReplaceAllString(src, `attr("$1")`)
	src = childReplace.ReplaceAllString(src, `children("$1")`)
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
			[]starlark.Value{&node},
			nil,
		)
	}
	return nil, errors.New("Result is not callable")
}
