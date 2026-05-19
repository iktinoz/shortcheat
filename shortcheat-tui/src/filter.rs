use crate::model::{App, Section};

pub fn filter_apps(apps: &[App], query: &str) -> Vec<App> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return apps.to_vec();
    }

    let mut out = Vec::new();
    for app in apps {
        let mut kept_sections = Vec::new();
        for section in &app.sections {
            let shortcuts: Vec<_> = section
                .shortcuts
                .iter()
                .filter(|s| {
                    s.description.to_lowercase().contains(&q)
                        || s.keys.to_lowercase().contains(&q)
                })
                .cloned()
                .collect();
            if !shortcuts.is_empty() {
                kept_sections.push(Section {
                    name: section.name.clone(),
                    shortcuts,
                });
            }
        }
        if !kept_sections.is_empty() {
            let mut a = app.clone();
            a.sections = kept_sections;
            out.push(a);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Section, Shortcut};

    fn fixture() -> Vec<App> {
        vec![App {
            app: "X".into(),
            description: "".into(),
            sections: vec![Section {
                name: "S".into(),
                shortcuts: vec![
                    Shortcut {
                        keys: "Ctrl + S".into(),
                        description: "Save".into(),
                    },
                    Shortcut {
                        keys: "Super + Q".into(),
                        description: "Close window".into(),
                    },
                ],
            }],
        }]
    }

    #[test]
    fn empty_query_returns_all() {
        let apps = fixture();
        let got = filter_apps(&apps, "");
        assert_eq!(got[0].sections[0].shortcuts.len(), 2);
    }

    #[test]
    fn matches_description() {
        let got = filter_apps(&fixture(), "save");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].sections[0].shortcuts.len(), 1);
        assert_eq!(got[0].sections[0].shortcuts[0].description, "Save");
    }

    #[test]
    fn matches_keys() {
        let got = filter_apps(&fixture(), "super");
        assert_eq!(got[0].sections[0].shortcuts.len(), 1);
    }

    #[test]
    fn drops_empty_apps() {
        let got = filter_apps(&fixture(), "nomatchstring");
        assert!(got.is_empty());
    }
}
