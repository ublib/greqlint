use std::fmt::format;

use crate::{
    // normalizer::Issue,
    parser,
    regex_generator::gen_regex,
};

use super::super::parser::ast::Root;
use greqlint_schema::schema::{CommitType, Schema};

use regex::Captures;

struct Variables<'a> {
    pub types: &'a Vec<CommitType>,
    pub scopes: &'a Vec<String>,
    // pub issues: &'a Vec<Issue>,
    // pub mrs: Vec<MR>,
}

pub fn lint_title(
    schema: &Schema,
    // TODO: issues
    // issues: &Vec<Issue>,
    title: &str,
) -> Result<(), String> {
    let format = match &schema.title_format {
        Some(format) => format,
        None => return Ok(()),
    };

    let default_scopes = vec![];
    let scopes = match schema.common_scopes {
        Some(ref scopes) => scopes,
        None => &default_scopes,
    };
    let vars = Variables {
        types: &schema.types,
        scopes,
        // issues,
    };
    let ast = parser::parse(format)?;
    let cap = capture(ast, title)?;
    validate_capture(cap, vars)?;
    Ok(())
}

fn capture<'a>(ast: Root<'a>, title: &'a str) -> Result<Captures<'a>, String> {
    let re = gen_regex(&ast);
    let Some(cap) = re.captures(title) else {
        return Err("Invalid title regexp".to_string());
    };
    Ok(cap)
}

fn validate_capture(cap: Captures<'_>, vars: Variables) -> Result<(), String> {
    let type_name = cap.name("type");
    let scope_name = cap.name("scope");
    // TODO: issues
    // let issue_num = cap.name("issue");
    // let mr_num = cap.name("mr");

    if let Some(type_name) = type_name {
        if !vars.types.iter().any(|t| t.type_name == type_name.as_str()) {
            return Err(format!(
                "\"type\" must be one of {:?}",
                vars.types
                    .iter()
                    .map(|t| t.type_name.as_str())
                    .collect::<Vec<&str>>()
            ));
        }
    }

    if let Some(scope_name) = scope_name {
        let n = scope_name.as_str().to_string();
        if !vars.scopes.contains(&n) {
            return Err(format!(
                "\"scope\" must be one of {:?}",
                vars.scopes
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>()
            ));
        }
        // TODO: check type.scope
    }

    // TODO: issues
    // if let Some(issue_num) = issue_num {
    //     let n: u32 = issue_num.as_str().parse().unwrap();
    //     if !vars.issues.iter().any(|i| i.number == n) {
    //         return Err("Invalid issue".to_string());
    //     }
    // }

    // if let Some(mr_num) = mr_num {
    //     let n: u32 = mr_num.as_str().parse().unwrap();
    //     if !vars.issues.iter().any(|i| i.number == n) {
    //         return Err("Invalid MR".to_string());
    //     }
    // }

    Ok(())
}

#[cfg(test)]
mod tests {
    use greqlint_schema::schema::Provider;

    use super::*;

    #[test]
    fn test_lint_title() {
        let schema = Schema {
            types: vec![CommitType {
                type_name: "feat".to_string(),
                scopes: Some(vec!["scope".to_string()]),
            }],
            common_scopes: Some(vec!["scope".to_string()]),
            provider: Provider::GitHubActions,
            title_format: Some("${type}(${scope}): .+".to_string()),
            branch_name_format: None,
        };

        assert_eq!(lint_title(&schema, "feat(scope): message"), Ok(()));
        assert_ne!(lint_title(&schema, "aaa(scope): message message"), Ok(()));
    }
}
