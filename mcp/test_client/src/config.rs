use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, ServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl McpConfig {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        
        let config: McpConfig = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;
        
        Ok(config)
    }
    
    pub fn get_server(&self, name: &str) -> Option<&ServerConfig> {
        self.mcp_servers.get(name)
    }
    
    pub fn first_server(&self) -> Option<(&String, &ServerConfig)> {
        self.mcp_servers.iter().next()
    }
}

impl ServerConfig {
    pub fn get_command_path(&self) -> PathBuf {
        PathBuf::from(&self.command)
    }
}

