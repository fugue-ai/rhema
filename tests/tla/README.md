# Rhema TLA+ Specifications

This directory contains the TLA+ (Temporal Logic of Actions) specifications for the Rhema system. These specifications provide formal models of the system's behavior and are used to verify safety and liveness properties.

## 📁 File Structure

- **`rhema_core.tla`** - Core system specification with basic safety invariants and actions
- **`rhema_edge_cases.tla`** - Edge case scenarios and failure recovery models
- **`rhema_invariants.tla`** - Detailed safety and liveness property specifications
- **`rhema_config.cfg`** - TLC model checker configuration
- **`rhema_current_state.tla`** - Documentation of current implementation status
- **`README.md`** - This documentation file

## 🎯 Current Status

### ✅ Implementation Alignment

The TLA specifications are **current and well-aligned** with the actual implementation. All major safety invariants and liveness properties have been implemented in the codebase.

### 📊 Validation Status

| Component | Status | Implementation Location |
|-----------|--------|------------------------|
| **Safety Invariants** | ✅ Complete | `src/safety/` |
| **Agent Coordination** | ✅ Complete | `src/agent/` |
| **Lock Management** | ✅ Complete | `src/agent/locks.rs` |
| **Sync Coordination** | ✅ Complete | `src/agent/coordination.rs` |
| **Edge Case Handling** | ✅ Complete | Various modules |
| **MCP Integration** | ✅ Complete | `src/mcp/` |

## 🔧 Running the TLA Specifications

### Prerequisites

