//! The script's texts: `lang/{en,fr,es,tr}.lang`, embedded and checked for the same keys while it compiles.
use sp_lang::{Catalog, Language};

static LANG: Catalog = sp_lang::catalog!("info-lite");

/// The text of `key` in the language of `locale` (English for any other), with `{0}`, `{1}` filled from `args`.
pub fn text(locale: &str, key: &str, args: &[&str]) -> String {
    let raw = LANG
        .get_or_en(Language::of_locale(locale), key)
        .unwrap_or_default();
    sp_lang::render(raw, args, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_page_answer_and_every_key_the_script_names_has_a_text() {
        let own = [
            "description",
            "permission_use",
            "command_info",
            "command_news",
            "data_error",
            "save_error",
        ];
        for key in own.iter().chain(sp_pages::KEYS.iter()) {
            for language in Language::ALL {
                assert!(
                    LANG.get(language, key).is_some_and(|t| !t.is_empty()),
                    "{key} in {}",
                    language.code()
                );
            }
        }
        assert_eq!(
            LANG.len(),
            own.len() + sp_pages::KEYS.len(),
            "a key nobody names"
        );
    }

    #[test]
    fn the_page_line_names_the_next_page_in_the_players_language() {
        let next = text("tr_tr", "next", &["1", "3", "info", "2"]);
        assert!(next.contains("/info 2"), "{next}");
        assert_ne!(next, text("en_us", "next", &["1", "3", "info", "2"]));
        assert_eq!(text("xx_yy", "empty", &[]), text("en_us", "empty", &[]));
    }
}
