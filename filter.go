package main

import (
	"github.com/pkg/errors"
	"go.starlark.net/starlark"
	"go.starlark.net/syntax"
)

type filter struct {
}

func (f *filter) String() string {
	return "filter"
}

func (f *filter) Type() string {
	return "filter"
}

func (f *filter) Freeze() {

}

func (f *filter) Truth() starlark.Bool {
	return starlark.True
}

func (f *filter) Hash() (uint32, error) {
	// TODO
	return 0, nil
}

func (f *filter) Binary(op syntax.Token, y starlark.Value, side starlark.Side) (starlark.Value, error) {
	if op != syntax.PIPE {
		return nil, errors.Errorf("%s not supported, only |", op)
	}
	if y, ok := y.(*filter); ok {
		return &pipeline{filters: []*filter{f, y}}, nil
	}
	return nil, errors.Errorf("Can only add another filter to a pipeline: got %T", y)
}

type pipeline struct {
	filters []*filter
}

func (p *pipeline) String() string {
	return "pipeline"
}

func (p *pipeline) Type() string {
	return "pipeline"
}

func (p *pipeline) Freeze() {

}

func (p *pipeline) Truth() starlark.Bool {
	return starlark.True
}

func (p *pipeline) Hash() (uint32, error) {
	// TODO
	return 0, nil
}

func (p *pipeline) Binary(op syntax.Token, y starlark.Value, side starlark.Side) (starlark.Value, error) {
	if op != syntax.PIPE {
		return nil, errors.Errorf("%s not supported, only |", op)
	}
	if y, ok := y.(*filter); ok {
		new := &pipeline{filters: make([]*filter, len(p.filters)+1)}
		new.filters[len(new.filters)-1] = y
		return new, nil
	}
	if y, ok := y.(*pipeline); ok {
		new := &pipeline{filters: make([]*filter, len(p.filters)+len(y.filters))}
		copy(new.filters, p.filters)
		copy(new.filters[len(p.filters):], y.filters)
		return new, nil
	}
	return nil, errors.Errorf("Can only add another filter to a pipeline: got %T", y)
}
