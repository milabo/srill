use std::collections::HashMap;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Mode {
    Sqs,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode_str = match self {
            Mode::Sqs => "sqs",
        };
        write!(f, "{mode_str}")
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SqsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SqsRecord>,
}

impl SqsEvent {
    pub fn new(body: &str, message_attributes: HashMap<String, MessageAttribute>) -> Self {
        Self {
            records: vec![SqsRecord::new(body, message_attributes)],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SqsRecord {
    pub message_id: String,
    pub receipt_handle: String,
    pub body: String,
    pub md5_of_body: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    pub event_source: String,
    pub aws_region: String,
    pub attributes: SqsRecordAttributes,
    pub message_attributes: HashMap<String, MessageAttribute>,
}

impl SqsRecord {
    pub fn new(body: &str, message_attributes: HashMap<String, MessageAttribute>) -> Self {
        let message_id = uuid::Uuid::new_v4().to_string();
        let receipt_handle = uuid::Uuid::new_v4().to_string();
        let md5_of_body = format!("{:x}", md5::compute(body));

        Self {
            message_id,
            receipt_handle,
            body: body.to_string(),
            md5_of_body,
            event_source_arn: "arn:aws:sqs:ap-northeast-1:123456789012:SQSQueue".to_string(),
            event_source: "aws:sqs".to_string(),
            aws_region: "ap-northeast-1".to_string(),
            attributes: SqsRecordAttributes::default(),
            message_attributes,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SqsRecordAttributes {
    pub approximate_receive_count: String,
    pub sender_id: String,
    pub sent_timestamp: String,
    pub approximate_first_receive_timestamp: String,
}

impl Default for SqsRecordAttributes {
    fn default() -> Self {
        Self {
            approximate_receive_count: "0".to_string(),
            sender_id: "sender".to_string(),
            sent_timestamp: "1520621625029".to_string(),
            approximate_first_receive_timestamp: "1520621634884".to_string(),
        }
    }
}

// reference: https://docs.aws.amazon.com/ja_jp/AWSSimpleQueueService/latest/APIReference/API_MessageAttributeValue.html
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageAttribute {
    pub data_type: MessageAttributeDataType,
    pub binary_list_values: Option<Vec<String>>,
    pub binary_value: Option<String>,
    pub string_list_values: Option<Vec<String>>,
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageAttributeDataType {
    String,
    Number,
    Binary,
    // TODO: Add Custom DataType
    // https://docs.aws.amazon.com/ja_jp/AWSSimpleQueueService/latest/APIReference/API_MessageAttributeValue.html#API_MessageAttributeValue_Contents
}
