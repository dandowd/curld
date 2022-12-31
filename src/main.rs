mod cli;
mod endpoints;
mod settings;
mod templates;

use endpoints::endpoint_settings::{self, EndpointSettings};
use settings::global_settings::GlobalSettings;

fn main() {
    let mut global_settings = GlobalSettings::init();
    global_settings.init_module(
        endpoint_settings::ENDPOINT_MODULE,
        EndpointSettings::default(),
    );
    global_settings.write();

    cli::run();
}
