mod endpoints;
mod settings;
mod cli;

use endpoints::endpoint_settings;
use settings::global_settings;

fn main() {
    let mut global_settings = global_settings::init();
    global_settings.init_module(endpoint_settings::ENDPOINT_MODULE, endpoint_settings::default());
    global_settings.write();

    cli::run();
}
