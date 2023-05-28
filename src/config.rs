use serde::Deserialize;

fn default_socket_address() -> String { String::from("127.0.0.1:3000") }

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_socket_address")]
    pub socket_address: String
}
