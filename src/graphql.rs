use std::{env, fs};
use zed_extension_api::{self as zed, settings::LspSettings, Result};

const SERVER_PATH: &str = "node_modules/graphql-language-service-cli/bin/graphql.js";
const PACKAGE_NAME: &str = "graphql-language-service-cli";

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
        let server_path = self.server_script_path(language_server_id)?;

        let config_dir = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings)
            .and_then(|settings| settings.get("config_dir").cloned())
            .and_then(|r| r.as_str().map(|s| s.to_string()))
            .unwrap_or(worktree.root_path().to_string());

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "server".to_string(),
                "-m".to_string(),
                "stream".to_string(),
                "-c".to_string(),
                config_dir,
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GraphQLExtension);
