#[macro_use] extern crate lalrpop_util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    lalrpop_mod!(pub grammar); // synthesized by LALRPOP
    
    #[test]
    fn calculator1() {
        assert!(grammar::TermParser::new().parse("22").is_ok());
        assert!(grammar::TermParser::new().parse("(22)").is_ok());
        assert!(grammar::TermParser::new().parse("((((22))))").is_ok());
        assert!(grammar::TermParser::new().parse("((22)").is_err());
    }
    
}
