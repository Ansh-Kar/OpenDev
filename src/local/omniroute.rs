//! OmniRoute AI Gateway Client for OpenDev.
//!
//! Provides typed access to OmniRoute's 356+ providers, model tagging,
//! DAG wave orchestrator, shared blackboard, and token-compressed inference.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::error::{anyhow, Result};

pub const DEFAULT_OMNIROUTE_URL: &str = "http://127.0.0.1:20128/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniRouteConfig {
    pub base_url: String,
    pub api_key: Option<String>,
}

impl Default for OmniRouteConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("OMNIROUTE_API_URL")
                .unwrap_or_else(|_| DEFAULT_OMNIROUTE_URL.to_string()),
            api_key: std::env::var("OMNIROUTE_API_KEY").ok(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickTaskRequest {
    pub tag: String, // "code", "plan", "reasoning", "vision", "chat"
    pub prompt: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<OmniRoutePolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OmniRoutePolicy {
    pub budget: Option<String>, // "cheap", "any", "best"
    pub max_attempts: Option<u32>,
    pub max_concurrency: Option<usize>,
    pub deadline_s: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickTaskResponse {
    pub ok: bool,
    pub text: Option<String>,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub score: Option<f64>,
    pub error: Option<String>,
}

pub struct OmniRouteClient {
    config: OmniRouteConfig,
    http: reqwest::Client,
}

impl OmniRouteClient {
    pub fn new(config: OmniRouteConfig) -> Self {
        Self {
            config,
            http: reqwest::Client::new(),
        }
    }

    /// Execute a quick single-step capability task through OmniRoute.
    pub async fn quick_task(&self, req: QuickTaskRequest) -> Result<QuickTaskResponse> {
        let url = format!("{}/orchestrate/quick", self.config.base_url);
        let mut builder = self.http.post(&url).json(&req);
        if let Some(key) = &self.config.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", key));
        }
        builder = builder.header("Idempotency-Key", uuid::Uuid::new_v4().to_string());

        let res = builder.send().await
            .map_err(|e| anyhow!("OmniRoute connection failed: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(anyhow!("OmniRoute error ({}): {}", status, body));
        }

        let resp: QuickTaskResponse = res.json().await
            .map_err(|e| anyhow!("Failed to parse OmniRoute response: {}", e))?;
        Ok(resp)
    }

    /// Submit a multi-task DAG plan to OmniRoute's orchestrator.
    pub async fn submit_plan(&self, plan: Value) -> Result<String> {
        let url = format!("{}/orchestrate/plan", self.config.base_url);
        let mut builder = self.http.post(&url).json(&plan);
        if let Some(key) = &self.config.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", key));
        }
        builder = builder.header("Idempotency-Key", uuid::Uuid::new_v4().to_string());

        let res = builder.send().await
            .map_err(|e| anyhow!("OmniRoute connection failed: {}", e))?;

        let body: Value = res.json().await
            .map_err(|e| anyhow!("Failed to parse OmniRoute plan response: {}", e))?;

        let job_id = body["job_id"]
            .as_str()
            .ok_or_else(|| anyhow!("OmniRoute did not return a job_id: {:?}", body))?
            .to_string();

        Ok(job_id)
    }

    /// Poll job status from OmniRoute.
    pub async fn get_job_status(&self, job_id: &str) -> Result<Value> {
        let url = format!("{}/orchestrate/jobs/{}?wait=5", self.config.base_url, job_id);
        let mut builder = self.http.get(&url);
        if let Some(key) = &self.config.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", key));
        }

        let res = builder.send().await
            .map_err(|e| anyhow!("OmniRoute poll error: {}", e))?;

        let body: Value = res.json().await
            .map_err(|e| anyhow!("Failed to parse job status: {}", e))?;

        Ok(body)
    }
}
