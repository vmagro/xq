use roxmltree::{Document};
use serde_json::json;

use xq::xml_to_json;

#[test]
fn single_text_element() {
    let doc = Document::parse("<hello>world</hello>").unwrap();
    let res = xml_to_json(doc.root_element());
    assert_eq!(res, json!("world"));
}

#[test]
fn with_attrs() {
    let doc = Document::parse("<foo baz=\"1\">bar</foo>").unwrap();
    let res = xml_to_json(doc.root_element());
    assert_eq!(res, json!({"@baz": "1", "text": "bar"}));
}

#[test]
fn elem_with_two_children() {
    let doc = Document::parse("<foo><bar>Hello</bar><baz>World</baz></foo>").unwrap();
    let res = xml_to_json(doc.root_element());
    assert_eq!(res, json!({"bar": "Hello", "baz": "World"}));
}

#[test]
fn elem_with_repeated_children() {
    let doc = Document::parse("<r><hero><name>Luke Skywalker</name><friend>Han Solo</friend><friend>Leia Organa</friend></hero></r>").unwrap();
    let res = xml_to_json(doc.root_element());
    assert_eq!(res, json!({"hero": {"name": "Luke Skywalker", "friends": ["Han Solo", "Leia Organa"]}}));
}