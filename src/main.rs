use evo_agent_sdk::{AgentRunner, kernel_handlers::SkillManageHandler};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    AgentRunner::run(SkillManageHandler).await
}
