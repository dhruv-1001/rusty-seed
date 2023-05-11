use tracing::subscriber;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub struct Tracer;

impl Tracer {
    pub fn init() {
        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
        let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
        subscriber::set_global_default(subscriber).unwrap();
    }
}
