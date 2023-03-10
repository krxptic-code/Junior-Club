extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac};
use jwt::{Header, Token, VerifyWithKey};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use hmac::digest::KeyInit;

/// Holds parameters that were encoded in the JSON web token.
///
/// # Attributes
/// * user_id (i32): the ID of a user
pub struct JwtToken {
    pub user_id: i32,
    pub body: String
}

impl JwtToken {

    /// Creates a JSON web token.
    ///
    /// # Arguments
    /// * user_id (i32): ID of the user to be encoded into the token.
    ///
    /// # Returns
    /// (String): The token with all arguments encoded into it
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"moon").unwrap();

        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        let token_str: String = claims.sign_with_key(&key).unwrap();
        return token_str
    }

    /// Extracts the user ID from the encoded JSON web token.
    ///
    /// # Arguments
    /// * encoded_token (String): The JSON web token to be decoded.
    ///
    /// # Returns
    /// (Result<JwtToken, &'static str>): struct containing parameters from the token
    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"moon").unwrap();
        let token_str: &str = encoded_token.as_str();

        let token: Result<Token<Header, BTreeMap<String, i32>, _>, _> = VerifyWithKey::verify_with_key(token_str, &key);
        return match token {
            Ok(token) => {
                let _header = token.header();
                let claims = token.claims();
                Ok(JwtToken { user_id: claims["user_id"], body: encoded_token })
            }
            Err(_) => Err("Could not decode")
        }
    }
}