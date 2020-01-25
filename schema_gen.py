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
        # return GraphQLField(GraphQLString, resolve=lambda e, _info: e.text)
        return GraphQLField(GraphQLString, resolve=lambda e, _info: e.text)
    if not len(elem) and elem.attrib:
        return GraphQLField(
            GraphQLObjectType(
                elem.tag,
                lambda: {
                    # TODO: just strings?
                    key: GraphQLField(
                        GraphQLString, resolve=(lambda key: lambda e, _info: e.attrib[key])(key)
                    )
                    for key in elem.attrib.keys()
                },
            ),
            resolve=lambda e, _: e,
        )
