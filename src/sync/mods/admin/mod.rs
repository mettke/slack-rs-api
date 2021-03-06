//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

#![allow(unused_imports)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::blacklisted_name)]

pub mod apps;
pub mod conversations;
pub mod emoji;
pub mod invite_requests;
pub mod teams;
pub mod usergroups;
pub mod users;

pub use crate::mod_types::admin::*;
use crate::sync::SlackWebRequestSender;
use std::borrow::Cow;
