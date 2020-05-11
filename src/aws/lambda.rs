#[cfg(feature = "s3")]
use crate::aws::s3::S3Bucket;
#[cfg(feature = "s3")]
use aws_lambda_events::event::s3::S3Event;
#[cfg(feature = "sqs")]
use aws_lambda_events::event::sqs::SqsEvent;
use std::error::Error;

type HandlerError = Box<dyn Error + Send + Sync + 'static>;

#[cfg(feature = "sqs")]
pub trait SqsHandle {
    fn handle_sqs_event(&self, event: SqsEvent) -> Result<(), HandlerError> {
        log::info!("Eingehendes SQS-Event: {:?}", &event);
        for record in event.records {
            if let Some(body) = record.body {
                self.handle_sqs_record(&body);
            }
        }

        Ok(())
    }

    fn handle_sqs_record(&self, body: &str);
}

#[cfg(feature = "s3")]
pub trait S3Handle {
    fn handle_s3_event(&self, event: S3Event) -> Result<(), HandlerError> {
        log::info!("Eingehendes S3-Event: {:?}", &event);
        for record in event.records {
            match S3Bucket::from_record(&record) {
                Err(e) => log::error!("Fehler beim umwandeln des S3-Buckets: {:?}", e),
                Ok(s3_bucket) => self.handle_s3_record(s3_bucket),
            }
        }

        Ok(())
    }

    fn handle_s3_record(&self, bucket: S3Bucket);
}
