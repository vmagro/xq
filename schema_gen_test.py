#!/usr/bin/env python3
from lxml import etree
from graphql import (
    graphql_sync,
    GraphQLSchema,
    GraphQLObjectType,
    GraphQLField,
    GraphQLString,
)
from graphql.utilities.schema_printer import print_schema

from schema_gen import gen_field


def query(field, querystr, elem):
    schema = GraphQLSchema(query=GraphQLObjectType(name="Query", fields={"f": field},),)
    print(print_schema(schema))
    return graphql_sync(schema, querystr, elem)


def test_string_text():
    e = etree.fromstring(r"<hello>textchild</hello>")
    field = gen_field(e)
    res = query(field, " { f }", e)
    assert res.data["f"] == "textchild"


def test_attr_no_child():
    e = etree.fromstring(r'<hello answer="42" to="world"/>')
    field = gen_field(e)
    res = query(field, " { f { answer, to } }", e)
    assert res.data["f"] == {"answer": "42", "to": "world"}


def test_children():
    e = etree.fromstring(r'<hello><child>text</child><other>blah</other></hello>')
    field = gen_field(e)
    res = query(field, " { f { child, other } }", e)
    assert res.data["f"] == {"child": "text", "other": "blah"}
