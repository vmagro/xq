#!/usr/bin/env python3
import sys
import xml.sax
from dataclasses import dataclass, field
from typing import Any, MutableSequence, Mapping
from graphql import GraphQLSchema, GraphQLObjectType, GraphQLField, GraphQLString

import filters
import gql
import strawberry
from graphql.utilities.schema_printer import print_schema



@dataclass(frozen=True)
class elem(object):
    tag: str
    attrs: Mapping[str, Any]


@dataclass
class StreamHandler(xml.sax.handler.ContentHandler):
    src: str
    elem_stack: MutableSequence[str] = field(default_factory=list)

    def startElement(self, name, attrs):
        self.elem_stack.append(name)
        attrs = {k: v for k, v in attrs.items()}
        e = elem(name, attrs)
        print(f"start element {name} {attrs}")
        f = eval(self.src, {k: getattr(filters, k) for k in dir(filters)})
        res = f(e)
        print(f"filter returned {res}")

    def endElement(self, name):
        self.elem_stack.pop()
        print(f"end element {name}")


def xq():
    # print(gql.schema)
    # print(gql.schema.execute(sys.argv[1]))

    schema = GraphQLSchema(
        query=GraphQLObjectType(
            name="Query",
            fields={
                "hello": GraphQLField(GraphQLString, resolve=lambda obj, info: "world")
            },
        )
    )
    print(schema)
    print(print_schema(schema))

    # @strawberry.type
    # class User:
    #     name: str
    #     age: int


    # @strawberry.type
    # class Query:
    #     @strawberry.field
    #     def user(self, info) -> User:
    #         return User(name="Patrick", age=100)


    # schema = strawberry.Schema(query=Query)
    # print(schema)
    # parser = xml.sax.make_parser()
    # parser.setContentHandler(StreamHandler(sys.argv[1]))
    # parser.parse(sys.stdin)


if __name__ == "__main__":
    xq()
