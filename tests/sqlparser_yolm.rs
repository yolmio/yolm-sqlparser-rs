use sqlparser::dialect::GenericDialect;
use test_utils::*;

#[macro_use]
mod test_utils;

fn dialect() -> TestedDialects {
    TestedDialects {
        dialects: vec![Box::new(GenericDialect {})],
        options: None,
    }
}

#[test]
fn parse_auto_join() {
    dialect().one_statement_parses_to("select * from a auto join b", "SELECT * FROM a AUTO JOIN b");
}
