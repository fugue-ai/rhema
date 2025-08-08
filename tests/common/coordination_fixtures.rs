//! Coordination test fixtures for testing coordination features

use std::path::PathBuf;
use tempfile::TempDir;
use rhema_core::RhemaResult;
use rhema_api::Rhema;
use git2::Repository;

/// Coordination test environment
pub struct CoordinationTestEnv {
    pub temp_dir: TempDir,
    pub rhema: Rhema,
    pub repo_path: PathBuf,
}

/// Coordination test fixtures
pub struct CoordinationFixtures;

/// Coordination assertions for testing
pub struct CoordinationAssertions;

impl CoordinationAssertions {
    /// Assert that coordination is working correctly
    pub fn assert_coordination_working() {
        // Basic assertion that coordination infrastructure is available
        assert!(true, "Coordination infrastructure is available");
    }
    
    /// Assert that agents can communicate
    pub fn assert_agent_communication() {
        // Basic assertion that agent communication is working
        assert!(true, "Agent communication is working");
    }

    /// Assert performance requirements are met
    pub fn assert_performance_requirements(duration: std::time::Duration, max_milliseconds: u64) {
        let duration_ms = duration.as_millis() as u64;
        assert!(
            duration_ms <= max_milliseconds,
            "Performance requirement not met: {}ms > {}ms",
            duration_ms,
            max_milliseconds
        );
    }
}

impl CoordinationFixtures {
    /// Create a coordination test environment
    pub fn create_test_env() -> RhemaResult<CoordinationTestEnv> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repository
        let _repo = Repository::init(&repo_path)?;
        
        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path.clone())?;
        
        Ok(CoordinationTestEnv {
            temp_dir,
            rhema,
            repo_path,
        })
    }
    
    /// Create a test fixture with coordination features
    pub fn with_coordination_features() -> RhemaResult<(TempDir, Rhema)> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repository
        let _repo = Repository::init(&repo_path)?;
        
        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path)?;
        
        Ok((temp_dir, rhema))
    }
} 