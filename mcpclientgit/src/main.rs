use anyhow::Result;
use rmcp::service::ServiceExt;
use rmcp::transport::{ConfigureCommandExt, TokioChildProcess};
use tokio::process::Command;
use rig_bedrock::completion::ANTHROPIC_CLAUDE_3_HAIKU;
use rig::client::completion::CompletionClientDyn;
use rig::client::ProviderClient;
use rig::completion::Prompt;
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let bedrock_client = rig_bedrock::client::Client::from_env();
    let agent = bedrock_client.agent(ANTHROPIC_CLAUDE_3_HAIKU).preamble("You are git expert");
    let git_command = Command::new(
        "/Users/hamzeghalebi/projects/tools/github-mcp-server/github-mcp-server"
    ).configure(|cmd| {
        cmd.arg("stdio");
        cmd.arg("all");
    });

    let tokio_child = TokioChildProcess::new(git_command)?;
    let service = ().serve(tokio_child).await?;
    let service_info = service.peer_info();

    eprintln!("service is {:?}", service);
    eprintln!("\n\n\n\nservice_info is {:?}", service_info);

    let tools = service.list_tools(Default::default()).await?.tools;

    let agent = tools
        .into_iter()
        .fold(agent, |agent, tool| {
            agent.rmcp_tool(tool, service.clone())
        })
        .build();

    let task = "Search for repositories owned by user 'hghalebi'. Then create a test issue in one of them with title 'Test Issue from Rust MCP Client' and body 'This is a funny test issue created automatically. Please ignore!'";

    eprintln!("Starting task execution...");
    let reply = agent.prompt(task)
        .multi_turn(10)
        .await?;
    eprintln!("Task completed! Reply: {}", reply);

    Ok(())
}
