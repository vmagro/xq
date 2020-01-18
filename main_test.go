package main

import (
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
	"go.starlark.net/starlark"
)

func TestGetAttr(t *testing.T) {
	assert.Equal(t, true, true)
	res, err := Eval(strings.NewReader(`<hello answer="42"/>`), `@answer`)
	if assert.NoError(t, err) {
		assert.Equal(t, starlark.MakeInt(42), res)
	}
}
