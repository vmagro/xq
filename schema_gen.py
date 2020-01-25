#!/usr/bin/env python3
from lxml import etree


from graphql import GraphQLObjectType, GraphQLField, GraphQLString


def gen_field(elem: etree.Element) -> GraphQLField:
    # if there are no children and no attributes, return the text node as a
    # simple scalar type
    if not len(elem) and not elem.attrib:
        print("No children and no attributes")
        # it might be a string, int, float or bool and will be disambiguated
        # later by the resolver
        return GraphQLField(GraphQLString, resolve=lambda e, _info: e.text)
    if not len(elem) and elem.attrib:
        return GraphQLObjectType(
            name=elem.tag,
            fields={
                # TODO: just strings?
                # key: GraphQLField(GraphQLString, resolve=lambda e, _info: e.attrib[key])
                key: GraphQLField(GraphQLString, resolve=lambda e, _info: "test")
                for key in elem.attrib.keys()
            },
        )
