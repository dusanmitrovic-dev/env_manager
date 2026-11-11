use logger::{Logger, SeverityLevel, info};

fn main() {
    let _ = Logger::builder()
        .with_minimal_severity_level(SeverityLevel::Debug)
        .initialize();

    _ = info!("Hello, world!");
}
