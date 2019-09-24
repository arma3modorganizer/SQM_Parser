extern crate pest;
#[macro_use]
extern crate pest_derive;


#[cfg(test)]
mod tests {
    use pest::Parser;
    use pest::iterators::Pairs;

    #[derive(Parser)]
    #[grammar = "sqm.pest"]
    pub struct SQMParser;

    #[test]
    fn test_array(){
        let parsed: Result<Pairs<Rule>, pest::error::Error<Rule>> = SQMParser::parse(Rule::array, "pos[]={A,B,C,D};");
        let result_vec = vec!["pos", "A", "B", "C", "D"];
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
        assert_eq!(result_vec, actual_vec);
    }
}
