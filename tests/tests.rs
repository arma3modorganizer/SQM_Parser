#[cfg(test)]
mod tests {

    #![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::implicit_return,
    clippy::result_unwrap_used,
    clippy::option_unwrap_used,
    clippy::print_stdout,
    clippy::use_debug,
    clippy::integer_arithmetic,
    clippy::default_trait_access,
    clippy::never_loop,
    )]

    use pest::Parser;
    use pest::iterators::Pairs;
    use sqm_parser::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_item(){
        let test_arrays = vec![
            "binarizationWanted=0;",
            "version=53;"
        ];

        let result_arrays = vec![
            vec!["binarizationWanted", "0"],
            vec!["version", "53"],
        ];

        for (i, test_entry) in test_arrays.iter().enumerate() {
            let parsed: Result<Pairs<Rule>, pest::error::Error<Rule>> = SQMParser::parse(Rule::item, test_entry);
            let mut actual_vec: Vec<&str> = vec![];
            match parsed {
                Ok(v) => {
                    for x in v {
                        let i = x.into_inner();
                        for x2 in i {
                            actual_vec.push(x2.as_span().as_str())
                        }
                    }
                }
                Err(e) => {
                    panic!(e)
                }
            }
            assert_eq!(result_arrays.get(i).expect("Couldn't get expected result"), &actual_vec);
        }
    }

    #[test]
    fn test_array(){
        let test_arrays = vec![
            "pos[]={A,B,C,D};",
            "aside[]={0.77140951,-2.8638169e-008,0.63634229};",
            "addons[]=\
            {\
                \"keko_common\",\
                \"A3_Modules_F_Curator_Curator\",\
                \"A3_Characters_F\"\
            };"
        ];

        let result_arrays = vec![
            vec!["pos", "A", "B", "C", "D"],
            vec!["aside", "0.77140951", "-2.8638169e-008", "0.63634229",],
            vec!["addons", "\"keko_common\"", "\"A3_Modules_F_Curator_Curator\"", "\"A3_Characters_F\"",],
        ];

        for (i, test_entry) in test_arrays.iter().enumerate() {
            let parsed: Result<Pairs<Rule>, pest::error::Error<Rule>> = SQMParser::parse(Rule::array, test_entry);
            let mut actual_vec: Vec<&str> = vec![];
            match parsed {
                Ok(v) => {
                    for x in v {
                        let i = x.into_inner();
                        for x2 in i {
                            actual_vec.push(x2.as_span().as_str())
                        }
                    }
                }
                Err(e) => {
                    panic!(e)
                }
            }
            assert_eq!(result_arrays.get(i).expect("Couldn't get expected result"), &actual_vec);
        }
    }

    #[test]
    fn test_mission(){
        let mission_file = include_str!("mission.sqm");
        let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, mission_file).unwrap();

        for p in parsed {
            for p2 in p.into_inner() {
                let mut p3 = p2.into_inner();

                let version = p3.next();
                let vnumber = p3.next();

                assert_eq!(version.unwrap().as_span().as_str(), "version");
                assert_eq!(vnumber.unwrap().as_span().as_str(), "53");


                break
            }
        }
    }

    #[test]
    fn test_parser(){
        let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, include_str!("mission.sqm")).unwrap();
        let parsed_file = parse_file(parsed);

        assert_eq!(parsed_file.items.get("version").expect("Couldn't get version"), "53");

        let json = serialize_pairs(&parsed_file, true);
        let mut file = File::create("mission.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    #[test]
    fn test_deserializer(){
        let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, include_str!("mission.sqm")).unwrap();
        let parsed_file = parse_file(parsed);
        let json_data = serialize_pairs(&parsed_file, false);

        let deserialized_file = deserialize_json(&json_data);

        assert_eq!(parsed_file.items.len(), deserialized_file.items.len());
        assert_eq!(parsed_file.classes.len(), deserialized_file.classes.len());
        assert_eq!(parsed_file.arrays.len(), deserialized_file.arrays.len());
    }

    #[test]
    fn attempt_sqm_construction(){
        let mission_data = include_str!("mission.sqm");
        let parsed: Pairs<Rule> = SQMParser::parse(Rule::file, mission_data).unwrap();
        let parsed_file = parse_file(parsed);

        serialize_to_sqm(&parsed_file, "test.sqm");



    }
}
