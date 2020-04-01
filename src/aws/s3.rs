use aws_lambda_events::event::s3::S3EventRecord;

pub struct S3Bucket {
    pub name: String,
    pub object_key: String,
}

impl S3Bucket {
    pub fn from_record(record: &S3EventRecord) -> Result<Self, String> {
        if let Some(ref name) = record.s3.bucket.name {
            if let Some(ref object_key) = record.s3.object.key {
                Ok(Self {
                    name: name.to_owned(),
                    object_key: object_key.to_owned(),
                })
            } else {
                Err(String::from("Kein S3 Object-Key angegeben"))
            }
        } else {
            Err(String::from("Kein S3 Bucket-Name angegeben"))
        }
    }

    pub fn read_from(&self, client: &rusoto_s3::S3Client) -> Result<String, String> {
        use futures::executor::block_on;
        use rusoto_s3::S3;
        use std::io::Read;

        let mut request = rusoto_s3::GetObjectRequest::default();
        request.key = self.object_key.to_owned();
        request.bucket = self.name.to_owned();

        match block_on(client.get_object(request)) {
            Ok(output) => {
                if let Some(body) = output.body {
                    let mut buf = Vec::new();
                    body.into_blocking_read()
                        .read_to_end(&mut buf)
                        .expect("Could not read buffer");
                    if let Ok(json) = std::str::from_utf8(&buf) {
                        Ok(json.to_owned())
                    } else {
                        Err(format!("Invalider content: {:?}", buf))
                    }
                } else {
                    Err(String::from("Leerer body"))
                }
            }
            Err(e) => Err(format!(
                "Auf den S3-Bucket konnte nicht zugegriffen werden: {:?}",
                e
            )),
        }
    }

    pub fn delete_from(&self, client: &rusoto_s3::S3Client) -> bool {
        use futures::executor::block_on;
        use rusoto_s3::S3;

        let mut request = rusoto_s3::DeleteObjectRequest::default();
        request.key = self.object_key.to_owned();
        request.bucket = self.name.to_owned();

        block_on(client.delete_object(request)).is_ok()
    }
}
