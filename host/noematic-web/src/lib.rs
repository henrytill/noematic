use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::prelude::*;

use noematic::message::{
    Action, Request, Response, ResponseAction, SaveResponsePayload, SearchResponsePayload,
};

/// Dew it
#[wasm_bindgen]
pub async fn execute(input: JsValue) -> Result<JsValue, JsError> {
    let request: Request = serde_wasm_bindgen::from_value(input)
        .or(Err(JsError::new("Failed to deserialize request")))?;

    let correlation_id = request.correlation_id;

    web_sys::console::log_2(&"execute".into(), &correlation_id.clone().into());

    let response = match request.action {
        Action::SaveRequest { payload: _ } => {
            let payload = SaveResponsePayload {
                status: "Success".to_string(),
                details: "Item saved".to_string(),
            };
            let action = ResponseAction::SaveResponse { payload };
            Response {
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
                action,
                correlation_id,
            }
        }
    };

    let serializer = Serializer::new().serialize_maps_as_objects(true);
    response
        .serialize(&serializer)
        .or(Err(JsError::new("Failed to serialize response")))
}
