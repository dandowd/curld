mod cli;
mod run;
mod settings;
mod templates;

use run::endpoint_settings::EndpointSettings;
use settings::global_settings::GlobalSettings;

fn main() {
    let mut global_settings = GlobalSettings::init();
    EndpointSettings::init(&mut global_settings);
    global_settings.write();

    cli::run();
}
