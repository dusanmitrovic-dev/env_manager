use logger::{Logger, SeverityLevel};

fn main() {
    let logger = Logger::new(SeverityLevel::Debug);

    _ = logger.info("Hello, world!");
}
