.PHONY: test
export UID = $(shell id -u)
export GID = $(shell id -g)

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

install: ## Install docker environnement
	docker-compose build

start: ## Start the server
	docker-compose up

stop: ## Stop the server
	docker-compose down

test: ## Test the application
	docker-compose run api cargo test

lint: ## Run the linter 
	docker-compose run api cargo clippy

format:
	docker-compose run api cargo fmt
