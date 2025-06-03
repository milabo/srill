# srill

Subscribe Redis and Invoke Lambda with cargo lambda, for Local development.


## Usage

First, run cargo lambda watch:
```sh
cargo lambda watch
```

Start srill:
```sh
srill <channel name> <lambda binary name>
```

Then publish a message to redis:
```sh
redis-cli publish <channel name> "Test message."
```

The lambda function will be invoked with SQS event.

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

- `--redis-url`: Redis URL
    - default: `redis://localhost:6379`
- `--mode`: Type of Lambda event
    - default: `sqs`
    - available:
        - `sqs` (SQS event)

## License

MIT
