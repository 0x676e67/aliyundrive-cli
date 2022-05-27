pub mod qr;

use crate::models::{auth, gen, query, suc};

#[allow(dead_code)]
pub enum State {
    CONFIRMED,
    EXPIRED,
    NEW,
}

impl State {
    pub fn eq(&self, str: &String) -> bool {
        let value = self.to_string();
        return str.eq(&value);
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::NEW => "NEW".to_string(),
            State::EXPIRED => "EXPIRED".to_string(),
            State::CONFIRMED => "CONFIRMED".to_string(),
        }
    }
}

pub trait QrCodeScanner {
    // get qrcode generator result contend.
    fn get_generator_result(&self) -> crate::Result<gen::GeneratorQrCodeResult>;

    // query qrcode result
    fn get_query_result(
        &self,
        from: &query::QueryQrCodeCkForm,
    ) -> crate::Result<query::QueryQrCodeResult>;

    // login login result（include authorization code）
    fn token_login(&self, token: auth::Token) -> crate::Result<suc::GotoResult>;

    // get web side login
    fn get_token(&self, auth: auth::AuthorizationCode);
}

pub trait QrCodeScannerState {
    fn is_new(&self) -> bool;

    fn is_expired(&self) -> bool;

    fn is_confirmed(&self) -> bool;
}
