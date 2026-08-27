use std::{env, fs};
use zed_extension_api::{self as zed, settings::LspSettings, Result};

const SERVER_PATH: &str = "node_modules/graphql-language-service-cli/dist/cli.js";
const PACKAGE_NAME: &str = "graphql-language-service-cli";
const BINARY_NAME: &str = "graphql-lsp";

struct GraphQLExtension;

impl GraphQLExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                                "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                            ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for GraphQLExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let server_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings);

        // `binary.path`, `binary.arguments` and `binary.env` are applied by Zed itself before
        // the extension is consulted (see `LspStore::get_language_server_binary`), so there is
        // nothing to do for them here. `use_system_binary` lives in the free-form `settings`
        // block precisely so that it does reach us.
        let use_system_binary = server_settings
            .as_ref()
            .and_then(|settings| settings.get("use_system_binary"))
            .and_then(|value| value.as_bool())
            .unwrap_or(false);

        let config_dir = server_settings
            .and_then(|settings| settings.get("config_dir").cloned())
            .and_then(|r| r.as_str().map(|s| s.to_string()))
            .unwrap_or(worktree.root_path().to_string());

        let args = vec![
            "server".to_string(),
            "-m".to_string(),
            "stream".to_string(),
            "-c".to_string(),
            config_dir,
        ];
        let env = vec![("GRAPHQL_NO_NAME_WARNING".to_string(), "true".to_string())];

        if use_system_binary {
            if let Some(path) = worktree.which(BINARY_NAME) {
                return Ok(zed::Command {
                    command: path,
                    args,
                    env,
                });
            }
        }

        let server_path = self.server_script_path(language_server_id)?;
        let mut node_args = vec![env::current_dir()
            .unwrap()
            .join(&server_path)
            .to_string_lossy()
            .to_string()];
        node_args.extend(args);

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: node_args,
            env,
        })
    }
}

zed::register_extension!(GraphQLExtension);
