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

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub use crate::mod_types::auth_types::*;
use crate::sync::SlackWebRequestSender;

/// Revokes a token.
///
/// Wraps https://api.slack.com/methods/auth.revoke

pub fn revoke<R>(
    client: &R,
    request: &RevokeRequest,
) -> Result<RevokeResponse, RevokeError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![request.test.as_ref().map(|test| ("test", test.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/auth.revoke");
    client
        .get(&url, &params[..])
        .map_err(RevokeError::Client)
        .and_then(|result| {
            serde_json::from_str::<RevokeResponse>(&result)
                .map_err(|e| RevokeError::MalformedResponse(result, e))
        })
}
/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test

pub fn test<R>(client: &R, request: &TestRequest) -> Result<TestResponse, TestError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/auth.test");
    client
        .get(&url, &params[..])
        .map_err(TestError::Client)
        .and_then(|result| {
            serde_json::from_str::<TestResponse>(&result)
                .map_err(|e| TestError::MalformedResponse(result, e))
        })
}