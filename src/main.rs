mod endpoints;
mod settings;

fn main() {
    settings::global_settings::init();
    endpoints::endpoint_settings::init();
}
