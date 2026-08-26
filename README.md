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

### Using a local language server

By default, the extension installs and manages its own copy of [`graphql-language-service-cli`](https://github.com/graphql/graphiql/tree/main/packages/graphql-language-service-cli) via npm. To use a locally provided server instead (e.g. from Nix, direnv, mise, or asdf environments, or when working offline), the extension resolves the server in this order:

1. The binary configured in Zed settings:
   ```json
   {
     "lsp": {
       "graphql": {
         "binary": {
           "path": "/usr/local/bin/graphql-lsp",
           // optional, defaults to ["server", "-m", "stream", "-c", "<config_dir>"]
           "arguments": ["server", "-m", "stream", "-c", "/path/to/config-dir"],
           // optional, merged with the default { "GRAPHQL_NO_NAME_WARNING": "true" }
           "env": { "NODE_OPTIONS": "--max-old-space-size=4096" }
         }
       }
     }
   }
   ```
2. A `graphql-lsp` binary found on the worktree's `PATH` — only when explicitly enabled:
   ```json
   {
     "lsp": {
       "graphql": {
         "settings": {
           "use_system_binary": true
         }
       }
     }
   }
   ```
3. The extension-managed npm installation (the default behavior — used whenever neither option above is configured).

`binary.arguments` fully replaces the default arguments, including `-c <config_dir>` — if you also use the `config_dir` setting, pass `-c` yourself. `binary.env` is merged with the defaults, with your values taking precedence. Both apply to whichever server ends up selected, including the extension-managed npm fallback (where arguments are appended after the script path).

When a local server is selected, no npm version check or installation is performed.

> **Note:** the stock `graphql-lsp` entry point shipped by `graphql-language-service-cli` (including global npm installs) fails on Node.js 22+ with `Cannot find module 'core-js/es6'`; the extension-managed install works around this by invoking `dist/cli.js` directly. If you enable `use_system_binary` or set `binary.path`, make sure the binary you provide starts under your Node version.

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
