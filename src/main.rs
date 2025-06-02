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
    #[clap(long, value_parser)]
    redis_url: String,
    #[clap(long, value_parser, default_value = "sqs")]
    mode: mode::Mode,
    #[clap(value_parser)]
    channel: String,
    #[clap(value_parser)]
    lambda: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    sub::subscribe(&args.redis_url, &args.channel, move |body| {
        let event = match args.mode {
            mode::Mode::Sqs => mode::SqsEvent::new(body),
        };
        let event_json = serde_json::to_string(&event).unwrap();
        match invoke::invoke(&args.lambda, &event_json) {
            Ok(result) => {
                println!("success: {}", result.success);
            }
            Err(e) => {
                eprintln!("Failed to invoke lambda: {}", e);
            }
        }
    })?;
    Ok(())
}
