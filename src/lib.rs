#![allow(
clippy::missing_docs_in_private_items,
clippy::missing_inline_in_public_items,
clippy::implicit_return,
clippy::result_unwrap_used,
clippy::option_unwrap_used,
)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pairs, Pair};
use crate::sqm::item::Item;
use crate::sqm::file::File;
use crate::sqm::array::Array;
use crate::sqm::class::Class;
use pest::Parser;

mod sqm;


#[derive(Parser)]
#[grammar = "sqm.pest"]
pub struct SQMParser;

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


pub fn parse_file(filedata: Pairs<Rule>) -> File{
    let file = filedata.enumerate().nth(0).unwrap().1.into_inner();


    let mut file_strc: File = File{
        items: vec![],
        arrays: vec![],
        classes: vec![]
    };

    for top_level_entries in file {
        match top_level_entries.as_rule() {
            Rule::item => {
                 file_strc.items.push(parse_item(top_level_entries));
            }
            Rule::class => {
                file_strc.classes.push(parse_class(top_level_entries));
            }
            Rule::array => {
                file_strc.arrays.push(parse_array(top_level_entries));
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::key | Rule::value | Rule::file => unreachable!()
        }
    }
    file_strc
}

pub fn parse_class(item: Pair<Rule>) -> Class{
    let inner = item.into_inner();
    let mut retclass: Class = Class{
        key: "".to_string(),
        items: vec![],
        arrays: vec![],
        classes: vec![]
    };

    for items in inner {
        match items.as_rule() {
            Rule::key => {
                retclass.key = get_string_from_pair(&items)
            }
            Rule::item => {
                retclass.items.push(parse_item(items));
            }
            Rule::class => {
                retclass.classes.push(parse_class(items));
            }
            Rule::array => {
                retclass.arrays.push(parse_array(items));
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::value | Rule::file => unreachable!()
        }
    }

    //println!("{:#?}", retclass);

    retclass
}

pub fn parse_item(item: Pair<Rule>) -> Item{
    let mut inner = item.into_inner();
    let key = get_string_from_pair(&inner.next().unwrap());
    let value = get_string_from_pair(&inner.next().unwrap());
    Item{
        key,
        value
    }
}

pub fn parse_array(item: Pair<Rule>) -> Array{
    let inner = item.into_inner();

    let mut retarray: Array = Array{ key: "".to_string(), values: vec![] };

    for x in inner {
        match x.as_rule() {
            Rule::key => {
                retarray.key =  get_string_from_pair(&x);
            }
            Rule::value => {
                retarray.values.push(get_string_from_pair(&x));
            }
            Rule::WHITESPACE | Rule::char | Rule::string | Rule::strict_string | Rule::number | Rule::item | Rule::array | Rule::class | Rule::file => unreachable!()
        }
    }
    retarray
}

fn get_string_from_pair(pair: &Pair<Rule>) -> String{
    String::from(pair.as_span().as_str())
}