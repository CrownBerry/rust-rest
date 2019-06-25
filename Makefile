ifneq ($(GC_REGISTRY)$(GC_PROJECT),)
	image := $(GC_REGISTRY)/$(GC_PROJECT)/rra
else
	image := eu.gcr.io/rust-rest-api/rra
endif

build-dev:
	cargo build

run-dev: build-dev
	cargo run

build-release:
	docker build --network=host -t $(image) .

push:
	docker push $(image)

lint:
	mkdir -p helm/.lint
	rsync -avzh helm/* helm/.lint/rust-rest/
	helm lint helm/.lint/rust-rest
	rm -rf helm/.lint

.PHONY: build-dev run-dev build-release push
