# GraphQL extensions for Zed Editor

## Features
- Syntax highlighting for `.graphql` files based on [tree-sitter-graphql](https://github.com/bkegley/tree-sitter-graphql)
- Outline view for `.graphql` files
- Operation detection in `.ts`, `.tsx`, `.js`, `.jsx`, `.vue`, `.astro` and `.svelte` files through GraphQL tags and comments
- Autocompletion
- Query validation
- Hover hints

## Configuration
Several features depend on the [GraphQL language server](https://github.com/graphql/graphiql/tree/main/packages/graphql-language-service-cli) which requires a config file.

Simple config exmaple:
```yml
# graphql.config.yml

schema: 'schema.graphql' # or 'http://localhost:9876/graphql'
documents: 'src/**/*.{graphql,js,ts,jsx,tsx}'
```

Several file formats and configuration options are available. For further details please refer to the documentation for [`graphql-config`](https://the-guild.dev/graphql/config)
