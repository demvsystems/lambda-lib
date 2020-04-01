use chrono::Local;

fn get_log_filter() -> log::LevelFilter {
    use std::env;
    match env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .as_ref()
    {
        "trace" => log::LevelFilter::Trace,
        "error" => log::LevelFilter::Error,
        "debug" => log::LevelFilter::Debug,
        "warn" => log::LevelFilter::Warn,
        _ => log::LevelFilter::Info,
    }
}

#[cfg(feature = "sentry")]
pub fn setup_sentry() -> sentry::internals::ClientInitGuard {
    use std::env;

    env::set_var("RUST_BACKTRACE", "1");
    let dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN ist nicht gesetzt");
    let guard = sentry::init(dsn);

    sentry::integrations::panic::register_panic_handler();

    guard
}

pub fn setup() {
    #[cfg(feature = "transaction_id")]
    use crate::request::transaction;

    let mut builder = env_logger::Builder::from_default_env();
    builder
        .format(|buf, record| {
            use chrono::TimeZone;
            use chrono_tz::Europe::Berlin;
            use env_logger::fmt::Color;
            use std::io::Write;

            let mut bold_red = buf.style();
            bold_red.set_color(Color::Red).set_bold(true);

            let mut bold_green = buf.style();
            bold_green.set_color(Color::Green).set_bold(true);

            let mut bold_blue = buf.style();
            bold_blue.set_color(Color::Blue).set_bold(true);

            let mut gray = buf.style();
            gray.set_color(Color::Rgb(100, 100, 100));

            let timestamp = Berlin
                .from_local_datetime(&Local::now().naive_local())
                .single()
                .expect("Unexpected Date");

            #[cfg(feature = "transaction_id")]
            let result = writeln!(
                buf,
                "Transaction-ID {} - {}[{}] {} {}",
                bold_green.value(transaction::read_id()),
                bold_blue.value(timestamp.format("[%d.%m.%Y - %H:%M:%S]")),
                bold_red.value(record.level()),
                record.args(),
                gray.value(format!("(in {:?} @ {:?})", record.file(), record.line()))
            );
            #[cfg(not(feature = "transaction_id"))]
            let result = writeln!(
                buf,
                "{}[{}] {} {}",
                bold_blue.value(timestamp.format("[%d.%m.%Y - %H:%M:%S]")),
                bold_red.value(record.level()),
                record.args(),
                gray.value(format!("(in {:?} @ {:?})", record.file(), record.line()))
            );

            result
        })
        .target(env_logger::Target::Stdout)
        .filter(None, get_log_filter())
        .init();
}
