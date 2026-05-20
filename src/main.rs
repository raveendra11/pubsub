mod publisher;
mod subscriber;
mod config;
mod error;

use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <publish|subscribe> [options]", args[0]);
        println!("\nExamples:");
        println!("  {} publish <project-id> <topic-id>", args[0]);
        println!("  {} subscribe <project-id> <subscription-id>", args[0]);
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "publish" => {
            if args.len() < 4 {
                eprintln!("Usage: {} publish <project-id> <topic-id>", args[0]);
                return Ok(());
            }
            let project_id = &args[2];
            let topic_id = &args[3];
            info!("Publishing to topic: projects/{}/topics/{}", project_id, topic_id);
            publisher::publish_message(project_id, topic_id).await?;
        }
        "subscribe" => {
            if args.len() < 4 {
                eprintln!("Usage: {} subscribe <project-id> <subscription-id>", args[0]);
                return Ok(());
            }
            let project_id = &args[2];
            let subscription_id = &args[3];
            info!(
                "Subscribing to: projects/{}/subscriptions/{}",
                project_id, subscription_id
            );
            subscriber::subscribe_to_messages(project_id, subscription_id).await?;
        }
        _ => {
            eprintln!("Unknown command: {}. Use 'publish' or 'subscribe'", command);
        }
    }

    Ok(())
}
