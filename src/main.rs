use clap::Parser;
use std::collections::HashMap;

mod invoke;
mod mode;
mod sub;

#[derive(Debug, clap::Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Args {
    /// Redis URL
    #[clap(long, value_parser, default_value = "redis://localhost:6379")]
    redis_url: String,
    /// Type of lambda event.
    #[clap(long, value_parser, default_value = "sqs")]
    mode: mode::Mode,
    /// Channel-Lambda pairs in format "channel1=lambda1,channel2=lambda2"
    #[clap(long, value_parser, value_delimiter = ',')]
    channels: Option<Vec<String>>,
    /// Config file path (TOML format)
    #[clap(long, value_parser)]
    config: Option<String>,
    /// Redis pub/sub channel name (legacy single channel mode)
    #[clap(value_parser)]
    channel: Option<String>,
    /// Lambda function name (legacy single channel mode)
    #[clap(value_parser)]
    lambda: Option<String>,
}

#[derive(Debug, Clone)]
struct ChannelLambdaPair {
    channel: String,
    lambda: String,
}

#[derive(Debug, serde::Deserialize)]
struct Config {
    redis_url: Option<String>,
    mode: Option<String>,
    channels: HashMap<String, String>,
}

fn parse_channel_pairs(channels: &[String]) -> anyhow::Result<Vec<ChannelLambdaPair>> {
    let mut pairs = Vec::new();
    for channel_pair in channels {
        let parts: Vec<&str> = channel_pair.split('=').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid channel-lambda pair format: '{}'. Expected 'channel=lambda'",
                channel_pair
            ));
        }
        pairs.push(ChannelLambdaPair {
            channel: parts[0].to_string(),
            lambda: parts[1].to_string(),
        });
    }
    Ok(pairs)
}

fn load_config(config_path: &str) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Determine channel-lambda pairs from various sources
    let (pairs, redis_url, mode) = get_channel_lambda_pairs(&args).await?;

    if pairs.is_empty() {
        return Err(anyhow::anyhow!(
            "No channel-lambda pairs specified. Use --channels, --config, or legacy channel/lambda arguments."
        ));
    }

    println!("Starting srill with {} channel(s):", pairs.len());
    for pair in &pairs {
        println!("  {} => {}", pair.channel, pair.lambda);
    }

    // Start subscription tasks for each channel-lambda pair
    let mut tasks = Vec::new();

    for pair in pairs {
        let redis_url = redis_url.clone();
        let mode = mode.clone();
        let task = tokio::spawn(async move {
            let prompt = format!(
                "[srill {} ==({})==> {}]",
                &pair.channel, &mode, &pair.lambda
            );

            if let Err(e) = sub::subscribe(&redis_url, &pair.channel, move |body| {
                // Parse Redis message for message_attributes
                let (parsed_body, message_attributes) =
                    match serde_json::from_str::<serde_json::Value>(body) {
                        Ok(serde_json::Value::Object(obj)) => {
                            // Extract message_attributes
                            let attrs = obj
                                .get("message_attributes")
                                .and_then(|v| serde_json::from_value(v.clone()).ok())
                                .unwrap_or_default();
                            // Keep original JSON as body
                            // Safe to unwrap since from_str was successful
                            let body = serde_json::to_string(&obj).unwrap();
                            (body, attrs)
                        }
                        // Not JSON object
                        _ => (body.to_string(), std::collections::HashMap::new()),
                    };

                let event = match mode {
                    mode::Mode::Sqs => mode::SqsEvent::new(&parsed_body, message_attributes),
                };
                let event_json = serde_json::to_string(&event).unwrap();
                match invoke::invoke(&pair.lambda, &event_json) {
                    Ok(result) => {
                        if result.success {
                            println!("{prompt} success");
                        } else {
                            println!("{prompt} failed");
                        }
                    }
                    Err(e) => {
                        eprintln!("{prompt} Failed to invoke lambda: {e}");
                    }
                }
            }) {
                eprintln!("Failed to subscribe to channel {}: {}", pair.channel, e);
            }
        });
        tasks.push(task);
    }

    // Wait for all tasks to complete (they should run indefinitely)
    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Task failed: {e}");
        }
    }

    Ok(())
}

async fn get_channel_lambda_pairs(
    args: &Args,
) -> anyhow::Result<(Vec<ChannelLambdaPair>, String, mode::Mode)> {
    let mut pairs = Vec::new();
    let mut redis_url = args.redis_url.clone();
    let mut mode = args.mode.clone();

    // Priority 1: Config file
    if let Some(config_path) = &args.config {
        let config = load_config(config_path)?;

        // Override redis_url and mode if specified in config
        if let Some(config_redis_url) = config.redis_url {
            redis_url = config_redis_url;
        }
        if let Some(config_mode) = config.mode {
            mode = match config_mode.as_str() {
                "sqs" => mode::Mode::Sqs,
                _ => {
                    return Err(anyhow::anyhow!(
                        "Unsupported mode in config: {}",
                        config_mode
                    ));
                }
            };
        }

        for (channel, lambda) in config.channels {
            pairs.push(ChannelLambdaPair { channel, lambda });
        }
        return Ok((pairs, redis_url, mode));
    }

    // Priority 2: --channels argument
    if let Some(channels) = &args.channels {
        let pairs = parse_channel_pairs(channels)?;
        return Ok((pairs, redis_url, mode));
    }

    // Priority 3: Legacy single channel/lambda arguments
    if let (Some(channel), Some(lambda)) = (&args.channel, &args.lambda) {
        pairs.push(ChannelLambdaPair {
            channel: channel.clone(),
            lambda: lambda.clone(),
        });
        return Ok((pairs, redis_url, mode));
    }

    Ok((pairs, redis_url, mode))
}
