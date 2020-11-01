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

pub use crate::mod_types::search_types::*;
use crate::sync::SlackWebRequestSender;

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages

pub fn messages<R>(
    client: &R,
    request: &MessagesRequest,
) -> Result<MessagesResponse, MessagesError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let params = vec![
        request
            .count
            .as_ref()
            .map(|count| ("count", count.to_string())),
        request
            .highlight
            .as_ref()
            .map(|highlight| ("highlight", highlight.to_string())),
        request.page.as_ref().map(|page| ("page", page.to_string())),
        Some(("query", request.query.to_string())),
        request.sort.as_ref().map(|sort| ("sort", sort.to_string())),
        request
            .sort_dir
            .as_ref()
            .map(|sort_dir| ("sort_dir", sort_dir.to_string())),
    ];
    let params: Vec<(&str, String)> = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("/search.messages");
    client
        .get(&url, &params[..])
        .map_err(MessagesError::Client)
        .and_then(|result| {
            serde_json::from_str::<MessagesResponse>(&result)
                .map_err(|e| MessagesError::MalformedResponse(result, e))
        })
}