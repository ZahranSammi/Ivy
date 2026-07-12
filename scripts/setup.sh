#!/bin/bash
echo "Setting up Ivy development environment..."
cp -n .env.example .env
docker compose -f deploy/docker-compose.dev.yml up -d
echo "Waiting for DBs..."
sleep 5
cd backend && sqlx database create && sqlx migrate run
echo "Setup complete!"
