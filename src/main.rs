#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use rusoto_core::default_tls_client;


use std::default::Default;

fn list() {
    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbClient::new(default_tls_client().unwrap(), provider, Region::UsEast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(&list_tables_input) {
      Ok(output) => {
        match output.table_names {
          Some(table_name_list) => {
            println!("Tables in database:");

            for table_name in table_name_list {
              println!("{}", table_name);
            }
          }
          None => println!("No tables in database!"),
        }
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }
}

#[get("/")]
fn index() -> &'static str {
    list();
    "Hello, world!"
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
