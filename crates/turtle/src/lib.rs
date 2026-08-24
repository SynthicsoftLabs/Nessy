// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use bowser_core::{SessionId, Task, TaskId, TaskState};
use std::collections::{HashMap, VecDeque};
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum TurtleError {
    #[error("task {0:?} not found")]
    TaskNotFound(TaskId),
    #[error("task {0:?} is not runnable")]
    InvalidTransition(TaskId),
}

#[derive(Default)]
pub struct Scheduler {
    tasks: Mutex<HashMap<TaskId, Task>>,
    queue: Mutex<VecDeque<TaskId>>,
}

impl Scheduler {
    pub async fn enqueue(&self, session_id: SessionId, input: serde_json::Value) -> TaskId {
        let task = Task { id: TaskId::new(), session_id, state: TaskState::Queued, input };
        let id = task.id;
        self.tasks.lock().await.insert(id, task);
        self.queue.lock().await.push_back(id);
        id
    }

    pub async fn claim(&self) -> Result<Option<Task>, TurtleError> {
        let id = self.queue.lock().await.pop_front();
        let Some(id) = id else { return Ok(None); };
        let mut tasks = self.tasks.lock().await;
        let task = tasks.get_mut(&id).ok_or(TurtleError::TaskNotFound(id))?;
        if task.state != TaskState::Queued {
            return Err(TurtleError::InvalidTransition(id));
        }
        task.state = TaskState::Running;
        Ok(Some(task.clone()))
    }

    pub async fn complete(&self, id: TaskId) -> Result<(), TurtleError> {
        self.transition(id, TaskState::Completed).await
    }

    pub async fn fail(&self, id: TaskId) -> Result<(), TurtleError> {
        self.transition(id, TaskState::Failed).await
    }

    async fn transition(&self, id: TaskId, next: TaskState) -> Result<(), TurtleError> {
        let mut tasks = self.tasks.lock().await;
        let task = tasks.get_mut(&id).ok_or(TurtleError::TaskNotFound(id))?;
        if task.state != TaskState::Running {
            return Err(TurtleError::InvalidTransition(id));
        }
        task.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn queue_claim_and_complete() {
        let scheduler = Scheduler::default();
        let session = SessionId::new();
        let id = scheduler.enqueue(session, serde_json::json!({"prompt":"hello"})).await;
        let task = scheduler.claim().await.unwrap().unwrap();
        assert_eq!(task.id, id);
        assert_eq!(task.state, TaskState::Running);
        scheduler.complete(id).await.unwrap();
        assert!(scheduler.claim().await.unwrap().is_none());
    }
}
