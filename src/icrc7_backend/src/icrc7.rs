use candid::Principal;
use ic_cdk::{call, caller};

use crate::common::{guards::not_anonymous_caller, types::Icrc7TokenMetadata};

pub mod query_methods;
pub mod update_methods;
