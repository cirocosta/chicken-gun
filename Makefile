IMAGE_NAME ?= cirocosta/chicken-gun

image:
	docker build -t $(IMAGE_NAME) .
