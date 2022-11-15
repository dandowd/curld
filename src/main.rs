mod endpoints;
mod settings;

use endpoints::endpoint_settings;
use settings::global_settings;

fn main() {
    let mut global_settings = global_settings::init();
    endpoint_settings::init(&mut global_settings);
    global_settings.write();
}
