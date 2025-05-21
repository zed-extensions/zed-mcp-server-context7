use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@upstash/context7-mcp";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@upstash/context7-mcp/dist/index.js";
const DEFAULT_MIN_TOKENS_ENV: &str = "DEFAULT_MINIMUM_TOKENS";

struct Context7ModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct Context7ModelContextExtensionSettings {
    #[serde(default)]
    default_minimum_tokens: Option<String>,
}

impl zed::Extension for Context7ModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-context7", project)?;

        let ext_settings: Option<Context7ModelContextExtensionSettings> =
            if let Some(settings_value) = settings.settings {
                match serde_json::from_value(settings_value) {
                    Ok(s) => Some(s),
                    Err(e) => return Err(format!("Failed to parse settings: {}", e).into()),
                }
            } else {
                None
            };

        let mut env_vars = Vec::new();

        if let Some(settings) = ext_settings {
            if let Some(default_minimum_tokens) = settings.default_minimum_tokens {
                env_vars.push((DEFAULT_MIN_TOKENS_ENV.to_string(), default_minimum_tokens));
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,

        _context_server_id: &ContextServerId,

        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("mcp-server-context7", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(context7_settings) =
                    serde_json::from_value::<Context7ModelContextExtensionSettings>(settings_value)
                {
                    if let Some(default_minimum_tokens) = context7_settings.default_minimum_tokens {
                        default_settings = default_settings
                            .replace("\"10000\"", &format!("\"{}\"", default_minimum_tokens));
                    }
                }
            }
        }

        let settings_schema = serde_json::to_string(&schemars::schema_for!(
            Context7ModelContextExtensionSettings
        ))
        .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,

            default_settings,

            settings_schema,
        }))
    }
}

zed::register_extension!(Context7ModelContextExtension);
