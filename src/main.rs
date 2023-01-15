mod cli;
mod run;
mod settings;
mod templates;

use run::run_settings::RunSettings;
use settings::global_settings::GlobalSettings;

fn main() {
    let mut global_settings = GlobalSettings::init();
    RunSettings::init(&mut global_settings);
    global_settings.write();

    cli::run();
}
