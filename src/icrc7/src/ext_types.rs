use candid::CandidType;
use candid::Principal;
use icrc_ledger_types::icrc1::account::Subaccount;
use serde::Deserialize;

use crate::errors::{ExtCommonError, ExtTransferError};

// b"\x0Atid"
pub static TDS: [u8; 4] = [10, 116, 105, 100];

#[derive(CandidType, Clone, Deserialize, Debug)]
pub struct TokenIdentifier(String);

pub type Balance = u128;

pub type ExtTokenIndex = u32;

pub type Extension = String;

pub static EXTENSIONS: [&str; 3] = ["@ext/common", "@ext/allowance", "@ext/nonfungible"];

impl TokenIdentifier {
    pub fn parse_token_identifier(canister_id: Principal, index: u128) -> Self {
        let mut result = [0u8; 18];
        result[0..4].copy_from_slice(&TDS);
        result[4..14].copy_from_slice(canister_id.as_slice());
        result[14..18].copy_from_slice(&(index as u32).to_be_bytes());
        TokenIdentifier(
            candid::Principal::try_from(&result.to_vec())
                .unwrap()
                .to_text(),
        )
    }

    pub fn parse_token_index(&self, canister_id: Principal) -> Result<u128, ExtCommonError> {
        let (canister, index) = self._parse_token_identifier();
        if &canister[..] != canister_id.as_slice() {
            return Err(ExtCommonError::InvalidToken(self.to_owned()));
        }
        Ok(index)
    }

    fn _parse_token_identifier(&self) -> (Vec<u8>, u128) {
        let array = Principal::from_text(self.0.clone())
            .unwrap()
            .as_slice()
            .to_vec();
        // ic_cdk::println!("parse_token_identifier {:?}", array);
        if array.len() <= 4 || &array[0..4] != TDS {
            return (array, 0);
        }
        if array.len() <= 8 {
            return (array, 0);
        }
        let canister = array[4..array.len() - 4].to_vec();
        let index = &array[array.len() - 4..array.len()];
        let index = (index[0] as u128) << 24
            | (index[1] as u128) << 16
            | (index[2] as u128) << 8
            | (index[3] as u128);
        (canister, index)
    }
}

#[derive(CandidType, Clone, Copy, Debug, Deserialize)]
pub struct AccountIdentifier([u8; 32]);

pub type AccountIdentifierHex = String;

impl AccountIdentifier {
    pub fn from_principal(principal: &Principal, subaccount: &Option<Subaccount>) -> Self {
        let subaccount: [u8; 32] = subaccount.unwrap_or_else(|| [0; 32]);

        assert!(subaccount.len() == 32, "Invalid Subaccount");

        use sha2::Digest;
        let mut hasher = sha2::Sha224::new();
        hasher.update(b"\x0Aaccount-id");
        hasher.update(principal.as_slice());
        hasher.update(&subaccount[..]);
        let hash: [u8; 28] = hasher.finalize().into();

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&hash);
        let crc32_bytes = hasher.finalize().to_be_bytes();

        let mut result: [u8; 32] = [0u8; 32];
        result[0..4].copy_from_slice(&crc32_bytes[..]);
        result[4..32].copy_from_slice(hash.as_ref());

        AccountIdentifier(result)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl Default for AccountIdentifier {
    fn default() -> Self {
        AccountIdentifier([0u8; 32])
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub enum User {
    #[serde(rename = "address")]
    Address(AccountIdentifierHex),
    #[serde(rename = "principal")]
    Principal(Principal),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExtTransferArg {
    pub from: User,
    pub to: User,
    pub token: TokenIdentifier,
    pub memo: Vec<u8>,
    pub amount: u128,
    pub notify: bool,
    pub subaccount: Option<Subaccount>,
}

// pub type ExtTransferResult = Result<u128, ExtTransferError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtTransferResult {
    #[serde(rename = "ok")]
    Ok(u128),
    #[serde(rename = "err")]
    Err(ExtTransferError),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExtBalanceArg {
    pub user: User,
    pub token: TokenIdentifier,
}

// pub type ExtBalanceResult = Result<Balance, ExtCommonError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtBalanceResult {
    #[serde(rename = "ok")]
    Ok(Balance),
    #[serde(rename = "err")]
    Err(ExtCommonError),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct AllowanceArg {
    pub owner: User,
    pub spender: Principal,
    pub token: TokenIdentifier,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExtApproveArg {
    pub subaccount: Option<Subaccount>,
    pub spender: Principal,
    pub allowance: Balance,
    pub token: TokenIdentifier,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExtAllowanceArg {
    pub owner: User,
    pub spender: Principal,
    pub token: TokenIdentifier,
}

// pub type ExtAllowanceResult = Result<Balance, ExtCommonError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtAllowanceResult {
    #[serde(rename = "ok")]
    Ok(Balance),
    #[serde(rename = "err")]
    Err(ExtCommonError),
}

// pub type ExtBearerResult = Result<AccountIdentifier, ExtCommonError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtBearerResult {
    #[serde(rename = "ok")]
    Ok(AccountIdentifierHex),
    #[serde(rename = "err")]
    Err(ExtCommonError),
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct ExtMetadataType {
    pub metadata: Option<Vec<u8>>,
}

impl ExtMetadataType {
    pub fn new(metadata: String) -> Self {
        Self {
            metadata: Some(metadata.into_bytes()),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ExtFungibleMetadataType {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub metadata: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum ExtMetadata {
    #[serde(rename = "fungible")]
    Fungible(ExtFungibleMetadataType),
    #[serde(rename = "nonfungible")]
    Nonfungible(ExtMetadataType),
}

impl Default for ExtMetadata {
    fn default() -> Self {
        ExtMetadata::Nonfungible(ExtMetadataType::default())
    }
}

// pub type ExtMetadataResult = Result<ExtMetadata, ExtCommonError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtMetadataResult {
    #[serde(rename = "ok")]
    Ok(ExtMetadata),
    #[serde(rename = "err")]
    Err(ExtCommonError),
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ExtMintArg {
    pub to: User,
    pub metadata: Option<Vec<u8>>,
}

// pub type ExtSupplyResult = Result<Balance, ExtCommonError>;

#[derive(CandidType, Clone, Deserialize)]
pub enum ExtSupplyResult {
    #[serde(rename = "ok")]
    Ok(Balance),
    #[serde(rename = "err")]
    Err(ExtCommonError),
}
