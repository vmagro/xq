use graphql_parser::parse_query;

fn main() {
    println!("Hello, world!");
    let ast = parse_query("query { field1, field2 }").unwrap();
    if ast.definitions.len() != 1 {
        panic!("Must have exactly 1 definition");
    }
    println!("{:?}", ast.definitions[0]);
}
