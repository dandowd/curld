mod cli;
mod run;
mod settings;
mod templates;

use run::settings::RunSettings;
use settings::{file::FileStorage, global_settings::GlobalSettings};

fn main() {
    let mut global_settings = GlobalSettings::new(FileStorage::new(None));
    RunSettings::init(&mut global_settings);
    global_settings.write();

    cli::run();
}
