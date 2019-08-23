use crate::queue::Queue;
use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::error::HandlerError;
#[cfg(feature = "s3")]
use aws_lambda_events::event::s3::S3Event;
#[cfg(feature = "s3")]
use crate::aws::s3::S3Bucket;

pub struct Lambda {
    queue: Queue
}

pub trait LambdaHandle {
    fn handle_sqs(&self, event: SqsEvent) -> Result<(), HandlerError> {
        log::info!("Eingehendes SQS-Event: {:?}", &event);
        for record in event.records {
            if let Some(body) = record.body {
                self.handle_sqs_record(&body);
            }
        }

        Ok(())
    }

    fn handle_sqs_record(&self, body: &str);

    #[cfg(feature = "s3")]
    fn handle_s3(&self, event: S3Event) -> Result<(), HandlerError> {
        log::info!("Eingehendes S3-Event: {:?}", &event);
        for record in event.records {
            match S3Bucket::from_record(&record) {
                Err(e) => log::error!("Fehler beim umwandeln des S3-Buckets: {:?}", e),
                Ok(s3_bucket) => self.handle_s3_record(s3_bucket),
            }
        }

        Ok(())
    }

    #[cfg(feature = "s3")]
    fn handle_s3_record(&self, bucket: S3Bucket);
}