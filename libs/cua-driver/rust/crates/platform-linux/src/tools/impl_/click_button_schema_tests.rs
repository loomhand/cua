use super::{chromium_background_must_refuse, maps_indicate_gtk, ClickTool};
use cua_driver_core::tool::Tool;

/// Surface 5: schema must advertise the three canonical button values and
/// describe the back-compat default. Linux already routed button=middle/right
/// pre-Surface-5; this freezes the schema shape so the contract can't drift.
#[test]
fn schema_advertises_button_enum_and_description() {
    let tool = ClickTool {
        state: super::ToolState::new(),
    };
    let d = tool.def();
    let props = d.input_schema.get("properties").expect("properties");
    let button = props.get("button").expect("button field present");
    assert_eq!(button.get("type").and_then(|v| v.as_str()), Some("string"));
    let enum_vals: Vec<&str> = button
        .get("enum")
        .and_then(|v| v.as_array())
        .expect("button.enum present")
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    for need in ["left", "right", "middle"] {
        assert!(enum_vals.contains(&need), "missing {need} in button.enum");
    }
    let desc = button
        .get("description")
        .and_then(|v| v.as_str())
        .expect("button.description present");
    let lc = desc.to_ascii_lowercase();
    assert!(lc.contains("left"), "description should mention default");
    assert!(
        lc.contains("wayland"),
        "description should call out wayland fallback"
    );
}

#[test]
fn chromium_background_requires_focus_free_inject_mode() {
    assert!(chromium_background_must_refuse(false, false, true));
    assert!(!chromium_background_must_refuse(false, true, true));
    assert!(!chromium_background_must_refuse(true, false, true));
    assert!(!chromium_background_must_refuse(false, false, false));
}

#[test]
fn gtk_process_maps_are_detected_without_matching_unrelated_libraries() {
    assert!(maps_indicate_gtk(
        "7f00-7f01 r-xp /usr/lib/x86_64-linux-gnu/libgtk-3.so.0.2404.32"
    ));
    assert!(maps_indicate_gtk(
        "7f00-7f01 r-xp /nix/store/hash-gtk4/lib/libgtk-4.so.1"
    ));
    assert!(!maps_indicate_gtk(
        "7f00-7f01 r-xp /usr/lib/x86_64-linux-gnu/libgdk_pixbuf-2.0.so"
    ));
}

#[tokio::test]
async fn verify_hit_is_refused_as_an_unknown_argument() {
    let mut registry = cua_driver_core::tool::ToolRegistry::new();
    registry.register(Box::new(ClickTool {
        state: super::ToolState::new(),
    }));
    for verify_hit in [true, false] {
        let result = registry
            .invoke(
                "click",
                serde_json::json!({"pid": 1, "element_token": "s00000001:1", "verify_hit": verify_hit}),
            )
            .await;
        assert_eq!(result.is_error, Some(true), "verify_hit={verify_hit}");
        let refusal = &result.structured_content.expect("structured refusal")["refusal"];
        assert_eq!(
            refusal["code"], "invalid_arguments",
            "verify_hit={verify_hit}"
        );
        assert!(
            refusal["message"]
                .as_str()
                .is_some_and(|m| m.starts_with("click: unknown argument verify_hit;")),
            "verify_hit={verify_hit}: {refusal}"
        );
    }
}
