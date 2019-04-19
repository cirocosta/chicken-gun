IMAGE_NAME ?= cirocosta/chicken-gun

image:
	docker build -t $(IMAGE_NAME) .

monitoring-stack:
	cd ./monitoring-stack && \
		docker-compose up -d
.PHONY: monitoring-stack
