use std::env;
use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;
use log::warn;

use zkp_utils::style;

pub const DEFAULT_PORT: u16 = 3000;
pub const DEFAULT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), DEFAULT_PORT);

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Args {
    /// Sets the address to listen on [default: 127.0.0.1:3000]
    /// Valid: `3000`, `127.0.0.1`, `127.0.0.1:3000` [env: PORT]
    #[clap(short, long, value_name = "URI")]
    #[clap(verbatim_doc_comment, value_parser = addr_from_str)]
    #[clap(default_value = "127.0.0.1", hide_default_value = true)]
    pub listen: SocketAddr,
}

pub fn addr_from_str(s: &str) -> Result<SocketAddr, AddrParseError> {
    let mut addr = DEFAULT_ADDR;

    let env_port = 'port: {
        if let Ok(env_port) = env::var("PORT") {
            if let Ok(env_port) = env_port.parse::<u16>() {
                break 'port Some(env_port);
            } else {
                warn!(
                    "invalid '{}PORT{}' environment variable: '{}{}{}', ignoring..",
                    style::BOLD,
                    style::RESET,
                    style::fg::YELLOW,
                    env_port,
                    style::fg::RESET,
                );
            }
        }
        None
    };

    if let Ok(port) = s.parse::<u16>() {
        addr.set_port(port);
        return Ok(addr);
    }

    if let Ok(host) = s.parse::<IpAddr>() {
        addr.set_ip(host);
        if let Some(port) = env_port {
            addr.set_port(port);
        }
        return Ok(addr);
    }

    s.parse()
}
