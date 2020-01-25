#!/usr/bin/env python3
import graphene
from lxml import etree


def gen_field(elem: etree.Element):
    print(elem)

    # if there are no children and no attributes, return the text node as a
    # simple scalar type
    if not len(elem) and not elem.attrib:
        print("No children and no attributes")
        # it might be a string, int, float or bool and will be disambiguated
        # later by the resolver
        return graphene.Scalar()
    if not len(elem) and elem.attrib:
        # ObjectType does a lot of introspection at class creation time, so do
        # all our temporary work in this class
        # class elemtype_tmp(object):
        #     __name__ = elem.tag

        # for key, _ in elem.attrib.items():
        #     setattr(elemtype_tmp, key, graphene.Scalar())

        # print(elemtype_tmp)
        print(f"adding fields {list(elem.attrib.keys())}")

        class elemtype(graphene.ObjectType):
            class Meta:
                name = elem.tag
                fields = {"hardcoded2": graphene.String()}
                # fields = {
                #     # TODO: try to find a type now?
                #     key: graphene.Field(graphene.String)
                #     for key in elem.attrib.keys()
                # }
            hardcoded = graphene.String()

        print(f"elemtype._meta = {elemtype._meta}")

        return elemtype
