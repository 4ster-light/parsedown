SRC_DIR = src
BUILD_DIR = build

all: build

build:
	@mkdir -p $(BUILD_DIR)
	@odin build $(SRC_DIR)/main.odin -out:$(BUILD_DIR)/parsedown -file

run: build
	@./$(BUILD_DIR)/parsedown

clean:
	@rm -rf $(BUILD_DIR)

.PHONY: all build run clean
