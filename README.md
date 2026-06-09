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

### Customizing the config directory

By default, the LSP attempts to load a config file from the workspace root directory. To specify the config file directory add the following to Zed settings:
```json
{
  // ...

  "lsp": {
    "graphql": {
      "settings": {
        "config_dir": "./nested-directory"
      }
    }
  }
}
```

## Releasing

1. Bump `version` in `extension.toml`.
2. Add a matching entry to `CHANGELOG.md` (newest at the top, `# <version> - <date>`).
3. Commit and merge to `main`.
4. Tag the release and push the tag:
   ```sh
   git tag v<version>   # e.g. v1.0.5, must match extension.toml
   git push origin v<version>
   ```

Pushing a `v*` tag triggers the [`release.yml`](.github/workflows/release.yml) workflow, which automatically opens a PR against [`zed-industries/extensions`](https://github.com/zed-industries/extensions) to publish the new version. Once a Zed maintainer merges that PR, the update rolls out to users.
