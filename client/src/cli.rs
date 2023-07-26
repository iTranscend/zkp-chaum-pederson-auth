use clap::{Parser, Subcommand};
use url::{ParseError, Url};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

fn test_validity(val: &str) -> Result<String, ParseError> {
    let _: Url = val.parse()?;
    Ok(val.to_string())
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Registers a new user
    Register(RegisterCommand),
    /// Logs in an existing user
    Login(LoginCommand),
}

#[derive(Debug, Parser)]
pub struct RegisterCommand {
    /// Specifies the username to register
    #[clap(short, long, value_name = "USERNAME")]
    pub username: Option<String>,

    /// Specifies the password to register
    #[clap(
        short,
        long,
        value_name = "PASSWORD",
        env = "PASSWORD",
        hide_env_values = true
    )]
    pub password: Option<String>,
    #[clap(flatten)]
    pub server: ServerOptions,
}

#[derive(Debug, Parser)]
pub struct LoginCommand {
    /// Specifies the username to login with
    #[clap(short, long, value_name = "USERNAME")]
    pub username: Option<String>,

    /// Specifies the password to login with
    #[clap(
        short,
        long,
        value_name = "PASSWORD",
        env = "PASSWORD",
        hide_env_values = true
    )]
    pub password: Option<String>,
    #[clap(flatten)]
    pub server: ServerOptions,
}

#[derive(Debug, Parser)]
pub struct ServerOptions {
    /// Specifies the server address to connect to
    #[clap(short = 's', long = "server", value_name = "URI", default_value = "http://127.0.0.1:3000", value_parser = test_validity)]
    pub addr: String,
}
