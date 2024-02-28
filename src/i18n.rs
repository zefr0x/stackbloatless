use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester,
};
use once_cell::sync::Lazy;

#[derive(rust_embed::RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

// TODO: Replace once_cell's Lazy with std's Lazy after stabilized.
pub static LANG_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = fluent_language_loader!();

    let requested_languages = DesktopLanguageRequester::requested_languages();
    i18n_embed::select(&loader, &Localizations, &requested_languages).unwrap();

    loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANG_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANG_LOADER, $message_id, $($args), *)
    }};
}
