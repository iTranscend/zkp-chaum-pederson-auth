use std::collections::HashMap;

use clap::Parser;
use log::{debug, error, info};
use num_bigint::BigUint;
use tokio::sync::RwLock;
use tonic::{async_trait, transport::Server};

use zkp_common::{consts, proto};
use zkp_utils::{biguint, logger, random, style};

mod cli;

#[derive(Debug)]
pub struct UserData {
    pub state: State,
    pub credentials: Credentials,
}

#[derive(Debug)]
pub struct Credentials {
    pub y1: BigUint,
    pub y2: BigUint,
}

#[derive(Debug)]
pub enum State {
    Authenticated {
        session_id: String,
    },
    Authenticating {
        r1: BigUint,
        r2: BigUint,
        c: BigUint,
    },
    Unauthenticated,
}

type UserName = String;
type AuthId = String;

#[derive(Default)]
pub struct AuthService {
    pub user_datastore: RwLock<HashMap<UserName, UserData>>,
    pub auth_pairs: RwLock<HashMap<AuthId, UserName>>, // improvement: these auth pair entries should expire after some time
}

// Alphanumeric Permutations: (26 + 10) ^ 32 = 63340286662973277706162286946811886609896461828096
const AUTH_ID_LEN: usize = 32;

// Alphanumeric Permutations: (26 + 10) ^ 12 = 475920314814253376
// User-scoped, so this is fine
const SESSION_ID_LEN: usize = 12;

