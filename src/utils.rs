use regex::Regex;
use roxmltree::Node;

pub fn get_children(i: Node) -> (String, String) {
    let mut name = String::new();
    let mut version = String::new();
    for ele in i.descendants() {
        if ele.tag_name().name().to_lowercase() == "version" {
            version = ele.text().unwrap().to_string();
            continue;
        }
        if ele.tag_name().name().to_lowercase() == "artifactid" {
            name = ele.text().unwrap().to_string();
            continue;
        }

        if !name.is_empty() & !version.is_empty() {
            break;
        }
    }

    return (name, version);
}

pub fn get_bundle_name(content: &str) -> String {
    let mut name = "";
    let re = Regex::new("Bundle-Name:.*\\n");
    match re {
        Ok(re) => {
            let found = re.find(content);
            match found {
                Some(bundle) => {
                    let line = bundle.as_str().replace("\r", "").replace("\n", "");
                    let splist: Vec<&str> = line.as_str().trim().split(":").collect();
                    if splist.len() > 1 {
                        name = splist[1].trim();
                    }
                    return name.to_string();
                }
                None => return name.to_string(),
            }
        }
        Err(_) => return name.to_string(),
    }
}
pub fn get_bundle_version(content: &str) -> String {
    let mut version = "";
    let re = Regex::new("Bundle-Version:.*\\n");
    match re {
        Ok(re) => {
            let found = re.find(content);
            match found {
                Some(bundle) => {
                    let line = bundle.as_str().replace("\r", "").replace("\n", "");
                    let splist: Vec<&str> = line.as_str().trim().split(":").collect();
                    if splist.len() > 1 {
                        version = splist[1].trim();
                    }
                    return version.to_string();
                }
                None => return version.to_string(),
            }
        }
        Err(_) => return version.to_string(),
    }
}

pub fn get_implementation_title(content: &str) -> String {
    let mut name = "";
    let re = Regex::new("Implementation-Title:.*\\n");
    match re {
        Ok(re) => {
            let found = re.find(content);
            match found {
                Some(bundle) => {
                    let line = bundle.as_str().replace("\r", "").replace("\n", "");
                    let splist: Vec<&str> = line.as_str().trim().split(":").collect();
                    if splist.len() > 1 {
                        name = splist[1].trim();
                    }
                    return name.to_string();
                }
                None => return name.to_string(),
            }
        }
        Err(_) => return name.to_string(),
    }
}
pub fn get_implementation_version(content: &str) -> String {
    let mut version = "";
    let re = Regex::new("Implementation-Version:.*\\n");
    match re {
        Ok(re) => {
            let found = re.find(content);
            match found {
                Some(bundle) => {
                    let line = bundle.as_str().replace("\r", "").replace("\n", "");
                    let splist: Vec<&str> = line.as_str().trim().split(":").collect();
                    if splist.len() > 1 {
                        version = splist[1].trim();
                    }
                    return version.to_string();
                }
                None => return version.to_string(),
            }
        }
        Err(_) => return version.to_string(),
    }
}
