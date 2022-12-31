mod cli;
mod endpoints;
mod settings;
mod templates;

use endpoints::endpoint_settings::EndpointSettings;
use settings::global_settings::GlobalSettings;

fn main() {
    let mut global_settings = GlobalSettings::init();
    EndpointSettings::init(&mut global_settings);
    global_settings.write();

    cli::run();
}
