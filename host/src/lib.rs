pub mod message;

use message::{
    Action, Request, Response, ResponseAction, SaveResponsePayload, SearchResponsePayload,
};

pub fn handle_request(request: Request) -> Response {
    let version = request.version;
    let correlation_id = request.correlation_id;

    match request.action {
        Action::SaveRequest { payload: _ } => {
            let payload = SaveResponsePayload {
                status: "Success".to_string(),
                details: "Item saved".to_string(),
            };
            let action = ResponseAction::SaveResponse { payload };
            Response {
                version,
                action,
                correlation_id,
            }
        }
        Action::SearchRequest { payload: _ } => {
            let payload = SearchResponsePayload {
                results: vec!["Item1".to_string(), "Item2".to_string()],
            };
            let action = ResponseAction::SearchResponse { payload };
            Response {
                version,
                action,
                correlation_id,
            }
        }
    }
}
