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

By default, the extension installs and manages its own copy of [`graphql-language-service-cli`](https://github.com/graphql/graphiql/tree/main/packages/graphql-language-service-cli) via npm. There are two ways to use a locally provided server instead (e.g. from Nix, direnv, mise, or asdf environments, or when working offline).

#### `use_system_binary` — pick up `graphql-lsp` from `PATH`

When enabled, the extension looks for a `graphql-lsp` binary on the worktree's `PATH` and uses it instead of the managed install. No npm version check, npm request, or managed install is performed. If nothing is found on `PATH`, the extension falls back to the managed install.

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

The server is launched with the same defaults as the managed install: arguments `server -m stream -c <config_dir>` (honoring the `config_dir` setting) and `GRAPHQL_NO_NAME_WARNING=true`.

This is opt-in. Without it, a `graphql-lsp` on `PATH` is ignored — see the note below for why.

#### `binary.path` — point Zed at a specific binary

This is handled by Zed itself, not by this extension, and works with any version of it. Zed launches the configured binary directly, so **the extension's defaults do not apply**:

```json
{
  "lsp": {
    "graphql": {
      "binary": {
        "path": "/usr/local/bin/graphql-lsp",
        "arguments": ["server", "-m", "stream", "-c", "/path/to/config-dir"],
        "env": { "NODE_OPTIONS": "--max-old-space-size=4096" }
      }
    }
  }
}
```

- `arguments` is **required**. If you omit it the server is launched with no arguments at all and exits immediately with `At least one command is required.`
- `config_dir` is ignored. Pass `-c <dir>` yourself in `arguments`.
- `env` is **not** merged with `GRAPHQL_NO_NAME_WARNING=true`. Add it yourself if you want it.
- No npm version check or installation is performed.

> **Do not set `binary.arguments` without `binary.path`.** Zed replaces the arguments of the managed command too, which drops the path to `dist/cli.js` and leaves the server unable to start (`Error: Cannot find module '<worktree>/server'`). To change the config directory for the managed install, use the `config_dir` setting instead.

> **Note:** the stock `graphql-lsp` entry point shipped by `graphql-language-service-cli` (including global npm installs) fails on Node.js 22+ with `Cannot find module 'core-js/es6'`; the extension-managed install works around this by invoking `dist/cli.js` directly. This is why `use_system_binary` is opt-in — picking up such a binary automatically would replace a working server with a broken one. If you enable it or set `binary.path`, make sure the binary you provide starts under your Node version.

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
