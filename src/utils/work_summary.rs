use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct WorkSummary {
    category: String,
    project: String,
    task: String,
    summary: String,
}

impl WorkSummary {
    pub fn new(category: String, project: String, task: String, summary: String) -> Self {
        return WorkSummary {
            category: category,
            project: project,
            task: task,
            summary: summary,
        };
    }
}
