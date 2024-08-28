use std::error::Error;

use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    pub sub: String,
    pub aud: String,
    pub iss: String,
    pub exp: usize,
    pub nbf: usize,
    pub iat: usize,
}

#[derive(Serialize)]
pub struct JwtToken {
    pub access_token: String,
}

pub trait TokenService: Send + Sync {
    /// Generate new token with expiration
    fn generate_token(&self, subject: &str) -> Result<JwtToken, Box<dyn Error>>;

    /// Validate token and retrieve token claims
    fn get_validated_claims(&self, token: &str) -> Result<UserClaims, Box<dyn Error>>;
}

pub struct JwtTokenService {
    token_lifetime_min: u32,
    signing_key: String,
    issuer: String,
    audience: String,
    token_validation_rules: Validation,
}

impl JwtTokenService {
    pub fn new(
        signing_key: &str,
        issuer: &str,
        audience: &str,
        validation_time_skew_sec: u32,
        token_lifetime_min: u32,
    ) -> Self {
        let mut token_validation = Validation::new(Algorithm::HS256);
        token_validation.leeway = validation_time_skew_sec as u64;
        token_validation.set_audience(&[audience]);
        token_validation.set_required_spec_claims(&["exp", "aud", "sub"]);

        Self {
            signing_key: signing_key.into(),
            issuer: issuer.into(),
            audience: audience.into(),
            token_validation_rules: token_validation,
            token_lifetime_min,
        }
    }
}

impl TokenService for JwtTokenService {
    fn generate_token(&self, subject: &str) -> Result<JwtToken, Box<dyn Error>> {
        let now = Utc::now();

        let exp = now
            .checked_add_signed(chrono::Duration::minutes(self.token_lifetime_min as i64))
            .expect("valid timestamp is required")
            .timestamp() as usize;

        let user_claims = UserClaims {
            aud: self.audience.to_string(),
            iss: self.issuer.to_string(),
            sub: subject.into(),

            exp,
            nbf: now.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let access_token = encode(
            &Header::default(),
            &user_claims,
            &EncodingKey::from_secret(self.signing_key.as_ref()),
        )?;

        Ok(JwtToken { access_token })
    }

    fn get_validated_claims(&self, token: &str) -> Result<UserClaims, Box<dyn Error>> {
        let decoded_token = decode::<UserClaims>(
            token,
            &DecodingKey::from_secret(self.signing_key.as_ref()),
            &self.token_validation_rules,
        )?;

        Ok(decoded_token.claims)
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose, Engine as _};
    use serde_json::Value;

    use super::*;

    #[test]
    fn will_generate_valid_token_with_required_claims() {
        let uut_svc = JwtTokenService::new("secret key", "issuer", "audience", 1, 5);

        let actual_token = uut_svc.generate_token("test_subject").unwrap();

        let token_parts: Vec<&str> = actual_token.access_token.split(".").collect();

        assert_eq!(3, token_parts.len());

        let body_bytes = general_purpose::STANDARD
            .decode(token_parts[1])
            .expect("Invalid base64url data");
        let body_str = String::from_utf8(body_bytes).expect("body must be utf8");
        let body_json: Value = serde_json::from_str(&body_str).expect("body must json");

        assert!(body_json.get("aud").is_some());
        assert!(body_json.get("sub").is_some());
        assert!(body_json.get("exp").is_some());
    }

    #[test]
    fn will_decode_valid_token() {
        let uut_svc = JwtTokenService::new("secret key", "issuer", "audience", 1, 5);

        let token_to_decode = uut_svc.generate_token("test_subject").unwrap();

        let actual_claims = uut_svc
            .get_validated_claims(&token_to_decode.access_token)
            .unwrap();

        assert_eq!("test_subject", actual_claims.sub);
        assert_eq!("audience", actual_claims.aud);
    }

    #[test]
    fn will_return_error_on_expired_token() {
        let uut_svc = JwtTokenService::new("secret key", "issuer", "audience", 1, 5);

        let now = Utc::now();

        let exp = now
            .checked_add_signed(chrono::Duration::minutes(-10))
            .expect("valid timestamp is required")
            .timestamp() as usize;

        let user_claims = UserClaims {
            aud: uut_svc.audience.to_string(),
            iss: uut_svc.issuer.to_string(),
            sub: "test_subject".into(),

            exp,
            nbf: now.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token_to_decode = encode(
            &Header::default(),
            &user_claims,
            &EncodingKey::from_secret(uut_svc.signing_key.as_ref()),
        )
        .expect("valid token required");

        let actual_decode_err = uut_svc.get_validated_claims(&token_to_decode).unwrap_err();

        assert_eq!("ExpiredSignature", actual_decode_err.to_string());
    }

    #[test]
    fn will_return_error_on_invalid_audience_token() {
        let uut_svc = JwtTokenService::new("secret key", "issuer", "audience", 1, 5);

        let now = Utc::now();

        let exp = now
            .checked_add_signed(chrono::Duration::minutes(10))
            .expect("valid timestamp is required")
            .timestamp() as usize;

        let user_claims = UserClaims {
            aud: "some_other_audience".into(),
            iss: uut_svc.issuer.to_string(),
            sub: "test_subject".into(),

            exp,
            nbf: now.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let token_to_decode = encode(
            &Header::default(),
            &user_claims,
            &EncodingKey::from_secret(uut_svc.signing_key.as_ref()),
        )
        .expect("valid token required");

        let actual_decode_err = uut_svc.get_validated_claims(&token_to_decode).unwrap_err();

        assert_eq!("InvalidAudience", actual_decode_err.to_string());
    }
}
