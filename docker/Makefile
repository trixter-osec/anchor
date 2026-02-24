# Anchor version.
trixter_osec_anchor_cli=0.40.5
# Solana toolchain.
SOLANA_CLI=v2.3.0
# Build version should match the Anchor cli version.
VERSIONED_IMG_NAME=trixterosec/anchor:$(trixter_osec_anchor_cli)

.PHONY: build build-push build-shell publish

default:

build: build/Dockerfile
	@docker build \
	--build-arg trixter_osec_anchor_cli=$(trixter_osec_anchor_cli) \
	--build-arg SOLANA_CLI=$(SOLANA_CLI) \
	$@ -t $(VERSIONED_IMG_NAME)

build-push:
	@docker push $(VERSIONED_IMG_NAME)

build-shell:
	@docker run -ti --rm --net=host \
		-v $(WORKDIR)/..:/workdir \
		$(VERSIONED_IMG_NAME) bash

publish: build build-push
