# srill

Subscribe Redis and Invoke Lambda with cargo lambda, for Local development.

## Features

- Subscribe to multiple Redis channels simultaneously
- Invoke different Lambda functions for each channel
- Support for configuration files (TOML format)
- Backward compatibility with single channel mode
- Parallel processing of multiple channels

## Usage

First, run cargo lambda watch:
```sh
cargo lambda watch
```

### Multiple Channels (Recommended)

Start srill with multiple channel-lambda pairs:
```sh
srill --channels channel1=lambda-function1,channel2=lambda-function2,channel3=lambda-function3
```

### Configuration File

Create a `srill.toml` configuration file:
```toml
redis_url = "redis://localhost:6379"
mode = "sqs"

[channels]
channel1 = "lambda-function1"
channel2 = "lambda-function2"
channel3 = "lambda-function3"
```

Then start srill:
```sh
srill --config srill.toml
```

### Legacy Single Channel Mode

Start srill (legacy mode):
```sh
srill <channel name> <lambda binary name>
```

### Publishing Messages

Then publish a message to redis:
```sh
redis-cli publish <channel name> "Test message."
```

The corresponding lambda function will be invoked with SQS event.

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
            "awsRegion": "ap-northeast-1"
        }
    ]
}
```

### Options

- `--redis-url`: Redis URL (default: `redis://localhost:6379`)
- `--mode`: Lambda event type (default: `sqs`)
- `--channels`: Channel-Lambda pairs in format `channel1=lambda1,channel2=lambda2`
- `--config`: Path to TOML configuration file

### Examples

```sh
# Multiple channels via command line
srill --redis-url redis://localhost:6379 --mode sqs --channels user-events=user-lambda,order-events=order-lambda

# Using configuration file
srill --config ./config/srill.toml

# Legacy single channel (backward compatibility)
srill my-channel my-lambda
```

## License

MIT
