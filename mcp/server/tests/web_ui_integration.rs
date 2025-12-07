//! Integration tests for web UI and MCP server

#[cfg(all(test, feature = "web-ui"))]
mod tests {
    use vim_mcp_server::model::load_embedded_database;
    
    #[tokio::test]
    async fn test_database_loads() {
        let result = load_embedded_database();
        assert!(result.is_ok());
        
        let db = result.unwrap();
        assert!(!db.items.is_empty());
    }
    
    // Note: Full end-to-end tests with actual HTTP server
    // would go here but are marked as #[ignore] by default
}


