use graphql_parser::parse_query;
use graphql_parser::query::{Definition, OperationDefinition};
use std::io::Read;

fn main() {
    // let json_root = &serde_json::from_reader(std::io::stdin()).unwrap();
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).unwrap();
    let doc = roxmltree::Document::parse(&buffer).unwrap();
    let root = doc.root_element();
    println!("{:?}", root);

    // let ast = parse_query("query { top_level, books { author }}").unwrap();
    // if ast.definitions.len() != 1 {
    //     panic!("Must have exactly 1 definition");
    // }
    // let query_root = &ast.definitions[0];
    // match query_root {
    //     Definition::Operation(OperationDefinition::Query(q)) => {
    //         let res = xq::eval(q, json_root);
    //         println!("{}", serde_json::to_string_pretty(&res).unwrap());
    //     }
    //     _ => {
    //         panic!("Unsupported root: {:?}", query_root);
    //     }
    // }
}
