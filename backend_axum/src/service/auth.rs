use jsonwebtoken::{
    DecodingKey, Header, Validation, decode, decode_header,
    jwk::{AlgorithmParameters, Jwk, JwkSet},
};
use reqwest::Client;
use tokio::{sync::RwLock, time::Instant};

use crate::{
    error::AppError,
    model::{auth::UserAuth, secret::SecretString},
};

pub struct JwksCache {
    last_refresh: Option<Instant>,
    jwks: JwkSet,
}

impl JwksCache {
    pub fn new() -> Self {
        let jwks = JwkSet { keys: Vec::new() };
        Self {
            last_refresh: None,
            jwks,
        }
    }
}

pub struct AuthService {
    http_client: Client,
    cooldown_refresh: u64,
    jwks_url: String,
    verify_aud: String,
    verify_iss: String,
    jwks_cache: RwLock<JwksCache>,
}

impl AuthService {
    pub async fn new(http_client: &Client) -> Result<Self, AppError> {
        let cooldown_refresh = std::env::var("AUTH_COOLDOWN_REFRESH")?.parse()?;
        let jwks_url = std::env::var("AUTH_JWKS_URL")?;
        let jwks_cache = JwksCache::new();

        let verify_aud = std::env::var("AUTH_AUDIENCE")?;
        let verify_iss = std::env::var("AUTH_ISSUER")?;

        let auth_service = Self {
            http_client: http_client.clone(),
            cooldown_refresh,
            jwks_url,
            jwks_cache: RwLock::new(jwks_cache),
            verify_aud,
            verify_iss,
        };

        auth_service.refresh_jwt_set().await?;

        Ok(auth_service)
    }

    #[tracing::instrument(skip_all)]
    pub async fn parse(&self, token: SecretString) -> Result<UserAuth, AppError> {
        let header = decode_header(&token.0);
        let Ok(header) = header else {
            return Err(AppError::BadRequest("Unable to decode access token".into()));
        };

        let Some(kid) = &header.kid else {
            return Err(AppError::BadRequest(
                "Access token missing `kid` header field".into(),
            ));
        };

        {
            // block scope so the read lock gets dropped
            let jwts_cache_reader = self.jwks_cache.read().await;
            if let Some(jwk) = jwts_cache_reader.jwks.find(&kid) {
                let user_auth = self.decode(token, jwk, &header)?;
                return Ok(user_auth);
            };
        };

        // If we get here, the jwk was not found in the cache and we need to refresh
        self.refresh_jwt_set().await?;

        let jwts_cache_reader = self.jwks_cache.read().await;
        if let Some(jwk) = jwts_cache_reader.jwks.find(&kid) {
            let user_auth = self.decode(token, jwk, &header)?;
            Ok(user_auth)
        } else {
            Err(AppError::BadRequest(
                "Unable to match JWK for access token".into(),
            ))
        }
    }

    pub fn decode(
        &self,
        token: SecretString,
        jwk: &Jwk,
        header: &Header,
    ) -> Result<UserAuth, AppError> {
        let AlgorithmParameters::RSA(ref rsa) = jwk.algorithm else {
            tracing::error!("Unexpected JWK algorithm: {:?}", jwk.algorithm);
            return Err(AppError::BadRequest(
                "Access token should be using RSA".into(),
            ));
        };

        let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;

        let validation = {
            let mut validation = Validation::new(header.alg);
            validation.set_audience(&[&self.verify_aud]);
            validation.set_issuer(&[&self.verify_iss]);
            validation
        };

        let decoded_token = decode::<UserAuth>(&token.0, &decoding_key, &validation);
        match decoded_token {
            Ok(token_data) => Ok(token_data.claims),
            Err(error) => {
                tracing::error!("Failed to decode token: {:?}", error);
                Err(AppError::Unauthorized)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn refresh_jwt_set(&self) -> Result<(), AppError> {
        let mut jwts_cache = self.jwks_cache.write().await;

        if let Some(last_refresh) = jwts_cache.last_refresh.as_ref() {
            let elapsed = last_refresh.elapsed().as_secs();
            if elapsed < self.cooldown_refresh {
                tracing::debug!(
                    "Skipping refresh: elapsed {elapsed} < cooldown {}",
                    self.cooldown_refresh
                );
                return Ok(());
            }
        }

        let jwts: JwkSet = self
            .http_client
            .get(&self.jwks_url)
            .send()
            .await?
            .json()
            .await?;

        tracing::debug!("Refresh fetched {} jwts", jwts.keys.len());

        jwts_cache.last_refresh = Some(Instant::now());
        jwts_cache.jwks = jwts;

        Ok(())
    }
}
