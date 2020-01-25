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
                name=elem.tag,
                fields={
                    # TODO: just strings?
                    key: GraphQLField(
                        GraphQLString,
                        resolve=(lambda key: lambda e, _info: e.attrib[key])(key),
                    )
                    for key in elem.attrib.keys()
                },
            ),
            resolve=lambda e, _: e,
        )

    # TODO: look for multiple children of the same tag and make it a list
    # TODO: look for single children and make it a direct child
    def child_resolver(tag):
        print(f"making child resolver for {tag}")

        def _resolver(obj, _):
            print(f"resolving {tag} with {obj}")
            return obj.find(tag).text

        return GraphQLField(GraphQLString, resolve=_resolver)

    return GraphQLField(
        GraphQLObjectType(
            name=elem.tag,
            fields={
                # child.tag: gen_field(child) for child in elem
                child.tag: child_resolver(child.tag)
                for child in elem
            },
        ),
        resolve=lambda e, _: e,
    )
