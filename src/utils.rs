// TODO: Replace once_cell's Lazy with std's Lazy after stabilized.
// https://github.com/rust-lang/rust/issues/109736
pub static SYSTEM_TIME_LOCALE: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(get_system_time_locale);

fn get_system_time_locale() -> String {
    use std::env;

    let system_time_locale = env::var("LC_ALL").unwrap_or_else(|_| {
        env::var("LC_TIME")
            // TODO: Use better default value.
            .unwrap_or_else(|_| env::var("LANG").unwrap_or_else(|_| "en".to_owned()))
    });

    // Don't include charset postfix.
    system_time_locale.split('.').next().unwrap().to_owned()
}
