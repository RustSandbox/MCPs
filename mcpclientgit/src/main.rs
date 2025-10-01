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
    eprintln!("Ex-01: Git");

    let bedrock_client = rig_bedrock::client::Client::from_env();
    let agent = bedrock_client.agent(ANTHROPIC_CLAUDE_3_HAIKU).preamble("You are git expert");
    let git_command = Command::new(
        "/Users/hamzeghalebi/projects/tools/github-mcp-server/github-mcp-server" // you should change this to your local path og github mcp server binary file
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
/*
out come:
Ex-01: Git
time=2025-10-01T16:55:41.099+02:00 level=INFO msg="starting server" version=version host="" dynamicToolsets=false readOnly=false
GitHub MCP Server running on stdio
service is RunningService { service: (), peer: PeerSink { tx: Sender { chan: Tx { inner: Chan { tx: Tx { block_tail: 0x15400dc00, tail_position: 0 }, semaphore: Semaphore { semaphore: Semaphore { permits: 1024 }, bound: 1024 }, rx_waker: AtomicWaker, tx_count: 2, rx_fields: "..." } } }, is_client: true }, handle: JoinHandle { id: Id(17) }, cancellation_token: CancellationToken { is_cancelled: false }, dg: DropGuard { inner: Some(CancellationToken { is_cancelled: false }) } }




service_info is Some(InitializeResult { protocol_version: ProtocolVersion("2025-03-26"), capabilities: ServerCapabilities { experimental: None, logging: Some({}), completions: None, prompts: Some(PromptsCapability { list_changed: None }), resources: Some(ResourcesCapability { subscribe: Some(true), list_changed: Some(true) }), tools: Some(ToolsCapability { list_changed: Some(true) }) }, server_info: Implementation { name: "github-mcp-server", title: None, version: "version", icons: None, website_url: None }, instructions: None })
Starting task execution...
Task completed! Reply: I first searched for repositories owned by the user 'hghalebi' and got a list of 88 repositories. I then chose to create a test issue in the repository named 'PropagandaTracker' with the title 'Test Issue from Rust MCP Client' and the body 'This is a funny test issue created automatically. Please ignore!'. The new issue was created successfully and the URL of the issue is returned.


 */