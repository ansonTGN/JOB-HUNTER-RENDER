// src/domain.rs
use serde::Serialize;

// Estándar Schema.org/EmploymentType
#[derive(Debug, Clone, PartialEq, Serialize, Default)]
pub enum EmploymentType {
    #[serde(rename = "FULL_TIME")] FullTime,
    #[allow(dead_code)] #[serde(rename = "PART_TIME")] PartTime,
    #[serde(rename = "CONTRACTOR")] Contractor,
    #[serde(rename = "TEMPORARY")] Temporary,
    #[serde(rename = "INTERN")] Intern,
    #[allow(dead_code)] #[serde(rename = "FREELANCE")] Freelance,
    #[default]
    #[serde(rename = "OTHER")] Unknown,
}

impl ToString for EmploymentType {
    fn to_string(&self) -> String {
        match self {
            EmploymentType::FullTime => "Jornada Completa".to_string(),
            EmploymentType::PartTime => "Media Jornada".to_string(),
            EmploymentType::Contractor => "Contrato / Autónomo".to_string(),
            EmploymentType::Temporary => "Temporal".to_string(),
            EmploymentType::Intern => "Becario".to_string(),
            EmploymentType::Freelance => "Freelance".to_string(),
            EmploymentType::Unknown => "No especificado".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum JobSource {
    GoogleLinkedIn, GoogleInfoJobs, GoogleIndeed, GoogleGlassdoor,
    GoogleManfred, GoogleTicjob, GoogleStackOverflow,
    GoogleWeWorkRemotely, GoogleRemoteOK, GoogleGeneral,
    #[allow(dead_code)] DirectLinkedIn,
    #[allow(dead_code)] DirectInfoJobs,
    #[allow(dead_code)] DirectIndeed,
}

impl ToString for JobSource {
    fn to_string(&self) -> String {
        match self {
            JobSource::GoogleLinkedIn => "LinkedIn".to_string(),
            JobSource::GoogleInfoJobs => "InfoJobs".to_string(),
            JobSource::GoogleIndeed => "Indeed".to_string(),
            JobSource::GoogleGlassdoor => "Glassdoor".to_string(),
            JobSource::GoogleManfred => "Manfred".to_string(),
            JobSource::GoogleTicjob => "TicJob".to_string(),
            JobSource::GoogleStackOverflow => "StackOverflow".to_string(),
            JobSource::GoogleWeWorkRemotely => "WeWorkRemotely".to_string(),
            JobSource::GoogleRemoteOK => "RemoteOK".to_string(),
            JobSource::GoogleGeneral => "Google Search".to_string(),
            _ => "Directo".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Job {
    #[serde(rename = "jobTitle")]
    pub title: String,
    #[serde(rename = "hiringOrganization")]
    pub company: String,
    #[serde(rename = "jobLocation")]
    pub location: String,
    #[serde(rename = "baseSalary")]
    pub salary: Option<String>,
    #[serde(rename = "employmentType")]
    pub contract_type: EmploymentType,
    pub url: String,
    #[serde(skip)]
    pub source: JobSource,
    #[serde(rename = "datePosted")]
    pub date_posted_iso: String,
    #[serde(rename = "description")]
    pub description_snippet: String,
    #[serde(rename = "jobLocationType")]
    pub location_type: Option<String>,
}

impl Job {
    pub fn new(title: String, company: String, url: String, source: JobSource) -> Self {
        use chrono::Utc;
        Self {
            title, company, url, source,
            location: "España".to_string(),
            salary: None,
            contract_type: EmploymentType::Unknown,
            date_posted_iso: Utc::now().format("%Y-%m-%d").to_string(),
            description_snippet: String::new(),
            location_type: None,
        }
    }
}