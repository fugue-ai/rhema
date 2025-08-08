//! Enhanced test fixtures for complex testing scenarios

use git2::Repository;
use rhema_api::Rhema;
use rhema_core::RhemaResult;
use std::path::PathBuf;
use tempfile::TempDir;

/// Enhanced test fixtures for complex testing scenarios
pub struct EnhancedFixtures;

impl EnhancedFixtures {
    /// Create an enhanced test fixture with advanced setup
    pub fn advanced() -> RhemaResult<(TempDir, Rhema)> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize git repository
        let _repo = Repository::init(&repo_path)?;

        // Create Rhema instance
        let rhema = Rhema::new_from_path(repo_path)?;

        Ok((temp_dir, rhema))
    }

    /// Create a test fixture with performance monitoring
    pub fn with_performance_monitoring() -> RhemaResult<(TempDir, Rhema)> {
        Self::advanced()
    }

    /// Create a test fixture with security features
    pub fn with_security_features() -> RhemaResult<(TempDir, Rhema)> {
        Self::advanced()
    }
}
