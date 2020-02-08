use graphql_parser::query::Query;
use graphql_parser::query::{Definition, OperationDefinition};
use serde_json::json;

use xq::eval;

fn parse_query(q: &str) -> Query {
    let ast = graphql_parser::parse_query(q).unwrap();

    let query_root = &ast.definitions[0];

    match query_root {
        Definition::Operation(OperationDefinition::Query(q)) => {
            return q.clone();
        }
        _ => {
            panic!("Unsupported root: {:?}", query_root);
        }
    }
}

#[test]
fn eval_missing_key() {
    let src = json!({});
    let query = parse_query("query { no_such_key }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({ "no_such_key": null }));
}

#[test]
fn single_top_level_key() {
    let src = json!({"top_level": "hello"});
    let query = parse_query("query { top_level }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"top_level": "hello"}));
}

#[test]
fn multiple_top_level_keys() {
    let src = json!({"top_level": "hello", "secondary": "world"});
    let query = parse_query("query { top_level, secondary }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"top_level": "hello", "secondary": "world"}));
}

#[test]
fn integer_value_key() {
    let src = json!({"top_level": 42});
    let query = parse_query("query { top_level }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"top_level": 42}));
}

#[test]
fn nested_key() {
    let src = json!({"top_level": {"inner": "Hello world", "ignore": "Don't look at me"}});
    let query = parse_query("query { top_level { inner } }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"top_level": {"inner": "Hello world"}}));
}

#[test]
fn aliased_field() {
    let src = json!({"top_level": "hello"});
    let query = parse_query("query { alias_name: top_level }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"alias_name": "hello"}));
}

#[test]
#[should_panic]
fn field_with_arguments_crashes() {
    let src = json!({"top_level": {"inner": "Hello world", "ignore": "Don't look at me"}});
    let query = parse_query("query { top_level(with_arg: true) { inner } }");
    let res = eval(&query, &src);
    assert_eq!(res, json!({"top_level": {"inner": "Hello world"}}));
}
