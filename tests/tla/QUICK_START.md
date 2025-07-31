# 🚀 Quick Start: Running TLA Specifications

## Option 1: Using TLA+ Toolbox (Easiest)

### Install TLA+ Toolbox
```bash
# On macOS with Homebrew
brew install --cask tla+-toolbox

# Or download manually from:
# https://lamport.azurewebsites.net/tla/toolbox.html
```

### Run Specifications
1. **Launch TLA+ Toolbox**
2. **Create New Spec**:
   - File → New → Spec
   - Choose "Create new TLA+ module"
3. **Import Files**:
   - `rhema_core.tla`
   - `rhema_edge_cases.tla` 
   - `rhema_invariants.tla`
4. **Load Configuration**:
   - Right-click spec → "New Model"
   - Choose `rhema_config.cfg`
5. **Run Model Checker**:
   - Right-click model → "Run TLC"

## Option 2: Command Line (Advanced)

### Prerequisites
- Java (already installed)
- TLA tools (tla2tools.jar)

### Quick Run
```bash
# Navigate to TLA directory
cd tests/tla

# Run the automated script
./run_tla.sh
```

### Manual Run
```bash
# Download TLA tools
wget https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar

# Run TLC model checker
java -cp tla2tools.jar tlc2.TLC rhema_config.cfg
```

## Option 3: CI/CD Integration

### GitHub Actions Example
```yaml
- name: Run TLA Model Checking
  run: |
    cd tests/tla
    wget https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar
    java -cp tla2tools.jar tlc2.TLC rhema_config.cfg
```

## 📊 Expected Output

When successful, you should see:
```
TLC2 Version 2.18 of Day Month DD HH:MM:SS TZ YYYY
Running breadth-first search Model-Checking with fp 12 and seed -1234567890123456789 with 4 workers on 8 cores with 8192MB heap and 64MB offheap.
Parsing file /path/to/rhema_core.tla
Parsing file /path/to/rhema_edge_cases.tla
Parsing file /path/to/rhema_invariants.tla
Semantic processing of module RHEMA_Core
Semantic processing of module RHEMA_EdgeCases
Semantic processing of module RHEMA_Invariants
Starting... (2025-01-XX HH:MM:SS)
Computing initial states...
Finished computing initial states: 1 distinct state generated.
Model checking completed. No error has been found.
Estimates of the probability that TLC did not check all reachable states because two distinct states had the same fingerprint:
calculated (optimistic): 0.0
based on the actual fingerprints: 0.0
```

## 🔍 What Gets Verified

### Safety Properties ✅
- Context consistency
- Dependency integrity  
- Agent coordination
- Lock consistency
- Sync status consistency
- No circular dependencies
- No deadlocks

### Liveness Properties ✅
- Agents eventually make progress
- Scopes eventually sync
- Conflicts eventually resolve
- System eventually recovers from failures

### Edge Cases ✅
- Agent crashes
- Network partitions
- Git corruption
- Resource exhaustion
- MCP connection issues
- Safety validation timeouts

## 🛠️ Troubleshooting

### Common Issues

**"TLA tools not found"**
```bash
# Download manually
wget https://github.com/tlaplus/tlaplus/releases/download/v1.8.0/tla2tools.jar
```

**"Java not found"**
```bash
# Install Java
brew install openjdk
```

**"Out of memory"**
```bash
# Increase heap size
java -Xmx16g -cp tla2tools.jar tlc2.TLC rhema_config.cfg
```

**"Model too large"**
- Reduce constants in `rhema_config.cfg`
- Use symmetry reduction
- Increase memory limits

## 📈 Performance Tips

### For Large Models
```bash
# Use more memory
java -Xmx32g -cp tla2tools.jar tlc2.TLC rhema_config.cfg

# Use multiple workers
java -cp tla2tools.jar tlc2.TLC -workers 8 rhema_config.cfg

# Enable fingerprinting
java -cp tla2tools.jar tlc2.TLC -fp 12 rhema_config.cfg
```

### For Quick Testing
```bash
# Reduce model size in config
# Edit rhema_config.cfg:
# Scope = {"scope1", "scope2"}  # Reduce from 3 to 2
# Agent = {"agent1", "agent2"}  # Reduce from 4 to 2
```

## 🎯 Success Criteria

Your TLA specifications are working correctly when:

1. ✅ **No errors found** in model checking
2. ✅ **All invariants pass** 
3. ✅ **All properties verified**
4. ✅ **State space explored** completely
5. ✅ **No deadlocks detected**

## 📞 Need Help?

- Check the full [README.md](README.md) for detailed documentation
- Review [gacp_current_state.tla](gacp_current_state.tla) for implementation status
- Run `./run_tla.sh` for automated setup and execution

---

**🎉 Congratulations!** You're now ready to formally verify the Rhema system! 