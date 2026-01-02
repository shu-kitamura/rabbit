use rabbit::{App, AppConfig};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> rabbit::error::Result<()> {
    let config = AppConfig::default();
    let mut app = App::new(config)?;
    app.run()
}