#[async_trait]
impl proto::Auth for AuthService {
    async fn register(
        &self,
        req: tonic::Request<proto::RegisterRequest>,
    ) -> Result<tonic::Response<proto::RegisterResponse>, tonic::Status> {
        debug!(
            "'{}{}{}' received: {:?}",
            style::fg::YELLOW,
            "RegisterRequest",
            style::fg::RESET,
            req
        );
        let proto::RegisterRequest { user, y1, y2 } = req.into_inner();

        info!(
            "'{}{}{}' received for '{}{}{}'",
            style::fg::YELLOW,
            "RegisterRequest",
            style::fg::RESET,
            style::fg::CYAN,
            user,
            style::fg::RESET,
        );

        let y1 = biguint::deserialize(&y1);
        let y2 = biguint::deserialize(&y2);

        let mut user_datastore = self.user_datastore.write().await;

        if user_datastore.contains_key(&user) {
            error!(
                "user '{}{}{}' already exists",
                style::fg::CYAN,
                user,
                style::fg::RESET
            );

            return Err(tonic::Status::already_exists(format!(
                "user '{}' already exists",
                user
            )));
        }

        let user_details = UserData {
            state: State::Unauthenticated,
            credentials: Credentials { y1, y2 },
        };

        user_datastore.insert(user.clone(), user_details);
        info!(
            "user '{}{}{}' registered successfully",
            style::fg::CYAN,
            user,
            style::fg::RESET
        );

        Ok(tonic::Response::new(proto::RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        req: tonic::Request<proto::AuthenticationChallengeRequest>,
    ) -> Result<tonic::Response<proto::AuthenticationChallengeResponse>, tonic::Status> {
        debug!(
            "'{}{}{}' received: {:?}",
            style::fg::YELLOW,
            "AuthenticationChallengeRequest",
            style::fg::RESET,
            req
        );
        let proto::AuthenticationChallengeRequest { user, r1, r2 } = req.into_inner();

        info!(
            "'{}{}{}' received for '{}{}{}'",
            style::fg::YELLOW,
            "AuthenticationChallengeRequest",
            style::fg::RESET,
            style::fg::CYAN,
            user,
            style::fg::RESET,
        );

        let r1 = biguint::deserialize(&r1);
        let r2 = biguint::deserialize(&r2);

        // c: random c
        let c = random::biguint(&consts::PARAMS.Q);

        let auth_id = random::alphanumeric(AUTH_ID_LEN);

        let mut user_datastore = self.user_datastore.write().await;
        let mut auth_pairs = self.auth_pairs.write().await;
        if let Some(user_data) = user_datastore.get_mut(&user) {
            user_data.state = State::Authenticating {
                r1,
                r2,
                c: c.clone(),
            };
        } else {
            error!(
                "user '{}{}{}' not found",
                style::fg::CYAN,
                user,
                style::fg::RESET
            );

            return Err(tonic::Status::not_found(format!(
                "user '{}' not found",
                user
            )));
        }
        auth_pairs.insert(auth_id.clone(), user.clone());
        info!(
            "authentication challenge created for user '{}{}{}'",
            style::fg::CYAN,
            user,
            style::fg::RESET
        );

        Ok(tonic::Response::new(
            proto::AuthenticationChallengeResponse {
                auth_id,
                c: biguint::serialize(c),
            },
        ))
    }

    async fn verify_authentication(
        &self,
        req: tonic::Request<proto::AuthenticationAnswerRequest>,
    ) -> Result<tonic::Response<proto::AuthenticationAnswerResponse>, tonic::Status> {
        debug!(
            "'{}{}{}' received: {:?}",
            style::fg::YELLOW,
            "AuthenticationAnswerRequest",
            style::fg::RESET,
            req
        );
        let proto::AuthenticationAnswerRequest { auth_id, s } = req.into_inner();

        info!(
            "'{}{}{}' received with auth_id '{}{}{}'",
            style::fg::YELLOW,
            "AuthenticationAnswerRequest",
            style::fg::RESET,
            style::fg::CYAN,
            auth_id,
            style::fg::RESET,
        );

        let s = biguint::deserialize(&s);

        let mut auth_pairs = self.auth_pairs.write().await;
        let mut user_datastore = self.user_datastore.write().await;

        let (user_id, user) = if let Some(user_id) = auth_pairs.remove(&auth_id) {
            if let Some(user_data) = user_datastore.get_mut(&user_id) {
                (user_id, user_data)
            } else {
                error!(
                    "authentication challenge created for user '{}{}{}' who doesn't exist",
                    style::fg::CYAN,
                    user_id,
                    style::fg::RESET
                );

                return Err(tonic::Status::internal(
                    "an authentication challenge was created for a user that doesn't exist",
                ));
            }
        } else {
            error!(
                "authentication challenge with auth_id '{}{}{}' not found / expired",
                style::fg::CYAN,
                auth_id,
                style::fg::RESET
            );

            return Err(tonic::Status::not_found(format!(
                "auth_id '{}' not found / expired",
                auth_id
            )));
        };

        let Credentials { y1, y2 } = &user.credentials;

        let (r1, r2, c) = if let State::Authenticating { r1, r2, c } = &user.state {
            (r1, r2, c)
        } else {
            error!(
                "user '{}{}{}' is not expecting to be authenticated",
                style::fg::CYAN,
                user_id,
                style::fg::RESET
            );

            return Err(tonic::Status::internal(
                "this user is not expecting to be authenticated",
            ));
        };

        if consts::PARAMS.verify((y1, y2), (r1, r2), c, &s) {
            let session_id = random::alphanumeric(SESSION_ID_LEN);
            user.state = State::Authenticated {
                session_id: session_id.clone(),
            };

            info!(
                "user '{}{}{}' authenticated successfully",
                style::fg::CYAN,
                user_id,
                style::fg::RESET
            );
            Ok(tonic::Response::new(proto::AuthenticationAnswerResponse {
                session_id,
            }))
        } else {
            user.state = State::Unauthenticated;

            error!(
                "authentication challenge failed for user '{}{}{}'",
                style::fg::CYAN,
                user_id,
                style::fg::RESET
            );
            Err(tonic::Status::unauthenticated(
                "authentication challenge failed",
            ))
        }
    }
}

async fn init() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    let auth_service = AuthService::default();

    eprintln!("================== ZKP Auth (Server) ==================");

    println!(
        "{}[i]{} Listening on '{}{}{}'",
        style::fg::GREEN,
        style::fg::RESET,
        style::fg::CYAN,
        args.listen,
        style::fg::RESET
    );

    Server::builder()
        .add_service(proto::AuthServer::new(auth_service))
        .serve(args.listen)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::setup();

    init().await?;

    Ok(())
}
