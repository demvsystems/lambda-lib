UPLOAD_LAMBDA = vertragsservice-upload-lambda
UUID_LAMBDA = vertragsservice-uuid-lambda
IR_LAMBDA = vertragsservice-ir-lambda

build:
	cargo build --features "sqs s3 gzip sentry_log transaction_id"
test:
	cargo test --features "sqs s3 gzip sentry_log transaction_id"
update_all:
	cargo update
	cd ../$(UPLOAD_LAMBDA) && cargo update
	cd ../$(UUID_LAMBDA) && cargo update
	cd ../$(IR_LAMBDA) && cargo update
build_all:
	cd ../$(UPLOAD_LAMBDA) && cargo build
	cd ../$(UUID_LAMBDA) && cargo build
	cd ../$(IR_LAMBDA) && cargo build
test_all:
	cd ../$(UPLOAD_LAMBDA) && make test
	cd ../$(UUID_LAMBDA) && make test
	cd ../$(IR_LAMBDA) && make test
aws_all:
	cd ../$(UPLOAD_LAMBDA) && make aws
	cd ../$(UUID_LAMBDA) && make aws
	cd ../$(IR_LAMBDA) && make aws
