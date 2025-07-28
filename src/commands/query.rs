use crate::{Gacp, GacpResult};
use colored::*;
use serde_yaml;

pub fn run(gacp: &Gacp, query: &str) -> GacpResult<()> {
    println!("{}", "Executing CQL query:".bold());
    println!("  {}", query.cyan());
    println!();
    
    let result = gacp.query(query)?;
    
    // Pretty print the result
    let yaml_string = serde_yaml::to_string(&result)?;
    println!("{}", "Result:".bold());
    println!("{}", yaml_string);
    
    Ok(())
} 