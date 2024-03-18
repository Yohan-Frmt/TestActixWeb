use log::info;

pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format(|formatter, record| {
            writeln!(
                formatter,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                formatter.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}
