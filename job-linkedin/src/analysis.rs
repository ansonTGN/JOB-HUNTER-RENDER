use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{digit1, multispace0},
    combinator::map,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use crate::domain::EmploymentType;

#[derive(Debug, Default)]
pub struct ExtractedMetadata {
    pub salary: Option<String>,
    pub contract: EmploymentType,
    pub is_remote: bool,
    pub days_ago: u64,
}

pub fn analyze_job_text(text: &str) -> ExtractedMetadata {
    let mut meta = ExtractedMetadata::default();
    
    let mut cursor = text;
    while !cursor.is_empty() {
        if let Ok((_, salary)) = parse_salary_string(cursor) {
            if !salary.contains("202") {
                meta.salary = Some(salary);
                break; 
            }
        }
        let next_char_len = cursor.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
        cursor = &cursor[next_char_len..];
    }

    let lower_text = text.to_lowercase();
    
    if lower_text.contains("remoto") || lower_text.contains("remote") || lower_text.contains("teletrabajo") || lower_text.contains("100%") {
        meta.is_remote = true;
    }

    if lower_text.contains("indefinido") || lower_text.contains("permanent") || lower_text.contains("fijo") {
        meta.contract = EmploymentType::FullTime;
    } else if lower_text.contains("temporal") || lower_text.contains("project") {
        meta.contract = EmploymentType::Temporary;
    } else if lower_text.contains("freelance") || lower_text.contains("autónomo") || lower_text.contains("contractor") {
        meta.contract = EmploymentType::Contractor;
    } else if lower_text.contains("beca") || lower_text.contains("internship") || lower_text.contains("prácticas") {
        meta.contract = EmploymentType::Intern;
    }

    if let Some(idx) = lower_text.find("hace") {
        let slice = &lower_text[idx..];
        let parts: Vec<&str> = slice.split_whitespace().collect();
        if parts.len() >= 3 {
             if let Ok(num) = parts[1].parse::<u64>() {
                 if parts[2].contains("días") || parts[2].contains("dia") {
                     meta.days_ago = num;
                 }
             }
        }
    }
    
    meta
}

fn parse_amount(input: &str) -> IResult<&str, String> {
    let number_k = map(pair(digit1, tag_no_case("k")), |(n, _)| format!("{}000", n));
    let number_dot = map(tuple((digit1, alt((tag("."), tag(","))), digit1)), |(a, _, b)| format!("{}{}", a, b));
    let plain = map(digit1, |s: &str| s.to_string());
    alt((number_k, number_dot, plain))(input)
}

fn parse_currency(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, alt((tag_no_case("€"), tag_no_case("eur"), tag_no_case("euros"), tag_no_case("$"), tag_no_case("usd"))))(input)
}

fn parse_salary_string(input: &str) -> IResult<&str, String> {
    let range = map(tuple((parse_amount, delimited(multispace0, alt((tag("-"), tag_no_case("a"))), multispace0), parse_amount, parse_currency)),
        |(min, _, max, cur)| format!("{} - {} {}", min, max, cur.to_uppercase()));
    let single = map(pair(parse_amount, parse_currency), |(val, cur)| format!("{} {}", val, cur.to_uppercase()));
    alt((range, single))(input)
}