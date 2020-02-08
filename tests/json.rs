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
    let res = eval(&query, src);
    assert!(res.is_object());
    let obj = res.as_object().unwrap();
    assert!(obj.contains_key("no_such_key"));
    assert_eq!(obj.get("no_such_key").unwrap(), &json!(null));
}
