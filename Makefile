.PHONY: all clean web desktop fullstack

# Default target
all: web

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf dist
	@cargo clean

# Build and serve web application
web:
	@echo "Building and serving web application..."
	@dx serve --platform web

# Build and serve desktop application
desktop:
	@echo "Building and serving desktop application..."
	@dx serve --platform desktop

# Build and serve fullstack application
fullstack:
	@echo "Building and serving fullstack application..."
	@dx serve --platform fullstack

# Build for release (web)
release-web:
	@echo "Building web application for release..."
	@dx build --release --platform web

# Build for release (desktop)
release-desktop:
	@echo "Building desktop application for release..."
	@dx build --release --platform desktop

# Build for release (fullstack)
release-fullstack:
	@echo "Building fullstack application for release..."
	@dx build --release --platform fullstack

# Run Tailwind CSS compiler
tailwind:
	@echo "Running Tailwind CSS compiler..."
	@npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch

# Help target
help:
	@echo "Available targets:"
	@echo "  all (default)  : Alias for 'web'"
	@echo "  clean          : Remove build artifacts"
	@echo "  web            : Build and serve web application"
	@echo "  desktop        : Build and serve desktop application"
	@echo "  fullstack      : Build and serve fullstack application"
	@echo "  release-web    : Build web application for release"
	@echo "  release-desktop: Build desktop application for release"
	@echo "  release-fullstack: Build fullstack application for release"
	@echo "  tailwind       : Run Tailwind CSS compiler"
	@echo "  help           : Show this help message"

# Run Tailwind CSS compiler and serve web application
dev:
	@echo "Running Tailwind CSS compiler and serving web application..."
	@npx tailwindcss -i ./input.css -o ./assets/tailwind.css
	@npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch &
	@dx serve --hot-reload