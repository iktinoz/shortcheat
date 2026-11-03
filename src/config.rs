use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use crate::model::App;

pub fn config_dir() -> Result<PathBuf> {
    let base =
        dirs::config_dir().ok_or_else(|| anyhow!("could not resolve user config directory"))?;
    Ok(base.join("shortcheat"))
}

pub fn load() -> Result<Vec<App>> {
    let dir = config_dir()?;
    if !dir.exists() {
        return Err(anyhow!(
            "config directory not found: {}\n\
             create it and copy examples: mkdir -p {0} && cp examples/*.yaml {0}/",
            dir.display()
        ));
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("yaml"))
        .collect();
    paths.sort();

    let mut apps = Vec::new();
    for p in paths {
        let text = fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
        if text.trim().is_empty() {
            continue;
        }
        let app: App =
            serde_yaml::from_str(&text).with_context(|| format!("parsing {}", p.display()))?;
        apps.push(app);
    }

    if apps.is_empty() {
        return Err(anyhow!("no YAML files found in {}", dir.display()));
    }
    Ok(apps)
}
