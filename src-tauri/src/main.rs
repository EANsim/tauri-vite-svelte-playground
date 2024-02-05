mod server {
    pub mod web_server;
}
fn main() {
    server::web_server::setup_server();
}
