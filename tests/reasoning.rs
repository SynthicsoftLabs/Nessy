// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC. Licensed under Apache-2.0.

use super::*;

#[test]
fn test_reasoning_engine() {
    let mut engine = ReasoningEngine::new();
    engine.add_fact("capital of France", "Paris");
    assert_eq!(engine.reason("What is the capital of France?"), Some("Paris"));
    assert_eq!(engine.reason("What is the capital of Germany?"), None);
    let reflected = engine.reflect();
    assert_eq!(reflected.len(), 1);
    assert_eq!(reflected[0].fact, "capital of France");
    assert_eq!(reflected[0].value, "Paris");
    let hypotheses = engine.generate_hypotheses("capital of France");
    assert_eq!(hypotheses, vec!["Paris is the capital of France"]);
    assert!(engine.verify("Paris is the capital of France"));
}

#[test]
fn test_reasoning_engine_generate_hypotheses_from_fact() {
    let mut engine = ReasoningEngine::new();
    engine.add_fact("capital of France", "Paris");

    let hypotheses = engine.generate_hypotheses("capital of France");
    assert_eq!(hypotheses, vec!["Paris is the capital of France"]);
}

#[test]
fn test_reasoning_engine_generate_hypotheses_from_question() {
    let mut engine = ReasoningEngine::new();
    engine.add_fact("capital of France", "Paris");

    let hypotheses = engine.generate_hypotheses("What is the capital of France?");
    assert_eq!(hypotheses, vec!["Paris is the capital of France"]);
}
