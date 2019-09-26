#![allow(
clippy::missing_docs_in_private_items,
clippy::missing_inline_in_public_items,
clippy::implicit_return,
clippy::result_unwrap_used,
clippy::option_unwrap_used,
clippy::print_stdout,
clippy::use_debug,
)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pairs, Pair};
use crate::sqm::file::File;
use crate::sqm::array::Array;
use crate::sqm::class::Class;
use pest::Parser;

mod sqm;


#[derive(Parser)]
#[grammar = "sqm.pest"]
pub struct SQMParser;

pub fn deserialize_json(json_string: &str) -> File{
    serde_json::from_str(json_string).unwrap()
}

pub fn serialize_sqm_string(sqm_data: &str, pretty: bool) -> String{
    let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, sqm_data).unwrap();
    let parsed_file = parse_file(parsed);
    serialize_pairs(&parsed_file, pretty)
}

pub fn serialize_pairs(filedata: &File, pretty: bool) -> String{
    if pretty {
        serde_json::to_string_pretty(&filedata).unwrap()
    }else {
        serde_json::to_string(&filedata).unwrap()
    }
}

pub fn serialize_to_sqm(filedata: &File, filename: &str){
    let file = std::fs::File::create(filename).unwrap();

    filedata.walk(&file);
}

pub fn parse_file(filedata: Pairs<Rule>) -> File{
    let file = filedata.enumerate().nth(0).unwrap().1.into_inner();


    let mut file_strc: File = File{
        items: Default::default(),
        arrays: Default::default(),
        classes: Default::default()
    };

    for top_level_entries in file {
        match top_level_entries.as_rule() {
            Rule::item => {
                let items = parse_item(top_level_entries);
                file_strc.items.insert(items.0, items.1);
            }
            Rule::class => {
                let items = parse_class(top_level_entries);
                file_strc.classes.insert(items.0, items.1);
            }
            Rule::array => {
                let items = parse_array(top_level_entries);
                file_strc.arrays.insert(items.0, items.1);
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::key | Rule::value | Rule::file => unreachable!()
        }
    }
    file_strc
}

pub fn parse_class(item: Pair<Rule>) -> (String, Class) {
    let inner = item.into_inner();
    let mut retclass: Class = Class{
        items: Default::default(),
        arrays: Default::default(),
        classes: Default::default()
    };

    let mut key: String = "".to_string();
    for items in inner {
        match items.as_rule() {
            Rule::key => {
                key = get_string_from_pair(&items)
            }
            Rule::item => {
                let items = parse_item(items);
                retclass.items.insert(items.0, items.1);
            }
            Rule::class => {
                let items = parse_class(items);
                retclass.classes.insert(items.0, items.1);
            }
            Rule::array => {
                let items = parse_array(items);
                retclass.arrays.insert(items.0, items.1);
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::value | Rule::file => unreachable!()
        }
    }

    //println!("{:#?}", retclass);

    (key, retclass)
}

pub fn parse_item(item: Pair<Rule>) -> (String, String){
    let mut inner = item.into_inner();
    let key = get_string_from_pair(&inner.next().unwrap());
    let value = get_string_from_pair(&inner.next().unwrap());
    (key, value)
}

pub fn parse_array(item: Pair<Rule>) -> (String, Array){
    let inner = item.into_inner();

    let mut retarray: Array = Array{ values: vec![] };
    let mut key: String = "".to_string();

    for x in inner {
        match x.as_rule() {
            Rule::key => {
                key =  get_string_from_pair(&x);
            }
            Rule::value => {
                retarray.values.push(get_string_from_pair(&x));
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::item | Rule::array | Rule::class | Rule::file => unreachable!()
        }
    }
    (key, retarray)
}

fn get_string_from_pair(pair: &Pair<Rule>) -> String{
    String::from(pair.as_span().as_str())
}