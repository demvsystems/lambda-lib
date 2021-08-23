use rusoto_sqs::SqsClient;

pub struct Queue {
    pub client: SqsClient,
}

impl Queue {
    async fn send_message<S: Into<String>>(&self, message: S, url: S) -> Result<(), String> {
        use rusoto_sqs::{SendMessageRequest, Sqs};

        let mut req = SendMessageRequest::default();
        req.message_body = message.into();
        req.queue_url = url.into();

        match self.client.send_message(req).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Die Nachricht konnte nicht synchronisiert werden: {:?}",
                e
            )),
        }
    }

    pub async fn send_serialized<T: ?Sized, S: Into<String>>(
        &self,
        value: &T,
        url: S,
    ) -> Result<(), String>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        match serde_json::to_string(&value) {
            Ok(json) => self.send_message(json, url.into()).await,
            Err(e) => Err(format!(
                "Das Objekt {:?} konnte nicht serialisiert werden: {:?}",
                value, e
            )),
        }
    }
}
