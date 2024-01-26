mod web_server {
    pub mod server;
}

fn main() {
    web_server::server::setup_server();
}
