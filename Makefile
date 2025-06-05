
install:
	@pnpm install

build: install
	@pnpm run tauri build

run: install
	@pnpm run tauri dev

help:
	@echo "Available commands:"
	@echo "  make install - Install dependencies"
	@echo "  make build   - Build the Tauri application"
	@echo "  make run     - Run the Tauri application in development mode"
	@echo "  make help    - Show this help message"

.PHONY: install build run help