// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use crate::agi::{AgiCapability, CapabilityGraph};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub objective: String,
    pub required: Vec<AgiCapability>,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub goal_id: Uuid,
    pub statement: String,
    pub assumptions: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: Uuid,
    pub capability: AgiCapability,
    pub provider: String,
    pub objective: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub goal_id: Uuid,
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub step_id: Uuid,
    pub result: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousCycle {
    pub goal: Goal,
    pub thought: Thought,
    pub plan: Plan,
    pub observations: Vec<Observation>,
}

#[derive(Debug, Default)]
pub struct Director;

impl Director {
    pub fn think(&self, goal: &Goal) -> Thought {
        Thought {
            goal_id: goal.id,
            statement: format!("Decompose objective into executable capabilities: {}", goal.objective),
            assumptions: vec!["Requested capabilities are the planning contract.".into()],
            evidence: Vec::new(),
        }
    }

    pub fn plan(&self, goal: &Goal, graph: &CapabilityGraph) -> Plan {
        let selected = graph.compose(&goal.required);
        let steps = selected
            .into_iter()
            .map(|node| PlanStep {
                id: Uuid::new_v4(),
                capability: node.capability.clone(),
                provider: node.provider.clone(),
                objective: goal.objective.clone(),
            })
            .collect();
        Plan { goal_id: goal.id, steps }
    }

    pub fn evaluate(&self, plan: &Plan, observations: &[Observation]) -> bool {
        let completed: HashSet<Uuid> = observations
            .iter()
            .filter(|observation| observation.success)
            .map(|observation| observation.step_id)
            .collect();
        !plan.steps.is_empty() && plan.steps.iter().all(|step| completed.contains(&step.id))
    }

    pub fn next_cycle(&self, cycle: &AutonomousCycle) -> Goal {
        let completed = cycle.observations.iter().filter(|o| o.success).count();
        Goal {
            id: Uuid::new_v4(),
            objective: if completed == cycle.plan.steps.len() {
                format!("Advance the objective beyond completed state: {}", cycle.goal.objective)
            } else {
                format!("Recover and continue objective: {}", cycle.goal.objective)
            },
            required: cycle.goal.required.clone(),
            priority: cycle.goal.priority,
        }
    }

    pub fn cycle(&self, goal: Goal, graph: &CapabilityGraph) -> AutonomousCycle {
        let thought = self.think(&goal);
        let plan = self.plan(&goal, graph);
        AutonomousCycle { goal, thought, plan, observations: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agi::CapabilityNode;
    use std::collections::HashSet;

    #[test]
    fn director_thinks_plans_and_evaluates() {
        let mut graph = CapabilityGraph::default();
        graph.register(CapabilityNode {
            id: "reasoner".into(),
            capability: AgiCapability::Reasoning,
            provider: "frontier".into(),
            version: "1".into(),
            inputs: vec!["objective".into()],
            outputs: vec!["reasoned_result".into()],
            prerequisites: HashSet::new(),
            quality: 1.0,
            latency_ms: 1,
            healthy: true,
        });
        let goal = Goal { id: Uuid::new_v4(), objective: "solve the task".into(), required: vec![AgiCapability::Reasoning], priority: 1 };
        let director = Director;
        let cycle = director.cycle(goal, &graph);
        assert!(!cycle.thought.statement.is_empty());
        assert_eq!(cycle.plan.steps.len(), 1);
        let observation = Observation { step_id: cycle.plan.steps[0].id, result: "complete".into(), success: true };
        assert!(director.evaluate(&cycle.plan, &[observation]));
    }
}
