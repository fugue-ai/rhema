---------------------------- MODULE RHEMA_Invariants ----------------------------
EXTENDS RHEMA_Core, FiniteSets, TLC

\* Additional constants for invariant checking
CONSTANTS
  MaxScopeDepth,      \* Maximum depth of scope hierarchy
  MaxDependencies,    \* Maximum dependencies per scope
  MaxContextFileSize, \* Maximum size of context files
  TimeoutValue        \* Timeout value for operations

\* Additional variables for tracking
VARIABLES
  operation_history,  \* History of operations
  conflict_count,     \* Count of conflicts detected
  timeout_count       \* Count of timeouts

\* Extended type invariant
ExtendedTypeInvariant ==
  /\ TypeInvariant
  /\ operation_history \in Seq(STRING)
  /\ conflict_count \in Nat
  /\ timeout_count \in Nat

\* Detailed safety invariants

\* 1. Context File Integrity
ContextFileIntegrity ==
  /\ \A scope \in scopes : 
       \A file \in ContextFileTypes :
         context_files[scope][file] \in Content
  /\ \A scope \in scopes :
       Cardinality(DOMAIN context_files[scope]) <= Cardinality(ContextFileTypes)

\* 2. Dependency Graph Properties
DependencyGraphProperties ==
  /\ \A scope \in scopes : 
       Cardinality(dependencies[scope]) <= MaxDependencies
  /\ \A scope \in scopes :
       scope \notin dependencies[scope]  \* No self-dependencies
  /\ \A scope1, scope2 \in scopes :
       IF scope1 \in dependencies[scope2]
       THEN scope2 \notin dependencies[scope1]  \* No bidirectional dependencies

\* 3. Lock Consistency
LockConsistency ==
  /\ \A scope \in scopes :
       locks[scope] \in agents \cup {null}
  /\ \A agent \in agents :
       Cardinality({scope \in scopes : locks[scope] = agent}) <= 1  \* One lock per agent

\* 4. Agent State Consistency
AgentStateConsistency ==
  /\ \A agent \in agents :
       agent_states[agent] \in {"idle", "working", "blocked", "completed"}
  /\ \A agent \in agents :
       IF agent_states[agent] = "working"
       THEN \E scope \in scopes : locks[scope] = agent

\* 5. Sync Status Consistency
SyncStatusConsistency ==
  /\ \A scope \in scopes :
       sync_status[scope] \in {"idle", "syncing", "completed", "failed"}
  /\ \A scope \in scopes :
       IF sync_status[scope] = "syncing"
       THEN \A dep \in dependencies[scope] : sync_status[dep] = "completed"

\* 6. Git State Consistency
GitStateConsistency ==
  /\ \A file \in RepositoryFiles :
       git_state[file] \in Content
  /\ \A scope \in scopes :
       \E file \in RepositoryFiles : file = scope  \* All scopes tracked in Git

\* 7. Operation History Consistency
OperationHistoryConsistency ==
  /\ Len(operation_history) >= 0
  /\ \A i \in DOMAIN operation_history :
       operation_history[i] \in STRING

\* 8. Conflict Detection
ConflictDetection ==
  /\ conflict_count >= 0
  /\ \A scope \in scopes :
       IF sync_status[scope] = "failed"
       THEN conflict_count > 0

\* 9. Timeout Management
TimeoutManagement ==
  /\ timeout_count >= 0
  /\ \A agent \in agents :
       IF agent_states[agent] = "blocked"
       THEN timeout_count < TimeoutValue

\* 10. Resource Bounds
ResourceBounds ==
  /\ Cardinality(scopes) <= Cardinality(Scope)
  /\ Cardinality(agents) <= Cardinality(Agent)
  /\ \A scope \in scopes :
       Cardinality(dependencies[scope]) <= MaxDependencies

\* 11. Circular Dependency Prevention
CircularDependencyPrevention ==
  /\ \A scope \in scopes :
       ~IsInCycle(scope, dependencies, scopes)

