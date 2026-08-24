// Copyright 2026 Adam Joseph Rivers, CEO, Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CognitivePhase {
    Orient,
    Reason,
    Plan,
    Execute,
    Observe,
    Evaluate,
    Learn,
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveGoal {
    pub id: Uuid,
    pub objective: String,
    pub priority: i32,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub session_id: Uuid,
    pub phase: CognitivePhase,
    pub goals: Vec<CognitiveGoal>,
    pub working_memory: BTreeMap<String, String>,
    pub observations: Vec<String>,
    pub learned_skills: Vec<String>,
    pub cycle: u64,
}

impl CognitiveState {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            phase: CognitivePhase::Orient,
            goals: Vec::new(),
            working_memory: BTreeMap::new(),
            observations: Vec::new(),
            learned_skills: Vec::new(),
            cycle: 0,
        }
    }

    pub fn add_goal(&mut self, objective: impl Into<String>, priority: i32) -> Uuid {
        let goal = CognitiveGoal {
            id: Uuid::new_v4(),
            objective: objective.into(),
            priority,
            completed: false,
        };
        let id = goal.id;
        self.goals.push(goal);
        id
    }

    pub fn next_goal(&self) -> Option<&CognitiveGoal> {
        self.goals
            .iter()
            .filter(|goal| !goal.completed)
            .max_by_key(|goal| goal.priority)
    }

    pub fn transition(&mut self, phase: CognitivePhase) {
        self.phase = phase;
        self.cycle = self.cycle.saturating_add(1);
    }

    pub fn observe(&mut self, observation: impl Into<String>) {
        self.observations.push(observation.into());
    }

    pub fn learn_skill(&mut self, skill: impl Into<String>) {
        let skill = skill.into();
        if !self.learned_skills.contains(&skill) {
            self.learned_skills.push(skill);
        }
    }

    pub fn complete_goal(&mut self, id: Uuid) -> bool {
        if let Some(goal) = self.goals.iter_mut().find(|goal| goal.id == id) {
            goal.completed = true;
            true
        } else {
            false
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.goals.is_empty() && self.goals.iter().all(|goal| goal.completed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cognitive_state_persists_goal_progress() {
        let mut state = CognitiveState::new(Uuid::new_v4());
        let id = state.add_goal("build the next capability", 100);
        state.transition(CognitivePhase::Reason);
        state.observe("capability identified");
        state.learn_skill("capability-composition");
        assert_eq!(state.next_goal().unwrap().id, id);
        assert!(state.complete_goal(id));
        assert!(state.is_complete());
    }
}
