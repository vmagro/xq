#!/usr/bin/env python3
import graphene
from lxml import etree

from schema_gen import gen_field


# def test_gen_string_text():
#     e = etree.fromstring(r"<hello>textchild</hello>")
#     field = gen_field(e)
#     assert field == graphene.Scalar()


def test_gen_attr_no_child():
    e = etree.fromstring(r'<hello answer="42"/>')
    field = gen_field(e)
    # TODO: figure out how to properly introspect this

    # class Query(graphene.ObjectType):
    #     root = graphene.Field(field)

    # schema = graphene.Schema(query=Query)
    # print(schema.execute("{ root { answer } }"))
    # print(schema)
