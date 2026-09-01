use bowser_core::reasoning::{ReasoningEngine, ReasoningResult};

#[test]
fn test_reasoning_engine() {
    let engine = ReasoningEngine::new();
    let result = engine.reason("What is the capital of France?");
    assert_eq!(result, ReasoningResult::Success("Paris".to_string()));
}
