use graphql_parser::parse_query;
use graphql_parser::query::{Definition, OperationDefinition};

fn main() {
    // let doc = roxmltree::Document::parse(std::include_str!("example.xml")).unwrap();
    // let xml_root = doc.root_element();
    let json_root = &serde_json::from_str(std::include_str!("example.json")).unwrap();

    // let ast = parse_query("query { books { author } }").unwrap();
    let ast = parse_query("query { top_level, not_a_field}").unwrap();
    if ast.definitions.len() != 1 {
        panic!("Must have exactly 1 definition");
    }
    let query_root = &ast.definitions[0];
    match query_root {
        Definition::Operation(OperationDefinition::Query(q)) => {
            let res = xq::eval(q, json_root);
            println!("{}", res.to_string())
        }
        _ => {
            panic!("Unsupported root: {:?}", query_root);
        }
    }
}
