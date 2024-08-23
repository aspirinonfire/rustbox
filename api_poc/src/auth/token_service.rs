use std::collections::HashMap;

#[allow(dead_code)]
pub struct JwtToken {
    token_value: String,
}

#[allow(dead_code)]
pub trait TokenService: Send + Sync {
    /// Generate new token with expiration
    fn generate_token(&self, token_lifetime_min: u16) -> JwtToken;

    /// Validate token and retrieve token claims
    fn get_validated_claims(&self, token: &str) -> Result<HashMap<String, String>, &'static str>;
}

#[allow(dead_code)]
pub struct JwtTokenService {
    pub signing_key: String,
    pub issuer: String,
    pub audience: String,
    pub validation_time_skew_sec: u16,
}

// TODO implement actual JWT token generation and validation
// see [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
impl TokenService for JwtTokenService {
    fn generate_token(&self, _token_lifetime_min: u16) -> JwtToken {
        // generate claims (iss, aud, exp, etc)
        // generate signature
        // build jwt string

        JwtToken {
            token_value: self.audience.clone(),
        }
    }

    fn get_validated_claims(&self, token: &str) -> Result<HashMap<String, String>, &'static str> {
        // validate signature, expiration (with skew window), and audience
        // extract and return claims if valid

        let mut claims = HashMap::<String, String>::new();

        if token != self.audience {
            return Err("invalid token");
        }

        // for now, return test claims
        claims.insert("aud".into(), self.audience.to_string());
        claims.insert("iss".into(), self.issuer.to_string());
        claims.insert("sub".into(), "test_user".into());

        Ok(claims)
    }
}
