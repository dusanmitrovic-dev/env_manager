use logger::{Logger, SeverityLevel, info};

fn main() {
    let logger = Logger::new(SeverityLevel::Debug);

    _ = info!(logger, "Hello, {}!", "world");
}
