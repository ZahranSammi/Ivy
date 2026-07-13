use std::time::Duration;

use ivy_core::traits::tool_executor::ToolExecutor;
use serde::{Deserialize, Serialize};

use super::planner::ExecutionPlan;

/// FR-009 default per-tool timeout.
// ponytail: one global timeout, not a per-tool override map — add per-tool
// values once `McpToolSchema`/tool.json actually carries a timeout field.
pub const DEFAULT_TOOL_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StepStatus {
    Completed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepResult {
    pub order: u32,
    pub tool: String,
    pub status: StepStatus,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
}

pub struct Executor;

impl Executor {
    /// Run every step of `plan` in order against `tool_executor`.
    ///
    /// A failing step (error or timeout) is retried once, then skipped so the rest of
    /// the plan keeps going (FR-009 error handling). Docker isolation and WebSocket
    /// streaming happen inside the `ToolExecutor` implementation itself (ivy-docker /
    /// ivy-server) — this loop only sequences calls to it.
    pub async fn run(
        tool_executor: &dyn ToolExecutor,
        plan: &ExecutionPlan,
        timeout: Duration,
    ) -> Vec<StepResult> {
        let mut results = Vec::with_capacity(plan.steps.len());
        for step in &plan.steps {
            let outcome =
                match run_step(tool_executor, &step.tool, step.parameters.clone(), timeout).await {
                    Ok(output) => Ok(output),
                    Err(_) => {
                        run_step(tool_executor, &step.tool, step.parameters.clone(), timeout).await
                    }
                };

            results.push(match outcome {
                Ok(output) => StepResult {
                    order: step.order,
                    tool: step.tool.clone(),
                    status: StepStatus::Completed,
                    output: Some(output),
                    error: None,
                },
                Err(e) => StepResult {
                    order: step.order,
                    tool: step.tool.clone(),
                    status: StepStatus::Skipped,
                    output: None,
                    error: Some(e.to_string()),
                },
            });
        }
        results
    }
}

async fn run_step(
    tool_executor: &dyn ToolExecutor,
    tool: &str,
    parameters: serde_json::Value,
    timeout: Duration,
) -> Result<serde_json::Value, anyhow::Error> {
    tokio::time::timeout(timeout, tool_executor.run_tool(tool, parameters))
        .await
        .map_err(|_| anyhow::anyhow!("tool '{tool}' timed out after {timeout:?}"))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestrator::planner::PlannedStep;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn plan_with(steps: Vec<PlannedStep>) -> ExecutionPlan {
        ExecutionPlan { steps }
    }

    fn step(order: u32, tool: &str) -> PlannedStep {
        PlannedStep {
            order,
            tool: tool.to_string(),
            parameters: serde_json::json!({}),
            depends_on: vec![],
        }
    }

    struct FlakyExecutor {
        // ponytail: fails-then-succeeds keyed on call count, not per-tool state —
        // fine for a single-tool test double.
        calls: AtomicU32,
        fail_first_n: u32,
    }

    #[async_trait]
    impl ToolExecutor for FlakyExecutor {
        async fn run_tool(
            &self,
            tool_id: &str,
            _parameters: serde_json::Value,
        ) -> Result<serde_json::Value, anyhow::Error> {
            let n = self.calls.fetch_add(1, Ordering::SeqCst);
            if n < self.fail_first_n {
                anyhow::bail!("simulated failure for {tool_id}");
            }
            Ok(serde_json::json!({ "tool": tool_id }))
        }

        async fn stop_tool(&self, _execution_id: &str) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn run_succeeds_on_first_try() {
        let exec = FlakyExecutor {
            calls: AtomicU32::new(0),
            fail_first_n: 0,
        };
        let plan = plan_with(vec![step(1, "amass")]);

        let results = Executor::run(&exec, &plan, Duration::from_secs(1)).await;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, StepStatus::Completed);
        assert_eq!(exec.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn run_retries_once_then_succeeds() {
        let exec = FlakyExecutor {
            calls: AtomicU32::new(0),
            fail_first_n: 1,
        };
        let plan = plan_with(vec![step(1, "amass")]);

        let results = Executor::run(&exec, &plan, Duration::from_secs(1)).await;

        assert_eq!(results[0].status, StepStatus::Completed);
        assert_eq!(exec.calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn run_skips_after_second_failure_and_continues_to_next_step() {
        let exec = FlakyExecutor {
            calls: AtomicU32::new(0),
            fail_first_n: 99,
        };
        let plan = plan_with(vec![step(1, "amass"), step(2, "subfinder")]);

        let results = Executor::run(&exec, &plan, Duration::from_secs(1)).await;

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].status, StepStatus::Skipped);
        assert!(results[0].error.is_some());
        assert_eq!(results[1].status, StepStatus::Skipped);
        // both steps attempted: 2 tries each = 4 calls total
        assert_eq!(exec.calls.load(Ordering::SeqCst), 4);
    }

    struct SlowExecutor;

    #[async_trait]
    impl ToolExecutor for SlowExecutor {
        async fn run_tool(
            &self,
            _tool_id: &str,
            _parameters: serde_json::Value,
        ) -> Result<serde_json::Value, anyhow::Error> {
            tokio::time::sleep(Duration::from_millis(200)).await;
            Ok(serde_json::json!({}))
        }

        async fn stop_tool(&self, _execution_id: &str) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn run_times_out_and_skips() {
        let exec = SlowExecutor;
        let plan = plan_with(vec![step(1, "nmap")]);

        let results = Executor::run(&exec, &plan, Duration::from_millis(10)).await;

        assert_eq!(results[0].status, StepStatus::Skipped);
        assert!(results[0].error.as_ref().unwrap().contains("timed out"));
    }
}
