.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	echo "::: Setting up..."
	npm install -g get-graphql-schema

update_gql_schema: ## Update the schema
	# Run the app before running this command
	get-graphql-schema http://127.0.0.1:8000/graphql > schema.graphql

migrate_db:
	diesel setup
	diesel migration run

reset_db:
	diesel database reset

fmt: ## Format the codebase.
	cargo fmt --all

fmt_check: ## Check is the codebase properly formatted.
	cargo fmt --all -- --check

lint: ## Lint the codebase.
	cargo clippy --locked --all-targets

test:
	env PGPASSWORD=$(db_password) psql --host localhost --username postgres zoo --command "DELETE FROM user_;"
	. ./.env && cargo test --all-targets

	env PGPASSWORD=$(db_password) psql --host localhost --username postgres zoo --command "DELETE FROM user_;"

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used in git hooks.
	# usage: make comply db_password=secret

check: sort_check fmt_check lint test ## Check if the repository comply with the rules and ready to be pushed.
