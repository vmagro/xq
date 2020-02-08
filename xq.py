#!/usr/bin/env python3
import sys
import xml.sax
from dataclasses import dataclass, field
from typing import Any, MutableSequence, Mapping
from graphql import (
    GraphQLSchema,
    GraphQLObjectType,
    GraphQLField,
    GraphQLString,
)
from lxml import etree

import filters
import gql
import strawberry
from graphql.utilities.schema_printer import print_schema
from graphql import graphql_sync, GraphQLSchema, GraphQLObjectType


from schema_gen import gen_field


def xq():
    # print(gql.schema)
    # print(gql.schema.execute(sys.argv[1]))

    root = etree.parse(sys.stdin).getroot()
    query = sys.argv[1]
    root_field = gen_field(root)

    schema = GraphQLSchema(query=root_field)
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
