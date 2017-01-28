#![cfg(feature = "ses")]

extern crate rusoto;

use rusoto::ses::;
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_something() {
    println!("{:#?}", 1);
}
