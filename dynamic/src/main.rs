use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use regex::Regex;

fn main() -> Result<(), Error> {
    let mut parameters = HashMap::new();
    parameters.insert(format!("a"), format!("b"));
    let result = render("test.txt", &parameters)?;
    println!("{}", result);

    Ok(())
}


fn render(template_name: &'static str, parameters: &HashMap<String, String>) -> Result<String, Error> {
    // loading file content
    let mut result = File::open(template_name)?;
    let mut content = String::new();
    result.read_to_string(&mut content)?;

    // template engine
    let parameter_pattern = Regex::new(r"<(?P<key>[a-zA-Z_]+)>").unwrap();
    parameter_pattern.replace_all(&content, | cap: &regex::Captures |{
        if let Some(param) = cap.name("key") {
            dbg!(parameters);
            dbg!(parameters.get(param.as_str()).unwrap_or(&param.as_str().to_string()).as_str().to_string())
        }else {
            format!("")
        }
    });
//    parameter_pattern.replace_all(&content, "123");

    Ok(dbg!(content))
}