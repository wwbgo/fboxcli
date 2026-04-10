use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lang {
    Zh,
    En,
}

static LANG: OnceLock<Lang> = OnceLock::new();

pub fn current_lang() -> Lang {
    *LANG.get_or_init(detect_lang)
}

fn detect_lang() -> Lang {
    let locale = sys_locale::get_locale().unwrap_or_default();
    if locale.starts_with("zh") {
        Lang::Zh
    } else {
        Lang::En
    }
}

/// Select message by system locale.
///
/// Usage:
///   t!("English message", "中文消息")
///   t!("Failed for ID: {}", "操作失败(ID: {})", id)
#[macro_export]
macro_rules! t {
    ($en:expr, $zh:expr $(, $arg:expr)*) => {
        match $crate::i18n::current_lang() {
            $crate::i18n::Lang::Zh => format!($zh $(, $arg)*),
            $crate::i18n::Lang::En => format!($en $(, $arg)*),
        }
    };
}
