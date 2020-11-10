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

pub mod profile;

pub use crate::mod_types::users::*;
use crate::sync::SlackWebRequestSender;

/// List conversations the calling user may access.
///
/// Wraps https://api.slack.com/methods/users.conversations

pub fn conversations<R>(
    client: &R,
    token: Option<&str>,
    request: &ConversationsRequest,
) -> Result<ConversationsResponse, ConversationsError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .exclude_archived
            .as_ref()
            .map(|exclude_archived| ("exclude_archived", exclude_archived.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
        request
            .types
            .as_ref()
            .map(|types| ("types", types.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.conversations");
    client
        .get(&url, &params[..])
        .map_err(ConversationsError::Client)
        .and_then(|result| {
            serde_json::from_str::<ConversationsResponse>(&result)
                .map_err(|e| ConversationsError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

pub fn delete_photo<R>(
    client: &R,
    token: &str,
    _request: &DeletePhotoRequest,
) -> Result<DeletePhotoResponse, DeletePhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.deletePhoto");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(DeletePhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<DeletePhotoResponse>(&result)
                .map_err(|e| DeletePhotoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

pub fn get_presence<R>(
    client: &R,
    token: &str,
    request: &GetPresenceRequest,
) -> Result<GetPresenceResponse, GetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.getPresence");
    client
        .get(&url, &params[..])
        .map_err(GetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<GetPresenceResponse>(&result)
                .map_err(|e| GetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

pub fn identity<R>(
    client: &R,
    token: Option<&str>,
    _request: &IdentityRequest,
) -> Result<IdentityResponse, IdentityError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![token.map(|token| ("token", token.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.identity");
    client
        .get(&url, &params[..])
        .map_err(IdentityError::Client)
        .and_then(|result| {
            serde_json::from_str::<IdentityResponse>(&result)
                .map_err(|e| IdentityError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

pub fn info<R>(
    client: &R,
    token: &str,
    request: &InfoRequest,
) -> Result<InfoResponse, InfoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        request
            .include_locale
            .as_ref()
            .map(|include_locale| ("include_locale", include_locale.to_string())),
        request.user.as_ref().map(|user| ("user", user.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.info");
    client
        .get(&url, &params[..])
        .map_err(InfoError::Client)
        .and_then(|result| {
            serde_json::from_str::<InfoResponse>(&result)
                .map_err(|e| InfoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

pub fn list<R>(
    client: &R,
    token: Option<&str>,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        token.map(|token| ("token", token.to_string())),
        request
            .cursor
            .as_ref()
            .map(|cursor| ("cursor", cursor.to_string())),
        request
            .include_locale
            .as_ref()
            .map(|include_locale| ("include_locale", include_locale.to_string())),
        request
            .limit
            .as_ref()
            .map(|limit| ("limit", limit.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.list");
    client
        .get(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Find a user with an email address.
///
/// Wraps https://api.slack.com/methods/users.lookupByEmail

pub fn lookup_by_email<R>(
    client: &R,
    token: &str,
    request: &LookupByEmailRequest,
) -> Result<LookupByEmailResponse, LookupByEmailError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        Some(("token", token.to_string())),
        Some(("email", request.email.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.lookupByEmail");
    client
        .get(&url, &params[..])
        .map_err(LookupByEmailError::Client)
        .and_then(|result| {
            serde_json::from_str::<LookupByEmailResponse>(&result)
                .map_err(|e| LookupByEmailError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Marked a user as active. Deprecated and non-functional.
///
/// Wraps https://api.slack.com/methods/users.setActive

pub fn set_active<R>(
    client: &R,
    token: &str,
    _request: &SetActiveRequest,
) -> Result<SetActiveResponse, SetActiveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setActive");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetActiveError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetActiveResponse>(&result)
                .map_err(|e| SetActiveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Set the user profile photo
///
/// Wraps https://api.slack.com/methods/users.setPhoto

pub fn set_photo<R>(
    client: &R,
    token: &str,
    request: &SetPhotoRequest,
) -> Result<SetPhotoResponse, SetPhotoError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .crop_w
            .as_ref()
            .map(|crop_w| ("crop_w", crop_w.to_string())),
        request
            .crop_x
            .as_ref()
            .map(|crop_x| ("crop_x", crop_x.to_string())),
        request
            .crop_y
            .as_ref()
            .map(|crop_y| ("crop_y", crop_y.to_string())),
        request
            .image
            .as_ref()
            .map(|image| ("image", image.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setPhoto");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetPhotoError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPhotoResponse>(&result)
                .map_err(|e| SetPhotoError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

pub fn set_presence<R>(
    client: &R,
    token: &str,
    request: &SetPresenceRequest,
) -> Result<SetPresenceResponse, SetPresenceError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![Some(("presence", request.presence.to_string()))];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/users.setPresence");
    client
        .post(&url, &params[..], &[("token", token.to_string())])
        .map_err(SetPresenceError::Client)
        .and_then(|result| {
            serde_json::from_str::<SetPresenceResponse>(&result)
                .map_err(|e| SetPresenceError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}
