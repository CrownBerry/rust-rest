ifneq ($(GC_REGISTRY)$(GC_PROJECT)$(RRA_VERSION),)
	image := $(GC_REGISTRY)/$(GC_PROJECT)/rra:$(RRA_VERSION)
	image-job := $(GC_REGISTRY)/$(GC_PROJECT)/rra-job:$(RRA_VERSION)
else
	image := eu.gcr.io/rust-rest-api/rra:0.2.3
	image-job := eu.gcr.io/rust-rest-api/rra-job:0.2.3
endif

build-dev:
	cargo build

run-dev: build-dev
	cargo run

build-release:
	docker build --network=host -t $(image) .

build-migrations:
	docker build --network=host -t $(image-job) migrations/

push:
	docker push $(image)

push-migrations:
	docker push $(image-job)

lint:
	mkdir -p helm/.lint
	rsync -avzh helm/* helm/.lint/rra/
	helm lint helm/.lint/rra
	rm -rf helm/.lint

.PHONY: build-dev run-dev build-release push
