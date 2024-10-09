SRC_DIR = src
BUILD_DIR = build
BINARY_NAME = parsedown

all: run

build: clean
	@echo "Building..."
	@mkdir -p $(BUILD_DIR)
	@odin build $(SRC_DIR) -out:$(BUILD_DIR)/$(BINARY_NAME)

run: clean
	@echo "Running..."
	@mkdir -p $(BUILD_DIR)
	@odin run $(SRC_DIR) -out:$(BUILD_DIR)/$(BINARY_NAME)

clean:
	@echo "Cleaning..."
	@rm -rf $(BUILD_DIR)

.PHONY: all build run clean
