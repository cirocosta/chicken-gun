IMAGE_NAME ?= cirocosta/chicken-gun

build:
	cargo install --path=. --force

image:
	docker build -t $(IMAGE_NAME) .

monitoring-stack:
	cd ./monitoring-stack && \
		docker-compose up -d
.PHONY: monitoring-stack
