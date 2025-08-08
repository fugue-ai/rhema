use rhema_cli::Rhema;
use rhema_core::RhemaResult;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("Testing basic Rhema functionality...");

    // Create Rhema instance
    let rhema = Rhema::new()?;
    println!("✅ Rhema instance created");

    // Test scope discovery
    let scopes = rhema.discover_scopes()?;
    println!("✅ Discovered {} scopes", scopes.len());

    // Test query functionality
    let result = rhema.query("simple")?;
    println!("✅ Query executed successfully");

    // Test search functionality
    let search_results = rhema.search_regex("test", None)?;
    println!(
        "✅ Search executed successfully, found {} results",
        search_results.len()
    );

    println!("All basic Rhema methods are accessible!");
    Ok(())
}