\* 12. Deadlock Prevention
DeadlockPrevention ==
  /\ \A agent1, agent2 \in agents :
       IF agent1 \neq agent2
       THEN ~AreDeadlocked(agent1, agent2, locks, dependencies)

\* Helper functions for invariant checking

\* Check if a scope is part of a cycle
IsInCycle(scope, deps, all_scopes) ==
  LET visited == {scope}
       rec == {scope}
  IN HasCycleHelper(scope, deps, all_scopes, visited, rec)

HasCycleHelper(current, deps, all_scopes, visited, rec_stack) ==
  IF current \in rec_stack
  THEN TRUE
  ELSE IF current \in visited
       THEN FALSE
       ELSE LET new_visited == visited \cup {current}
                 new_rec == rec_stack \cup {current}
            IN \E next \in deps[current] :
                 HasCycleHelper(next, deps, all_scopes, new_visited, new_rec)

\* Check if two agents are deadlocked
AreDeadlocked(agent1, agent2, locks, deps) ==
  LET scope1 == CHOOSE scope \in DOMAIN locks : locks[scope] = agent1
       scope2 == CHOOSE scope \in DOMAIN locks : locks[scope] = agent2
  IN scope1 \in deps[scope2] /\ scope2 \in deps[scope1]

\* All safety invariants combined
AllSafetyInvariants ==
  /\ ContextFileIntegrity
  /\ DependencyGraphProperties
  /\ LockConsistency
  /\ AgentStateConsistency
  /\ SyncStatusConsistency
  /\ GitStateConsistency
  /\ OperationHistoryConsistency
  /\ ConflictDetection
  /\ TimeoutManagement
  /\ ResourceBounds
  /\ CircularDependencyPrevention
  /\ DeadlockPrevention

\* Liveness properties

\* 1. Progress Guarantee
ProgressGuarantee ==
  \A agent \in agents :
    \E future_state \in {"working", "completed"} :
      agent_states[agent] = future_state

\* 2. Sync Completion
SyncCompletion ==
  \A scope \in scopes :
    \E future_status \in {"completed", "failed"} :
      sync_status[scope] = future_status

\* 3. Conflict Resolution
ConflictResolution ==
  \A scope \in scopes :
    IF sync_status[scope] = "failed"
    THEN \E future_status \in {"idle", "syncing", "completed"} :
           sync_status[scope] = future_status

\* 4. Agent Fairness
AgentFairness ==
  \A agent \in agents :
    IF agent_states[agent] = "blocked"
    THEN \E future_state \in {"idle", "working", "completed"} :
           agent_states[agent] = future_state

\* All liveness properties combined
AllLivenessProperties ==
  /\ ProgressGuarantee
  /\ SyncCompletion
  /\ ConflictResolution
  /\ AgentFairness

\* Temporal properties

\* Eventually all agents make progress
EventuallyAllAgentsProgress ==
  \A agent \in agents :
    \E future_state \in {"working", "completed"} :
      []<>(agent_states[agent] = future_state)

\* Eventually all scopes sync
EventuallyAllScopesSync ==
  \A scope \in scopes :
    \E future_status \in {"completed", "failed"} :
      []<>(sync_status[scope] = future_status)

\* No permanent deadlocks
NoPermanentDeadlocks ==
  \A agent \in agents :
    ~[]<>(agent_states[agent] = "blocked")

\* All temporal properties combined
AllTemporalProperties ==
  /\ EventuallyAllAgentsProgress
  /\ EventuallyAllScopesSync
  /\ NoPermanentDeadlocks

\* Model checking configuration
\* These properties should be checked during model checking

\* Safety properties to verify
SafetyProperties ==
  /\ AllSafetyInvariants
  /\ SafetyInvariants  \* From RHEMA_Core

\* Liveness properties to verify
LivenessProperties ==
  /\ AllLivenessProperties
  /\ ContextSyncLiveness  \* From RHEMA_Core
  /\ AgentProgress        \* From RHEMA_Core

\* Temporal properties to verify
TemporalProperties ==
  /\ AllTemporalProperties

============================================================================= 