SRC_DIR = src
BUILD_DIR = build
BINARY_NAME = parsedown

.PHONY: all build run clean

all: build

build: clean
	@echo "Building..."
	@mkdir -p $(BUILD_DIR)
	@odin build $(SRC_DIR) -out:$(BUILD_DIR)/$(BINARY_NAME)

run: build
	@echo "Running..."
	@./$(BUILD_DIR)/$(BINARY_NAME)

clean:
	@echo "Cleaning..."
	@rm -rf $(BUILD_DIR)
