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

#[derive(Debug, Deserialize, JsonSchema)]
struct Context7ModelContextExtensionSettings {
    #[serde(default)]
    context7_api_key: Option<String>,
}

struct Context7ModelContextExtension;

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

        let settings_struct: Context7ModelContextExtensionSettings = match settings.settings {
            Some(v) => serde_json::from_value(v).map_err(|e| e.to_string())?,
            None => Context7ModelContextExtensionSettings {
                context7_api_key: None,
            },
        };

        let mut args = Vec::new();
        args.push(
            env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string(),
        );

        if let Some(key) = settings_struct.context7_api_key {
            args.push(format!("--api-key={}", key));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: Default::default(),
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
                    match context7_settings.context7_api_key {
                        Some(context7_api_key) => {
                            default_settings = default_settings.replace(
                                "\"YOUR_CONTEXT7_API_KEY\"",
                                &format!("\"{}\"", context7_api_key),
                            );
                        }
                        None => {
                            // If no API key provided, replace the placeholder with null
                            // so the default_settings won't contain an invalid placeholder key.
                            default_settings =
                                default_settings.replace("\"YOUR_CONTEXT7_API_KEY\"", "\"\"");
                        }
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
