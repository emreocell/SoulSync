//! Display languages.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Languages SoulSync can display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    En,
    De,
    Es,
}

impl Lang {
    /// Picks a language from a locale string such as `de_DE.UTF-8`.
    /// Anything that is not German or Spanish falls back to English.
    pub fn from_locale(locale: &str) -> Lang {
        let locale = locale.to_ascii_lowercase();
        if locale.starts_with("de") {
            Lang::De
        } else if locale.starts_with("es") {
            Lang::Es
        } else {
            Lang::En
        }
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "english" => Ok(Lang::En),
            "de" | "german" | "deutsch" => Ok(Lang::De),
            "es" | "spanish" | "español" | "espanol" => Ok(Lang::Es),
            other => Err(format!("unknown language '{other}' (use en, de or es)")),
        }
    }
}

/// A name in English and German. Names that only exist in these two
/// (Pokémon, places, games) are shown in English for the other languages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Names {
    pub en: String,
    pub de: String,
}

impl Names {
    pub fn get(&self, lang: Lang) -> &str {
        match lang {
            Lang::En | Lang::Es => &self.en,
            Lang::De => &self.de,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_languages() {
        assert_eq!("DE".parse(), Ok(Lang::De));
        assert_eq!("en".parse(), Ok(Lang::En));
        assert_eq!("es".parse(), Ok(Lang::Es));
        assert_eq!("Español".parse(), Ok(Lang::Es));
        assert!("fr".parse::<Lang>().is_err());
        assert_eq!(Lang::from_locale("de_AT.UTF-8"), Lang::De);
        assert_eq!(Lang::from_locale("es_MX.UTF-8"), Lang::Es);
        assert_eq!(Lang::from_locale("C"), Lang::En);
    }

    #[test]
    fn spanish_shows_english_names() {
        let n = Names {
            en: "Black 2".into(),
            de: "Schwarz 2".into(),
        };
        assert_eq!(n.get(Lang::Es), "Black 2");
        assert_eq!(n.get(Lang::De), "Schwarz 2");
    }
}
