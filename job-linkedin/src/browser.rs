use thirtyfour::prelude::*;
use std::env;
use std::time::Duration;

pub struct BrowserFactory;

impl BrowserFactory {
    pub async fn create() -> anyhow::Result<WebDriver> {
        let selenium_url = env::var("SELENIUM_URL").unwrap_or_else(|_| "http://localhost:4444".to_string());
        
        let mut caps = DesiredCapabilities::chrome();
        
        // MODIFICACIÓN PARA CLOUD: Headless y Optimizaciones
        caps.add_chrome_arg("--headless")?; 
        caps.add_chrome_arg("--no-sandbox")?;
        caps.add_chrome_arg("--disable-dev-shm-usage")?;
        caps.add_chrome_arg("--disable-gpu")?;
        caps.add_chrome_arg("user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")?;

        // MODIFICACIÓN: Bucle de reintentos (Wait for service)
        let mut driver = None;
        for i in 1..=10 {
            match WebDriver::new(&selenium_url, caps.clone()).await {
                Ok(d) => {
                    driver = Some(d);
                    break;
                },
                Err(_) => {
                    println!("⏳ Intento {}/10: Esperando a Chrome en {}...", i, selenium_url);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }

        let driver = driver.ok_or_else(|| anyhow::anyhow!("No se pudo conectar a Selenium tras varios intentos"))?;

        driver.execute(r#"
            Object.defineProperty(navigator, 'webdriver', {get: () => undefined});
        "#, vec![]).await?;

        Ok(driver)
    }
}