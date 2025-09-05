pub mod events {
    #[cfg(feature = "sqs")]
    pub mod sqs {
        pub use aws_lambda_events::event::sqs::*;
    }
}
