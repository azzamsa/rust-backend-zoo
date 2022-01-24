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
