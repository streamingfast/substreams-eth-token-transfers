ENDPOINT ?= mainnet.eth.streamingfast.io:443
ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_json_transfers -s 12292922 -t +10

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: sink_files
sink_files: build
	substreams-sink-files run --file-working-dir="$(ROOT_DIR)/sink-files/working" --state-store="$(ROOT_DIR)/sink-files/workdir/state.yaml" $(ENDPOINT) "$(ROOT_DIR)/substreams.yaml" map_transfers ".transfers[]" "$(ROOT_DIR)/sink-files/out"

.PHONY: package
package: build
	substreams pack -o substreams.spkg ./substreams.yaml
