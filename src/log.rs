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
    use sentry::integrations::log;
    use sentry::integrations::log::LoggerOptions;
    use std::env;

    env::set_var("RUST_BACKTRACE", "1");
    let dsn = env::var("SENTRY_DSN").expect("SENTRY_DSN ist nicht gesetzt");
    let guard = sentry::init(dsn);

    sentry::integrations::panic::register_panic_handler();

    let mut builder = env_logger::Builder::from_default_env();
    let logger = builder.build();
    log::init(
        Some(Box::new(logger)),
        LoggerOptions {
            filter: get_log_filter(),
            ..Default::default()
        },
    );

    guard
}

pub fn setup() {
    #[cfg(feature = "transaction_id")]
    use crate::request::transaction;
    use colored::*;

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            #[cfg(feature = "transaction_id")]
            out.finish(format_args!(
                "Transaction-ID {} - {}[{}] {} {}",
                transaction::read_id().green().bold(),
                chrono::Local::now()
                    .format("[%d.%m.%Y][%H:%M:%S]")
                    .to_string()
                    .blue()
                    .bold(),
                record.level().to_string().red().bold(),
                message,
                format!("(in {:?} @ {:?})", record.file(), record.line())
            ));
            #[cfg(not(feature = "transaction_id"))]
            out.finish(format_args!(
                "{}[{}] {} {}",
                chrono::Local::now()
                    .format("[%d.%m.%Y][%H:%M:%S]")
                    .to_string()
                    .blue()
                    .bold(),
                record.level().to_string().red().bold(),
                message,
                format!("(in {:?} @ {:?})", record.file(), record.line())
            ));
        })
        .level(get_log_filter())
        .chain(std::io::stdout())
        .chain(std::io::stderr())
        .apply()
        .expect("Fern konnte nicht initialisiert werden")
}
