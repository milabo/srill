use clap::Parser;

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
    /// Redis pub/sub channel name.
    #[clap(value_parser)]
    channel: String,
    /// Lambda function name.
    #[clap(value_parser)]
    lambda: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let lambda = args.lambda.clone();
    let prompt = format!(
        "[srill {} ==({})==> {}]",
        &args.channel, &args.mode, &lambda
    );
    sub::subscribe(&args.redis_url, &args.channel, move |body| {
        let event = match args.mode {
            mode::Mode::Sqs => mode::SqsEvent::new(body),
        };
        let event_json = serde_json::to_string(&event).unwrap();
        match invoke::invoke(&lambda, &event_json) {
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
    })?;
    Ok(())
}
