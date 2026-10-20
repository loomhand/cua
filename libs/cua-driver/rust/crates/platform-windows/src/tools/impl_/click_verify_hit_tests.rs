use super::ClickTool;
use cua_driver_core::tool::Tool;

#[test]
fn schema_advertises_verify_hit() {
    let tool = ClickTool {
        state: super::ToolState::new(None),
    };
    let d = tool.def();
    assert_eq!(
        d.input_schema["properties"]["verify_hit"]["type"],
        "boolean"
    );
}

#[tokio::test]
async fn verify_hit_without_an_element_token_is_refused_before_any_input() {
    let tool = ClickTool {
        state: super::ToolState::new(None),
    };
    for args in [
        serde_json::json!({"pid": 1, "window_id": 2, "x": 10, "y": 10, "verify_hit": true}),
        serde_json::json!({"pid": 1, "window_id": 2, "verify_hit": true}),
    ] {
        let result = tool.invoke(args.clone()).await;
        assert_eq!(result.is_error, Some(true), "{args}");
        let code = result
            .structured_content
            .as_ref()
            .map(|s| s["code"].clone());
        assert_eq!(code, Some(serde_json::json!("invalid_arguments")), "{args}");
    }
}
