.PHONY: dev build test clean setup lint fmt check

# ---- Development ----
dev:
	@echo "🌿 Starting Ivy development environment..."
	docker compose -f deploy/docker-compose.dev.yml up -d neo4j redis postgres
	@echo "Starting backend..."
	cd backend && cargo run --bin ivy-server &
	@echo "Starting frontend..."
	cd frontend && npm run dev

build:
	@echo "🔨 Building Ivy..."
	cd backend && cargo build --release
	cd frontend && npm run build

# ---- Testing ----
test:
	@echo "🧪 Running tests..."
	cd backend && cargo test
	cd frontend && npm test

test-integration:
	@echo "🧪 Running integration tests..."
	cd backend && cargo test --test '*' -- --ignored

# ---- Code Quality ----
lint:
	cd backend && cargo clippy -- -D warnings
	cd frontend && npm run lint

fmt:
	cd backend && cargo fmt
	cd frontend && npm run format

check:
	cd backend && cargo check
	cd frontend && npx tsc --noEmit

# ---- Database ----
migrate:
	cd backend && sqlx migrate run --source ivy-db/src/migrations

migrate-create:
	@read -p "Migration name: " name; \
	cd backend && sqlx migrate add --source ivy-db/src/migrations $$name

# ---- Docker ----
docker-build:
	docker compose -f deploy/docker-compose.yml build

docker-up:
	docker compose -f deploy/docker-compose.yml up -d

docker-down:
	docker compose -f deploy/docker-compose.yml down

# ---- Tools ----
build-tools:
	@echo "🔧 Building tool Docker images..."
	@for dir in tools/ivy-*/; do \
		name=$$(basename $$dir); \
		echo "Building $$name..."; \
		docker build -t ivy/$$name:latest $$dir; \
	done

# ---- Plugins ----
install-exploit-plugin:
	@echo "🔌 Installing exploit plugin..."
	@for dir in plugins/ivy-exploit/tools/ivy-*/; do \
		name=$$(basename $$dir); \
		echo "Building $$name..."; \
		docker build -t ivy/$$name:latest $$dir; \
	done

# ---- Cleanup ----
clean:
	cd backend && cargo clean
	cd frontend && rm -rf .next node_modules
	docker compose -f deploy/docker-compose.yml down -v

# ---- Setup ----
setup:
	@echo "🌿 Setting up Ivy development environment..."
	cp -n .env.example .env || true
	cd backend && cargo build
	cd frontend && npm install
	@echo "✅ Setup complete! Run 'make dev' to start."
