mod endpoints;
mod settings;
mod cli;

use endpoints::endpoint_settings;
use settings::global_settings;
use clap::Parser;

fn main() {
    let mut global_settings = global_settings::init();
    endpoint_settings::init(&mut global_settings);
    global_settings.write();

    cli::run();
}
