use crate::domain::{Job, JobSource};
use crate::browser::BrowserFactory;
use crate::analysis;
use thirtyfour::prelude::*;
use async_trait::async_trait;
use std::time::Duration;
use rand::Rng;
use url::Url;
use chrono::{Utc, Duration as ChronoDuration};

#[async_trait]
pub trait JobScraper: Send + Sync {
    async fn search(&self, query: &str, location: &str) -> anyhow::Result<Vec<Job>>;
}

async fn random_sleep(min_ms: u64, max_ms: u64) {
    let millis = {
        let mut rng = rand::thread_rng();
        rng.gen_range(min_ms..=max_ms)
    };
    tokio::time::sleep(Duration::from_millis(millis)).await;
}

pub struct GoogleScraper {
    target_site: String,
    source_type: JobSource,
}

impl GoogleScraper {
    pub fn new(target_site: &str, source_type: JobSource) -> Self {
        Self { target_site: target_site.to_string(), source_type }
    }
}

#[async_trait]
impl JobScraper for GoogleScraper {
    async fn search(&self, query: &str, location: &str) -> anyhow::Result<Vec<Job>> {
        let driver = BrowserFactory::create().await?;
        
        let search_term = if self.target_site.is_empty() {
            format!("{} {} empleos", query, location)
        } else {
            format!("site:{} \"{}\" \"{}\"", self.target_site, query, location)
        };
        
        let url = format!("https://www.google.com/search?q={}&hl=es&gl=es&num=20", urlencoding::encode(&search_term));
        println!("🚀 [Google X-Ray] URL: {}", url);
        driver.goto(&url).await?;
        random_sleep(2000, 4000).await;

        let consent_xpath = "//button//div[contains(text(), 'Aceptar todo')] | //button//div[contains(text(), 'Acepto')] | //*[@id='L2AGLb']";
        if let Ok(btn) = driver.find(By::XPath(consent_xpath)).await {
            let _ = btn.click().await;
            random_sleep(1500, 2500).await;
        }

        let mut jobs = Vec::new();
        let mut cards = driver.find_all(By::Css("div.g")).await?;
        if cards.is_empty() { cards = driver.find_all(By::Css("div.MjjYud")).await?; }

        println!("📦 [Google] Bloques detectados: {}", cards.len());

        for card in cards.iter() {
            // Título
            let title_elem = match card.find(By::Css("h3")).await {
                Ok(e) => e, Err(_) => continue,
            };
            let title_raw = title_elem.text().await.unwrap_or_default();
            if title_raw.trim().is_empty() { continue; }

            // Enlace
            let mut link_res = card.find(By::Css("a:has(h3)")).await;
            if link_res.is_err() { link_res = card.find(By::Css("a")).await; }
            let url = match link_res {
                Ok(e) => e.attr("href").await.unwrap_or_default().unwrap_or_default(),
                Err(_) => continue,
            };
            if url.contains("google.com") || url.is_empty() { continue; }

            // Snippets
            let snippet = match card.find(By::Css("div[style*='-webkit-line-clamp'], .VwiC3b, span.st")).await {
                Ok(el) => el.text().await.unwrap_or_default(), Err(_) => String::new()
            };
            let meta_line = match card.find(By::Css("span.LEwnzc, span.f")).await {
                Ok(el) => el.text().await.unwrap_or_default(), Err(_) => String::new()
            };

            // Parseo Título/Empresa
            let parts: Vec<&str> = title_raw.split(" - ").collect();
            let title = parts.get(0).unwrap_or(&title_raw.as_str()).trim().to_string();
            let company = if parts.len() > 1 { parts[1].trim().to_string() } else { 
                if let Ok(parsed) = Url::parse(&url) {
                    parsed.host_str().unwrap_or("Web").replace("www.", "").to_string()
                } else { "Vía Google".to_string() }
            };

            let mut job = Job::new(title, company, url, self.source_type.clone());
            job.description_snippet = snippet.clone();

            // ANÁLISIS SEMÁNTICO
            let full_text = format!("{} {} {}", title_raw, snippet, meta_line);
            let meta = analysis::analyze_job_text(&full_text);

            job.salary = meta.salary;
            job.contract_type = meta.contract;
            
            if meta.is_remote { 
                job.location_type = Some("TELECOMMUTE".to_string()); // Schema.org value
            }

            // Calculo de fecha ISO
            if meta.days_ago > 0 {
                let date = Utc::now() - ChronoDuration::days(meta.days_ago as i64);
                job.date_posted_iso = date.format("%Y-%m-%d").to_string();
            } else if full_text.to_lowercase().contains("hoy") || full_text.to_lowercase().contains("today") {
                 job.date_posted_iso = Utc::now().format("%Y-%m-%d").to_string();
            }

            if !snippet.is_empty() {
                 let clean_snip = if snippet.len() > 80 { format!("{}...", &snippet[0..80]) } else { snippet.clone() };
                 job.location = clean_snip; // Usamos snippet como location visual si no hay otra cosa
            } else {
                 job.location = location.to_string();
            }

            jobs.push(job);
        }

        // Fallback Nuclear (H3)
        if jobs.is_empty() {
             let titles = driver.find_all(By::Css("h3")).await?;
             for t in titles {
                 if let Ok(parent) = t.find(By::XPath("./parent::a")).await {
                     let url = parent.attr("href").await.unwrap_or_default().unwrap_or_default();
                     let txt = t.text().await.unwrap_or_default();
                     if !url.is_empty() && !url.contains("google.com") {
                         jobs.push(Job::new(txt, "Vía Google".to_string(), url, self.source_type.clone()));
                     }
                 }
             }
        }

        jobs.sort_by(|a, b| a.url.cmp(&b.url));
        jobs.dedup_by(|a, b| a.url == b.url);

        driver.quit().await?;
        Ok(jobs)
    }
}

