#[macro_use] extern crate lalrpop_util;
mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    lalrpop_mod!(pub calculator1); // synthesized by LALRPOP
    
    #[test]
    fn calculator1() {
        assert!(calculator1::ExprParser::new().parse("22").is_ok());
        assert!(calculator1::ExprParser::new().parse("(22)").is_ok());
        assert!(calculator1::ExprParser::new().parse("((((22))))").is_ok());
        assert!(calculator1::ExprParser::new().parse("((22)").is_err());
        assert_eq!(calculator1::ExprParser::new().parse("((22))").unwrap(), 22);
        
    
        let expr = calculator1::ExprParser::new()
            .parse("22 * 44 + 66")
            .unwrap();
        assert_eq!(expr, ((22 * 44) + 66));
         
        let expr = calculator1::ExprParser::new()
            .parse("22 * (44 + 66)")
            .unwrap();
        assert_eq!(expr, (22 * ( 44 + 66)));
        
    }


    lalrpop_mod!(pub calculator4);
    #[test]
    fn calculator4() {
        let expr = calculator4::ExprParser::new()
            .parse("22 * 44 + 66")
            .unwrap();
        assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
    }
}
