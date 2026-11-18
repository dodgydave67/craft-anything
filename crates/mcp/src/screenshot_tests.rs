use super::*;
use craft_anything_evidence::{Confidence, Environment, EvidenceBuilder, Subject};

#[test]
fn runtime_screenshots_are_images_for_the_client_and_remain_in_retained_evidence() {
    let engine = Arc::new(Engine::new(craft_anything_engine::testing::session()));
    let server = Server::new(engine, ServerOptions::default(), Box::new(std::io::sink()));
    let evidence = EvidenceBuilder::new("runtime_ui_read", Subject::default(), Environment::default())
        .result(json!({"nodes": [], "screenshot": {"media_type": "image/png", "base64": "cG5n", "width": 1, "height": 1}}))
        .summary("A window")
        .confidence(Confidence::Observed)
        .build()
        .unwrap();
    let output = ToolOutput::evidence(evidence);
    let before = output.to_json();
    let body = server.success_body("runtime_ui_read", &output);
    assert_eq!(body["content"][1], json!({"type": "image", "data": "cG5n", "mimeType": "image/png"}));
    assert!(body["content"][0]["text"].as_str().unwrap().contains("image_content"));
    assert!(!body["content"][0]["text"].as_str().unwrap().contains("cG5n"));
    assert!(body["structuredContent"]["result"]["screenshot"].get("base64").is_none());
    assert_eq!(before, output.to_json(), "transport projection must not mutate the original evidence");
    let other = server.success_body("core_evidence_show", &output);
    assert_eq!(other["content"].as_array().unwrap().len(), 1, "other tools keep their existing contract");
}

#[test]
fn missing_or_non_png_screenshots_keep_the_text_contract() {
    let engine = Arc::new(Engine::new(craft_anything_engine::testing::session()));
    let server = Server::new(engine, ServerOptions::default(), Box::new(std::io::sink()));
    for value in [json!({}), json!({"screenshot": {"media_type": "text/plain", "base64": "cG5n"}})] {
        let output = ToolOutput::Value(value);
        assert_eq!(server.success_body("runtime_ui_read", &output)["content"].as_array().unwrap().len(), 1);
    }
}
