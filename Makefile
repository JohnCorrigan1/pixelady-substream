ENDPOINT ?= mainnet.eth.streamingfast.io:443
# START_BLOCK ?= 12292922
# START_BLOCK ?= 15524870
#first transfer 15,526,593
#START_BLOCK ?= 15525870
#START_BLOCK ?= 13992009
#START_BLOCK ?= 13988541
# START_BLOCK ?= 17111298
START_BLOCK ?= 15526583
STOP_BLOCK ?= +1000
PIPE ?= |
AUTH ?= (curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$$STREAMINGFAST_KEY'"}' $(PIPE) jq -r .token)

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: run
run: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: gui
gui: build
	substreams gui -e $(ENDPOINT) substreams.yaml map_transfers -s $(START_BLOCK) -t $(STOP_BLOCK)

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="google,sf/substreams/sink/database,sf/substreams/rpc,sf/substreams/v1"

.PHONY: pack
pack: build
	substreams pack substreams.yaml

.PHONY: auth
auth:
	export SUBSTREAMS_API_TOKEN=$(AUTH)