// Scrapers Dummy
pub struct InfoJobsScraper;
#[async_trait]
impl JobScraper for InfoJobsScraper { async fn search(&self, _q: &str, _l: &str) -> anyhow::Result<Vec<Job>> { Ok(vec![]) } }
pub struct LinkedInScraper;
#[async_trait]
impl JobScraper for LinkedInScraper { async fn search(&self, _q: &str, _l: &str) -> anyhow::Result<Vec<Job>> { Ok(vec![]) } }

pub struct ScraperStrategy;
impl ScraperStrategy {
    pub fn get_scraper(source: &str) -> anyhow::Result<Box<dyn JobScraper + Send + Sync>> {
        match source {
            "google_linkedin" => Ok(Box::new(GoogleScraper::new("linkedin.com/jobs/view", JobSource::GoogleLinkedIn))),
            "google_infojobs" => Ok(Box::new(GoogleScraper::new("infojobs.net", JobSource::GoogleInfoJobs))),
            "google_indeed"   => Ok(Box::new(GoogleScraper::new("es.indeed.com", JobSource::GoogleIndeed))),
            "google_glassdoor"=> Ok(Box::new(GoogleScraper::new("glassdoor.es", JobSource::GoogleGlassdoor))),
            "google_manfred"  => Ok(Box::new(GoogleScraper::new("getmanfred.com", JobSource::GoogleManfred))),
            "google_ticjob"   => Ok(Box::new(GoogleScraper::new("ticjob.es", JobSource::GoogleTicjob))),
            "google_stackoverflow" => Ok(Box::new(GoogleScraper::new("stackoverflow.com/jobs", JobSource::GoogleStackOverflow))),
            "google_wwr"      => Ok(Box::new(GoogleScraper::new("weworkremotely.com", JobSource::GoogleWeWorkRemotely))),
            "google_remoteok" => Ok(Box::new(GoogleScraper::new("remoteok.com", JobSource::GoogleRemoteOK))),
            "google_general"  => Ok(Box::new(GoogleScraper::new("", JobSource::GoogleGeneral))),
            "linkedin_direct" => Ok(Box::new(LinkedInScraper)),
            "infojobs_direct" => Ok(Box::new(InfoJobsScraper)),
            _ => Err(anyhow::anyhow!("Fuente no soportada")),
        }
    }
}