UPLOAD_LAMBDA = vertragsservice-upload-lambda
UUID_LAMBDA = vertragsservice-uuid-lambda
IR_LAMBDA = vertragsservice-ir-lambda

update:
	cd ../$(UPLOAD_LAMBDA) && cargo update
	cd ../$(UUID_LAMBDA) && cargo update
	cd ../$(IR_LAMBDA) && cargo update
build:
	cd ../$(UPLOAD_LAMBDA) && cargo build
	cd ../$(UUID_LAMBDA) && cargo build
	cd ../$(IR_LAMBDA) && cargo build
test:
	cd ../$(UPLOAD_LAMBDA) && make test
	cd ../$(UUID_LAMBDA) && make test
	cd ../$(IR_LAMBDA) && make test
local:
	cd ../$(UPLOAD_LAMBDA) && make local
	cd ../$(UUID_LAMBDA) && make local
	cd ../$(IR_LAMBDA) && make local
aws:
	cd ../$(UPLOAD_LAMBDA) && make aws
	cd ../$(UUID_LAMBDA) && make aws
	cd ../$(IR_LAMBDA) && make aws