Este es un **README.md** diseñado con estándares de industria, ideal para repositorios de portafolio o proyectos empresariales.

---

# 🕵️ Job Hunter Pro: OSINT-Driven Executive Job Search
### *High-Performance Anti-Blocking Job Meta-Scraper*

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/platform-Docker-blue.svg)](https://www.docker.com/)
[![Render](https://img.shields.io/badge/deploy-Render-430098.svg)](https://render.com/)

**Job Hunter Pro** es una solución de ingeniería avanzada diseñada para la localización y extracción de ofertas de empleo en las principales plataformas de España y globales (**LinkedIn, InfoJobs, Manfred, Indeed**, etc.). 

A diferencia de los scrapers tradicionales, este sistema utiliza técnicas de **Google Dorking (X-Ray Search)** para indexar información sin ser detectado, evitando CAPTCHAs y bloqueos de IP. La extracción de datos se realiza mediante **Parser Combinators (`nom`)**, garantizando una estructuración semántica precisa de sueldos, tipos de contrato y fechas.

---

## 🌟 Características Principales

- **Estrategia OSINT (Google X-Ray):** Prioriza el acceso a través de Google para obtener resultados de portales blindados como LinkedIn e InfoJobs sin necesidad de cuentas de usuario.
- **Análisis Semántico con `nom`:** Motor de procesamiento de lenguaje natural basado en parsers de Rust (en lugar de Regex) para identificar sueldos, jornadas y modalidades de trabajo.
- **Estandarización Internacional:** Genera automáticamente esquemas de datos compatibles con **Schema.org/JobPosting**.
- **Interfaz Premium Master-Detail:** Visualización fluida con HTMX que permite explorar ofertas y ver su estructura JSON-LD en modales dedicados.
- **Exportación Estructurada:** Descarga instantánea de resultados en formatos **CSV (Excel)** y **JSON API**.
- **Arquitectura Hexagonal:** Código desacoplado y altamente mantenible, separando la lógica de negocio del motor de navegación.

---

## 🏗️ Arquitectura Técnica

El sistema se basa en un diseño de **Puertos y Adaptadores**:

1.  **Dominio (Core):** Define la entidad `Job` y los enums de estándares internacionales (`EmploymentType`).
2.  **Infraestructura (Adapters):** 
    - **Navegador:** Instancia aislada de Chrome vía Selenium Grid.
    - **Scrapers:** Lógica polimórfica para diferentes fuentes de datos.
    - **Web Server:** Implementación de alto rendimiento con **Axum**.
3.  **Frontend:** Templates Type-safe con **Askama**, estilizados con **Tailwind CSS** e interactividad asíncrona vía **HTMX**.

---

## 🛠️ Tech Stack

- **Backend:** [Rust](https://www.rust-lang.org/) (Tokio runtime)
- **Web Framework:** [Axum 0.7](https://github.com/tokio-rs/axum)
- **Navegación:** [Thirtyfour](https://github.com/stevepryde/thirtyfour) (WebDriver)
- **Parsing:** [Nom 7.1](https://github.com/rust-bakery/nom)
- **Templates:** [Askama](https://github.com/djc/askama)
- **Frontend:** [Tailwind CSS](https://tailwindcss.com/) & [HTMX](https://htmx.org/)
- **DevOps:** [Docker Compose](https://docs.docker.com/compose/) & [Render Blueprints](https://render.com/docs/blueprints)

---

## 🚀 Instalación y Ejecución Local

### Prerrequisitos
- Docker y Docker Compose instalados.

### Pasos
1. Clonar el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/job-hunter-pro.git
   cd job-hunter-pro
   ```

2. Levantar la infraestructura:
   ```bash
   docker-compose up --build
   ```

3. Acceder a la aplicación:
   - **Buscador:** `http://localhost:3000`
   - **Monitor del Bot (VNC):** `http://localhost:7900` (Password: `secret`)

---

## ☁️ Despliegue en Render (Cloud)

Este proyecto está configurado para despliegue automático mediante **Render Blueprints**.

1. Sube este código a un repositorio privado de GitHub/GitLab.
2. En el Dashboard de Render, selecciona **"New Blueprint Instance"**.
3. Conecta tu repositorio.
4. Render creará automáticamente dos servicios coordinados:
    - **`job-browser`**: Instancia privada de Chrome.
    - **`job-hunter-app`**: Servicio web público con el binario de Rust.

---

## 📊 Modelo de Datos (Schema.org)

Cada oferta identificada se estructura siguiendo el estándar global, permitiendo ver el JSON técnico:

```json
{
  "@context": "https://schema.org/",
  "@type": "JobPosting",
  "title": "Senior Rust Developer",
  "hiringOrganization": "TechCorp Spain",
  "employmentType": "FULL_TIME",
  "baseSalary": "50.000€ - 65.000€",
  "jobLocationType": "TELECOMMUTE",
  "datePosted": "2025-12-28"
}
```

---

## ⚖️ Aviso Legal
Este software se proporciona con fines educativos y de investigación. El uso de técnicas de scraping debe cumplir con los Términos de Servicio de las plataformas destino y con la legislación vigente en materia de protección de datos (RGPD). El autor no se hace responsable del uso indebido de esta herramienta.

---
**Desarrollado con ❤️ en Rust.**