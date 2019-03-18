use regex::{Regex, Captures};
use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::collections::HashMap;

struct Template(String);

impl <'a>Template {

    fn new(content: &'a str) -> Self {
        Template(content.to_string())
    }

    fn load<P: AsRef<Path>>(path: P)->Result<Self, Error> {
        let mut result = File::open(path)?;
        let mut content = String::new();
        result.read_to_string(&mut content)?;
        Ok(Template(content))
    }

    fn render(&self, parameters: &HashMap<String, String>) -> String {
        let parameter_pattern = Regex::new(r"\{\{\s?(?P<key>[a-zA-Z_]+)\s?}}").unwrap();
        let result = parameter_pattern.replace_all(&self.0, |cap: &Captures| {
            if let Some(param) = cap.name("key") {
                parameters.get(param.as_str()).unwrap_or(&cap.get(0).unwrap().as_str().to_string()).as_str().to_string()
            } else {
                format!("")
            }
        });
        result.to_string()
    }
}


fn main()-> Result<(), Error> {
    let template = Template::load("test.txt")?;
    let mut map = HashMap::new();
    map.insert(format!("a"), format!("b"));
    let content = template.render(&map);
    println!("{}", content);
    Ok(())
}
