use regex::Regex;

pub fn mail_extractor(text: &str) {
    let email_regx = Regex::new(r"[A-Za-z-0-9]+@[A-Za-z-0-9]+[a-z-+]{2,4}").unwrap();
    let emails_found:Vec<&str> = email_regx.find_iter(text).map( |found| found.as_str()).collect();
    println!("{:#?}", emails_found); 

}