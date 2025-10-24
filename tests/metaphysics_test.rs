use mingbu::metaphysics::ba_zi_json;

#[test]
fn test_ba_zi_json_structure() {
    let json_str = ba_zi_json(1990, 5, 15, 10).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    
    assert!(parsed["year"].is_array());
    assert!(parsed["day_master"].is_string());
    assert_eq!(parsed["year"].as_array().unwrap().len(), 2);
}