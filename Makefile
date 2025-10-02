CRATES := meowbot-ai meowbot-core meowbot-db meowbot-platform meowbot-weather
MODE ?= debug

OUT_DIR := bin

.PHONY: all $(CRATES) clean

build: $(CRATES)

$(CRATES):
	cargo build -p $@ $(if $(filter $(MODE),release),--release,)
	mkdir -p $(OUT_DIR)/$@
	cp target/$(MODE)/$@ $(OUT_DIR)/$@/
	@if [ -f $@/.env ]; then cp $@/.env $(OUT_DIR)/$@/.env; fi

clean:
	rm -rf $(OUT_DIR)
	cargo clean

run-ai:
	./$(OUT_DIR)/meowbot-ai/meowbot-ai

run-core:
	./$(OUT_DIR)/meowbot-core/meowbot-core

run-db:
	./$(OUT_DIR)/meowbot-db/meowbot-db

run-platform:
	./$(OUT_DIR)/meowbot-platform/meowbot-platform

run-weather:
	./$(OUT_DIR)/meowbot-weather/meowbot-weather