# srill

[![CI](https://github.com/milabo/srill/actions/workflows/ci.yml/badge.svg)](https://github.com/milabo/srill/actions/workflows/ci.yml)
[![crate](https://img.shields.io/crates/v/srill.svg)](https://crates.io/crates/srill)
[![documentation](https://docs.rs/srill/badge.svg)](https://docs.rs/srill)
[![minimum rustc 1.90](https://img.shields.io/badge/rustc-1.90+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

Subscribe Redis and Invoke Lambda with cargo lambda, for Local development.

## Features

- Subscribe to multiple Redis channels simultaneously
- Invoke different Lambda functions for each channel
- Support for configuration files (TOML format)

## Usage

First, run cargo lambda watch:

```sh
cargo lambda watch
```

If you change the watch port, configure the same port in `srill`:

```sh
cargo lambda watch --invoke-port 9011
srill --invoke-port 9011 --channels channel1=lambda-function1
```

### Multiple Channels

Start srill with multiple channel-lambda pairs:

```sh
srill --channels channel1=lambda-function1,channel2=lambda-function2,channel3=lambda-function3
```

### Configuration File

Create a `srill.toml` configuration file:

```toml
redis_url = "redis://localhost:6379"
invoke_port = 9000

[channels]
channel1 = "lambda-function1"
channel2 = "lambda-function2"
channel3 = "lambda-function3"
```

Then start srill:

```sh
srill --config srill.toml
```

### Single Channel Mode

Start srill:

```sh
srill <channel name> <lambda binary name>
```

## Publishing Messages

### Publishing SQS Events

Publishers should create and publish SQS event JSON:

```rust
use redis::Commands;
use srill::events::sqs::{SqsEvent, SqsMessage};
use serde_json;

fn publish_message(conn: &mut redis::Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Customize this event as you like.
    let event = SqsEvent {
        records: vec![SqsMessage {
            body: Some("Test message.".to_string()),
            ..Default::default()
        }],
    };

    let _: () = conn.publish("<channel_name>", &serde_json::to_string(&event)?)?;
    Ok(())
}
```

## Message Format

The Lambda function receives the complete SQS event as published to Redis:

```json
{
  "Records": [
    {
      "messageId": "<uuid-v4>",
      "receiptHandle": "<random string>",
      "body": "Test message.",
      "md5OfBody": "e62f489304eae26e9960977058872c3f",
      "attributes": {
        "ApproximateReceiveCount": "2",
        "SentTimestamp": "1520621625029",
        "SenderId": "sender",
        "ApproximateFirstReceiveTimestamp": "1520621634884"
      },
      "eventSourceARN": "arn:aws:sqs:ap-northeast-1:123456789012:SQSQueue",
      "eventSource": "aws:sqs",
      "awsRegion": "ap-northeast-1",
      "messageAttributes": {
        "Attribute3": {
          "binaryValue": "MTEwMA==",
          "stringListValues": ["abc", "123"],
          "binaryListValues": ["MA==", "MQ==", "MA=="],
          "dataType": "Binary"
        },
        "Attribute2": {
          "stringValue": "123",
          "stringListValues": [],
          "binaryListValues": ["MQ==", "MA=="],
          "dataType": "Number"
        },
        "Attribute1": {
          "stringValue": "AttributeValue1",
          "stringListValues": [],
          "binaryListValues": [],
          "dataType": "String"
        }
      }
    }
  ]
}
```

### Options

- `--redis-url`: Redis URL (default: `redis://localhost:6379`)
- `--invoke-port`: Port passed to `cargo lambda invoke` (default: `9000`)
- `--channels`: Channel-Lambda pairs in format `channel1=lambda1,channel2=lambda2`
- `--config`: Path to TOML configuration file

When both `--invoke-port` and `srill.toml` specify a port, the CLI option wins.

### Examples

```sh
# Multiple channels via command line
srill --redis-url redis://localhost:6379 --channels user-events=user-lambda,order-events=order-lambda

# Matching a non-default cargo lambda watch port
srill --invoke-port 9011 --channels user-events=user-lambda

# Using configuration file
srill --config ./config/srill.toml

# Single channel
srill my-channel my-lambda
```

## License

MIT
