#!/usr/bin/env python3
from dataclasses import dataclass
from typing import Mapping, Any

import filters


@dataclass(frozen=True)
class elem(object):
    attrs: Mapping[str, Any]


def test_attr_filter():
    assert filters.attr("hello")(elem(attrs={"hello": "world"})) == "world"
