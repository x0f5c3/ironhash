use prototypes::PROTOTYPES;
use rayon::prelude::*;
use regex::Regex;
use std::fmt;
pub mod prototypes;
#[derive(Debug, Clone)]
pub struct Prototype {
    pub regex: Regex,
    pub modes: HashInfo,
}

#[derive(Debug, Clone)]
pub struct Mode {
    pub hashcat: Option<u64>,
    pub john: Option<String>,
    pub extended: bool,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct HashInfo {
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone)]
pub struct HashResult {
    pub hash: String,
    pub modes: Option<HashInfo>,
}

impl HashResult {
    pub async fn new(hash: String) -> Self {
        let hash = hash.trim().to_string();
        let res = PROTOTYPES
            .par_iter()
            .filter_map(|x| {
                if x.regex.is_match(&hash) {
                    Some(x.modes.modes.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<Mode>>>();
        let mut result = Vec::new();
        for x in res {
            for i in x {
                result.push(i);
            }
        }
        let optional = if result.is_empty() {
            None
        } else {
            Some(HashInfo { modes: result })
        };
        HashResult {
            hash,
            modes: optional,
        }
    }
}
impl fmt::Display for HashInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut hashtypes = String::new();
        for mode in &self.modes {
            if !mode.extended {
                let formatted = format!("[+] {} ", mode.name);
                hashtypes.push_str(&formatted);
                if let Some(cat) = &mode.hashcat {
                    hashtypes.push_str(&format!("[Hashcat Mode: {}] ", cat));
                }
                if let Some(john) = &mode.john {
                    hashtypes.push_str(&format!("[JtR Format: {}]", &john));
                }
                hashtypes.push('\n');
            }
        }
        write!(f, "{}", hashtypes)
    }
}

impl fmt::Display for HashResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(modes) = &self.modes {
            write!(f, "Hash: {}\nMatches:\n{}", self.hash, modes)
        } else {
            write!(f, "Hash: {}\nNo match found!!!!", self.hash)
        }
    }
}
