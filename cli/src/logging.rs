use tracing_subscriber::EnvFilter;

pub fn setup() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .with_writer(std::io::stderr)
        .init();
    trace!("Logger initialized");
}
