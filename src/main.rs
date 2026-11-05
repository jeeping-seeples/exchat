mod config;

use anyhow::Result;

fn main() -> Result<()> {
    // M0 stub: the egui window arrives in milestone 1.
    let settings = config::Settings::load()?;
    println!("settings file: {}", config::Settings::path()?.display());
    println!(
        "{} provider(s), max search rounds {}",
        settings.providers.len(),
        settings.max_search_rounds
    );
    settings.save()?;
    Ok(())
}
