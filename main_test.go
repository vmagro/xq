package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"go.starlark.net/starlark"
)

func TestGetAttr(t *testing.T) {
	res, err := Eval(strings.NewReader(`<hello answer="42"/>`), `@answer`)
	if assert.NoError(t, err) {
		assert.Equal(t, starlark.String("42"), res)
	}
}

func TestChildrenGetAttr(t *testing.T) {
	res, err := Eval(strings.NewReader(`<root><e count="1"/><e count="2"/></root>`), `/e | @count`)
	if assert.NoError(t, err) {
		assert.Equal(t, []starlark.Value{
			starlark.String("1"),
			starlark.String("2"),
		}, res)
	}
}
