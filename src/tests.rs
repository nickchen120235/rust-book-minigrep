#[cfg(test)]
use super::*;

#[test]
fn one_result() {
    let query = "safe";
    let content = "\
safe, fast
wtf
";
    assert_eq!(vec!["safe, fast"], search(query, content));
}

#[test]
fn case_insentive() {
    let query = "WTF";
    let content = "\
safe, fast
wtf
";
    assert_eq!(vec!["wtf"], search_case_insentive(query, content));
}