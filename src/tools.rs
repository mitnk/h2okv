use regex::Regex;

pub fn re_contains(ptn: &str, text: &str) -> bool {
    let re;
    match Regex::new(ptn) {
        Ok(x) => {
            re = x;
        }
        Err(e) => {
            println!("Regex new failed: {:?}", e);
            return false;
        }
    }
    re.is_match(text)
}
