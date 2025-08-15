/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use crate::lock::*;

    #[test]
    fn test_lock_module_structure() {
        // Test that we can access the main lock module components
        assert_eq!(DEFAULT_LOCK_FILE, "rhema.lock");
        assert_eq!(DEFAULT_LOCK_VERSION, "1.0.0");

        // Test that LockFileOps is accessible
        let _ops = LockFileOps;

        // Test that ValidationResult is accessible
        let validation_result = ValidationResult {
            is_valid: true,
            messages: vec!["test".to_string()],
            validation_time_ms: 100,
        };
        assert!(validation_result.is_success());
        assert_eq!(validation_result.message_count(), 1);

        // Test that LockFileStats is accessible
        let _stats = LockFileStats {
            total_scopes: 0,
            total_dependencies: 0,
            circular_dependencies: 0,
            file_size_bytes: 0,
            last_modified: chrono::Utc::now(),
            generation_time_ms: 0,
        };
    }
}