1. **TLA+ Toolbox** - Download from [TLA+ website](https://lamport.azurewebsites.net/tla/toolbox.html)
2. **TLC Model Checker** - Included with TLA+ Toolbox
3. **Java Runtime Environment** - Required for TLC

### Running with TLA+ Toolbox

1. Open TLA+ Toolbox
2. Import the TLA files:
   - `rhema_core.tla`
   - `rhema_edge_cases.tla`
   - `rhema_invariants.tla`
3. Load the configuration file `rhema_config.cfg`
4. Run TLC model checker

### Running from Command Line

```bash
# Navigate to the TLA directory
cd tests/tla

# Run TLC model checker
java -cp /path/to/tla2tools.jar tlc2.TLC rhema_config.cfg
```

## 📋 Safety Properties Verified

### Core Safety Invariants

1. **ContextConsistency** - YAML content validity and scope references
2. **DependencyIntegrity** - No circular dependencies, valid references
3. **AgentCoordination** - Agent state consistency and concurrency limits
4. **LockConsistency** - Lock ownership and timeout management
5. **SyncStatusConsistency** - Synchronization state transitions

### Edge Case Safety Properties

1. **NoPermanentFailures** - System eventually recovers from failures
2. **RecoveryProgress** - Recovery operations make progress
3. **BoundedRetries** - Retry attempts are bounded
4. **McpConnectionStability** - MCP connections remain stable
5. **SafetyValidationQueueBounds** - Validation queue size is bounded

## 🔄 Liveness Properties Verified

### Core Liveness Properties

1. **ContextSyncLiveness** - Scopes eventually sync
2. **AgentProgress** - Agents eventually make progress
3. **ProgressGuarantee** - All agents eventually work or complete
4. **SyncCompletion** - Sync operations eventually complete
5. **ConflictResolution** - Conflicts are eventually resolved

### Edge Case Liveness Properties

1. **EventuallyRecoverFromCrashes** - Agent crashes are eventually recovered
2. **EventuallyRecoverFromPartitions** - Network partitions are eventually resolved
3. **EventuallyRecoverFromCorruption** - Git corruption is eventually fixed
4. **EventuallyRecoverMcpConnections** - MCP connections are eventually restored
5. **EventuallyCompleteSafetyValidations** - Safety validations eventually complete

## 🧪 Test Scenarios

The specifications include comprehensive test scenarios:

### Basic Scenarios
- **TestEmptyRepository** - Empty repository behavior
- **TestSingleScope** - Single scope operations
- **TestCircularDependencies** - Circular dependency detection

### Failure Scenarios
- **TestAgentCrashes** - Agent crash recovery
- **TestNetworkPartitions** - Network partition handling
- **TestGitCorruption** - Git corruption recovery
- **TestResourceExhaustion** - Resource exhaustion handling

### Advanced Scenarios
- **TestMcpConnectionIssues** - MCP daemon connection problems
- **TestSafetyValidationQueueOverflow** - Validation queue overflow
- **TestConcurrentSafetyValidations** - Concurrent validation handling
- **TestEnhancedErrorRecovery** - Enhanced error recovery mechanisms

## 🔍 Model Checking Configuration

### Constants

```tla
MaxConcurrentAgents = 3
MaxBlockTime = 10
Scope = {"scope1", "scope2", "scope3"}
Agent = {"agent1", "agent2", "agent3", "agent4"}
ContextFileTypes = {"rhema.yaml", "knowledge.yaml", "todos.yaml", "decisions.yaml", "patterns.yaml", "conventions.yaml"}
RepositoryFiles = {"scope1", "scope2", "scope3", "shared.yaml"}
Content = {"", "valid_content", "invalid_content", "updated", "corrupted", "access_denied", "recovered"}
GitOperationTypes = {"commit", "merge", "rebase", "reset"}
```

### Constraints

- Maximum 4 concurrent agents
- Maximum 3 scopes
- Maximum 2 dependencies per scope
- Safety validation queue size ≤ 10

## 📈 Performance Settings

- **Memory Limit**: 8GB
- **Timeout**: 1 hour
- **Detailed Error Reporting**: Enabled
- **State Space Exploration Statistics**: Enabled
- **Counterexample Generation**: Enabled
- **Deadlock Detection**: Enabled
- **Fairness Checking**: Enabled

## 🔄 Continuous Integration

### Automated Validation

The TLA specifications can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run TLA Model Checking
  run: |
    java -cp $TLA_PATH/tla2tools.jar tlc2.TLC tests/tla/rhema_config.cfg
```

### Property Validation

The implementation includes runtime validation of TLA properties:

```rust
// Example: Validating safety invariants
let safety_validator = SafetyValidator::new();
safety_validator.validate_all_safety_invariants(
    &agents,
    &locks,
    &sync_status,
    &sync_dependencies,
    &dependencies,
    max_concurrent_agents,
    max_block_time,
)?;
```

## 🎯 Future Enhancements

### Planned Improvements

1. **Advanced Specifications**
   - Model complex query provenance tracking
   - Specify performance monitoring invariants
   - Model CI/CD pipeline interactions

2. **Automated Validation**
   - Generate test cases from TLA specifications
   - Real-time validation of system state
   - Automated property checking in CI/CD

3. **Formal Verification**
   - Prove implementation correctness against TLA spec
   - Use theorem provers for complex properties
   - Formal verification of safety critical components

## 📚 References

- [TLA+ Homepage](https://lamport.azurewebsites.net/tla/tla.html)
- [TLA+ Toolbox](https://lamport.azurewebsites.net/tla/toolbox.html)
- [TLC Model Checker](https://lamport.azurewebsites.net/tla/tlc.html)
- [Rhema Architecture Documentation](../ARCHITECTURE.md)

## 🤝 Contributing

When making changes to the Rhema system:

1. **Update TLA specifications** to reflect new behavior
2. **Run model checking** to verify properties still hold
3. **Update this documentation** to reflect changes
4. **Add new test scenarios** for new features
5. **Validate implementation** against updated specifications

## 📞 Support

For questions about the TLA specifications:

1. Check the implementation code for examples
2. Review the safety validation modules
3. Run the model checker to identify issues
4. Consult the architecture documentation

---

**Last Updated**: January 2025  
**Status**: ✅ Current and Complete  
**Implementation Alignment**: ✅ Fully Aligned 