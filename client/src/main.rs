use num_bigint::BigUint;

use clap::Parser;
use log::error;

use zkp_common::{consts, proto};
use zkp_utils::{biguint, logger, random, style, string};

mod cli;
mod utils;

const MAX_TRIES: usize = 3;

async fn register_user(details: cli::RegisterCommand) -> anyhow::Result<()> {
    let mut client = proto::AuthClient::connect(details.server.addr).await?;

    eprintln!("=============== ZKP Auth (Registration) ===============");
    let mut user_id = utils::maybe_input(details.username, "Enter a User ID:")?;
    let mut password = utils::maybe_password(details.password, "Select a Password:")?;
    'outer: {
        for i in 0..MAX_TRIES {
            if i > 0 {
                user_id = utils::maybe_input(None, "Enter a User ID:")?;
                password = utils::maybe_password(None, "Select a Password:")?;
            }
            let x = string::as_biguint(&password);

            let (y1, y2) = consts::PARAMS.obfuscate(&x);

            let register_request = tonic::Request::new(proto::RegisterRequest {
                user: user_id.clone(),
                y1: biguint::serialize(y1),
                y2: biguint::serialize(y2),
            });
            if let Err(err) = client.register(register_request).await {
                match err.code() {
                    tonic::Code::AlreadyExists => error!(
                        "user '{}{}{}' already exists",
                        style::fg::YELLOW,
                        user_id,
                        style::fg::RESET
                    ),
                    _ => {
                        error!(
                            "failed to register user: '{}{:?}{}",
                            style::fg::RED,
                            err.code(),
                            style::fg::RESET
                        );
                    }
                }
            } else {
                println!(
                    "{}[i]{} Successfully registered user",
                    style::fg::GREEN,
                    style::fg::RESET
                );
                break 'outer;
            }
            eprintln!(
                "---------------------- [ {} / {} ] ----------------------",
                i + 1,
                MAX_TRIES
            );
        }
        error!("failed to register user after {} tries", MAX_TRIES);
    }
    eprintln!("=============== ZKP Auth (Registration) ===============");

    Ok(())
}

async fn login_user(details: cli::LoginCommand) -> anyhow::Result<()> {
    let mut client = proto::AuthClient::connect(details.server.addr).await?;

    eprintln!("=================== ZKP Auth (Login) ==================");
    let mut user_id = utils::maybe_input(details.username, "Enter Your User ID:")?;
    let mut password = utils::maybe_password(details.password, "Enter Your Password:")?;
    'outer: {
        for i in 0..MAX_TRIES {
            'inner: {
                if i > 0 {
                    user_id = utils::maybe_input(None, "Enter a User ID:")?;
                    password = utils::maybe_password(None, "Select a Password:")?;
                }
                let x = string::as_biguint(&password);

                // k: random k
                let k = random::biguint(&consts::PARAMS.Q);

                let (r1, r2) = consts::PARAMS.obfuscate(&k);

                let auth_response = match client
                    .create_authentication_challenge(tonic::Request::new(
                        proto::AuthenticationChallengeRequest {
                            user: user_id.clone(),
                            r1: biguint::serialize(r1),
                            r2: biguint::serialize(r2),
                        },
                    ))
                    .await
                {
                    Ok(auth_response) => auth_response,
                    Err(err) => {
                        match err.code() {
                            tonic::Code::NotFound => error!(
                                "user '{}{}{}' does not exist",
                                style::fg::YELLOW,
                                user_id,
                                style::fg::RESET
                            ),
                            _ => {
                                error!(
                                    "failed to create authentication challenge: '{}{:?}{}",
                                    style::fg::RED,
                                    err.code(),
                                    style::fg::RESET
                                );
                            }
                        }
                        break 'inner;
                    }
                };

                let proto::AuthenticationChallengeResponse { auth_id, c } =
                    auth_response.into_inner();

                let c = biguint::deserialize(&c);

                let s = consts::PARAMS.solve_challenge(&k, &c, &x);

                match client
                    .verify_authentication(tonic::Request::new(
                        proto::AuthenticationAnswerRequest {
                            auth_id,
                            s: biguint::serialize(s),
                        },
                    ))
                    .await
                {
                    Ok(auth_ans_response) => {
                        let proto::AuthenticationAnswerResponse { session_id } =
                            auth_ans_response.into_inner();

                        println!(
                            "{}[i]{} Successfully authenticated user, session ID is: {:?}",
                            style::fg::GREEN,
                            style::fg::RESET,
                            session_id
                        );
                        break 'outer;
                    }
                    Err(err) => match err.code() {
                        tonic::Code::NotFound => error!(
                            "user '{}{}{}' does not have an authentication challenge",
                            style::fg::YELLOW,
                            user_id,
                            style::fg::RESET
                        ),
                        tonic::Code::Unauthenticated => {
                            error!("failed to authenticate, invalid credentials",)
                        }
                        _ => {
                            error!(
                                "failed to verify authentication: '{}{:?}{}",
                                style::fg::RED,
                                err.code(),
                                style::fg::RESET
                            );
                        }
                    },
                }
            }

            eprintln!(
                "---------------------- [ {} / {} ] ----------------------",
                i + 1,
                MAX_TRIES
            );
        }
        error!("failed to register user after {} tries", MAX_TRIES);
    }
    eprintln!("=================== ZKP Auth (Login) ==================");

    Ok(())
}

async fn init() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    match args.command {
        cli::Command::Register(register) => register_user(register).await?,
        cli::Command::Login(login) => login_user(login).await?,
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::setup();

    init().await?;

    Ok(())
}
