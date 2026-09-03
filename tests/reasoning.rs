// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0.

use super::*;

#[test]
fn test_reasoning_engine() {
    let mut engine = ReasoningEngine::new();
    engine.add_fact("capital of France", "Paris");
    assert_eq!(engine.reason("What is the capital of France?"), Some("Paris".to_string()));
    assert_eq!(engine.reason("What is the capital of Germany?"), None);
    let reflected = engine.reflect();
    assert_eq!(reflected.len(), 1);
    assert_eq!(reflected[0].0, "capital of France");
    assert_eq!(reflected[0].1, "Paris");
    let hypotheses = engine.generate_hypotheses("Paris is the capital of France");
    assert!(hypotheses.is_empty());
    assert!(engine.verify("Paris is the capital of France"));
}
