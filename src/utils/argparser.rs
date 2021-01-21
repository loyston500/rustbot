use regex::Regex;
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

/// an argument parser written in rust for rust
pub fn argparse(s: String) -> Result<(HashMap<String, String>, Vec<String>, HashSet<String>), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[^\\s\"]+|\"[^\"]*\"").unwrap();
    }
    let mes:Vec<String> = RE.find_iter(&s.as_str()).filter_map(|this| Some(this.as_str().to_string())).collect();
    let length:usize = mes.len();
    let mut i:usize = 0;
    let mut params = HashMap::new();
    let mut inputs:Vec<String> = Vec::new();
    let mut flags = HashSet::new();
    while i < length {
        if mes[i].starts_with("--") {
            flags.insert((&mes[i][2..]).to_string());
        }
        else if mes[i].starts_with("-") {
            params.insert((&mes[i][1..]).to_string(), (&mes[i + 1].replace("\"", "")).to_string());
            i += 1;
        }
        else if mes[i].starts_with("\"") && mes[i].ends_with("\"") {
            inputs.push((&mes[i][1..(mes[i].len()-1)]).to_string());
        }
        else {
            inputs.push((&mes[i]).to_string());
        }
        i += 1;
    }
    Ok((params, inputs, flags))
}

