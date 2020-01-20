#!/usr/bin/env python3
from abc import abstractmethod
from dataclasses import dataclass
from typing import Any, Callable


@dataclass(frozen=True)
class filter(object):
    impl: Callable[[Any], Any]

    @abstractmethod
    def __call__(self, elem):
        return self.impl(elem)


def attr(name: str) -> filter:
    return filter(lambda obj: obj.attrs[name])
