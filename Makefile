VERSION := $(shell cargo read-manifest| jq -r .version)
export VERSION

.PHONY:
format:
	cargo clippy --fix --allow-dirty
	cargo fmt

.PHONY:
release:
	git tag -a v$(VERSION) -m "release v$(VERSION)"
	git push origin v$(VERSION)
	cargo publish