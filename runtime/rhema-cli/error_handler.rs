use colored::*;
use rhema_core::RhemaError;
use std::io::{self, Write};

/// Error severity levels for CLI output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// Centralized error handler for CLI operations
pub struct ErrorHandler {
    verbose: bool,
    quiet: bool,
    color_enabled: bool,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new(verbose: bool, quiet: bool) -> Self {
        // For now, always enable colors. We can add terminal detection later
        let color_enabled = true;
        Self {
            verbose,
            quiet,
            color_enabled,
        }
    }

    /// Display an error with appropriate formatting
    pub fn display_error(&self, error: &RhemaError) -> io::Result<()> {
        if self.quiet {
            return Ok(());
        }

        let severity = self.classify_error(error);
        let message = self.format_error(error, severity);

        let mut stderr = io::stderr();
        writeln!(stderr, "{}", message)?;

        if self.verbose {
            self.display_error_context(error)?;
        }

        Ok(())
    }

    /// Display multiple errors
    pub fn display_errors(&self, errors: &[RhemaError]) -> io::Result<()> {
        if errors.is_empty() {
            return Ok(());
        }

        let mut stderr = io::stderr();
        writeln!(stderr, "{}", "Multiple errors occurred:".red().bold())?;

        for (i, error) in errors.iter().enumerate() {
            writeln!(stderr, "  {}. {}", i + 1, error)?;
        }

        Ok(())
    }

    /// Display a warning message
    pub fn display_warning(&self, message: &str) -> io::Result<()> {
        if self.quiet {
            return Ok(());
        }

        let formatted = if self.color_enabled {
            format!("⚠️  {}", message.yellow())
        } else {
            format!("Warning: {}", message)
        };

        let mut stderr = io::stderr();
        writeln!(stderr, "{}", formatted)?;
        Ok(())
    }

    /// Display an info message
    pub fn display_info(&self, message: &str) -> io::Result<()> {
        if self.quiet {
            return Ok(());
        }

        let formatted = if self.color_enabled {
            format!("ℹ️  {}", message.blue())
        } else {
            format!("Info: {}", message)
        };

        let mut stderr = io::stderr();
        writeln!(stderr, "{}", formatted)?;
        Ok(())
    }

    /// Classify error by severity
    fn classify_error(&self, error: &RhemaError) -> ErrorSeverity {
        match error {
            // Fatal errors - should exit immediately
            RhemaError::GitRepoNotFound(_)
            | RhemaError::ConfigError(_)
            | RhemaError::AuthenticationError(_)
            | RhemaError::AuthorizationError(_)
            | RhemaError::SafetyViolation(_) => ErrorSeverity::Fatal,

            // Errors - operation failed but may be recoverable
            RhemaError::FileNotFound(_)
            | RhemaError::ScopeNotFound(_)
            | RhemaError::NotFound(_)
            | RhemaError::Validation(_)
            | RhemaError::InvalidQuery(_)
            | RhemaError::SchemaValidation(_)
            | RhemaError::IoError(_)
            | RhemaError::YamlError(_)
            | RhemaError::JsonError(_)
            | RhemaError::ParseError(_)
            | RhemaError::InvalidInput(_)
            | RhemaError::NetworkError(_)
            | RhemaError::ExternalServiceError(_) => ErrorSeverity::Error,

            // Warnings - operation succeeded but with issues
            RhemaError::CircularDependency(_)
            | RhemaError::ValidationError(_)
            | RhemaError::PerformanceError(_) => ErrorSeverity::Warning,

            // Info - general information
            _ => ErrorSeverity::Info,
        }
    }

    /// Format error message with appropriate styling
    fn format_error(&self, error: &RhemaError, severity: ErrorSeverity) -> String {
        let prefix = match severity {
            ErrorSeverity::Fatal => "💥 Fatal Error",
            ErrorSeverity::Error => "❌ Error",
            ErrorSeverity::Warning => "⚠️  Warning",
            ErrorSeverity::Info => "ℹ️  Info",
        };

        let message = error.to_string();

        if self.color_enabled {
            match severity {
                ErrorSeverity::Fatal => format!("{}: {}", prefix.red().bold(), message.red()),
                ErrorSeverity::Error => format!("{}: {}", prefix.red(), message),
                ErrorSeverity::Warning => format!("{}: {}", prefix.yellow(), message.yellow()),
                ErrorSeverity::Info => format!("{}: {}", prefix.blue(), message.blue()),
            }
        } else {
            format!("{}: {}", prefix, message)
        }
    }

    /// Display additional error context and suggestions
    fn display_error_context(&self, error: &RhemaError) -> io::Result<()> {
        let mut stderr = io::stderr();

        let context = match error {
            RhemaError::GitRepoNotFound(_) => {
                "💡 Try running this command from a Git repository directory"
            }
            RhemaError::ConfigError(_) => {
                "💡 Check your configuration files and ensure they are valid"
            }
            RhemaError::FileNotFound(path) => {
                writeln!(stderr, "📁 File path: {}", path)?;
                "💡 Verify the file exists and you have read permissions"
            }
            RhemaError::ScopeNotFound(_) => "💡 Run 'rhema scopes' to see available scopes",
            RhemaError::Validation(_) => {
                "💡 Check your input data and ensure it meets validation requirements"
            }
            RhemaError::InvalidQuery(_) => "💡 Review the query syntax and ensure it's valid",
            RhemaError::AuthenticationError(_) => {
                "💡 Check your authentication credentials and permissions"
            }
            RhemaError::NetworkError(_) => "💡 Check your network connection and try again",
            _ => return Ok(()),
        };

        if self.color_enabled {
            writeln!(stderr, "{}", context.cyan())?;
        } else {
            writeln!(stderr, "{}", context)?;
        }

        Ok(())
    }

    /// Get exit code for error severity
    pub fn exit_code(&self, error: &RhemaError) -> i32 {
        match self.classify_error(error) {
            ErrorSeverity::Fatal => 1,
            ErrorSeverity::Error => 1,
            ErrorSeverity::Warning => 0,
            ErrorSeverity::Info => 0,
        }
    }
}

/// Convenience function to display an error and exit
pub fn display_error_and_exit(error: &RhemaError, verbose: bool, quiet: bool) -> ! {
    let handler = ErrorHandler::new(verbose, quiet);
    let _ = handler.display_error(error);
    std::process::exit(handler.exit_code(error));
}

/// Convenience function to display multiple errors and exit
pub fn display_errors_and_exit(errors: &[RhemaError], verbose: bool, quiet: bool) -> ! {
    let handler = ErrorHandler::new(verbose, quiet);
    let _ = handler.display_errors(errors);
    std::process::exit(1);
}
