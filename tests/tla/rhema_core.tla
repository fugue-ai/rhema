---------------------------- MODULE RHEMA_Core ----------------------------
EXTENDS Naturals, Sequences, FiniteSets, TLC

CONSTANTS
  MaxConcurrentAgents,  \* Maximum agents per scope
  MaxBlockTime,         \* Maximum time an agent can be blocked
  Scope,                \* Set of all possible scopes
  Agent,                \* Set of all possible agents
  ContextFileTypes,     \* Set of context file types
  RepositoryFiles,      \* Set of all repository files
  Content,              \* Set of all possible content values
  GitOperationTypes     \* Set of Git operation types

VARIABLES
  scopes,           \* Set of all scopes
  agents,           \* Set of active agents
  git_state,        \* Git repository state
  context_files,    \* Context file contents
  dependencies,     \* Cross-scope dependencies
  locks,            \* Resource locks
  sync_status,      \* Synchronization status
  agent_states      \* Agent state information

vars == <<scopes, agents, git_state, context_files, dependencies, locks, sync_status, agent_states>>

\* Type definitions
ContextFile == [ContextFileTypes -> Content]
GitState == [RepositoryFiles -> Content]
AgentState == [Agent -> {"idle", "working", "blocked", "completed"}]
LockState == [Scope -> Agent \cup {null}]

\* Type invariant
TypeInvariant ==
  /\ scopes \in SUBSET(Scope)
  /\ agents \in SUBSET(Agent)
  /\ git_state \in GitState
  /\ context_files \in [Scope -> ContextFile]
  /\ dependencies \in [Scope -> SUBSET(Scope)]
  /\ locks \in LockState
  /\ sync_status \in [Scope -> {"idle", "syncing", "completed", "failed"}]
  /\ agent_states \in AgentState

\* Initial state
Init ==
  /\ scopes = {}
  /\ agents = {}
  /\ git_state = [file \in RepositoryFiles |-> ""]
  /\ context_files = [scope \in {} |-> [file \in ContextFileTypes |-> ""]]
  /\ dependencies = [scope \in {} |-> {}]
  /\ locks = [scope \in Scope |-> null]
  /\ sync_status = [scope \in Scope |-> "idle"]
  /\ agent_states = [agent \in Agent |-> "idle"]

\* Helper functions
ValidYAML(content) == content \in Content
ValidScopeReferences(scope, all_scopes) == scope \in all_scopes

\* Check if a scope is part of a cycle (matches implementation)
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

\* Check for circular dependencies (matches implementation)
HasCircularDependencies(deps) ==
  \E scope \in scopes : IsInCycle(scope, deps, scopes)

NoCircularDependencies(deps) == ~HasCircularDependencies(deps)

\* Check if two agents are deadlocked
AreDeadlocked(agent1, agent2, locks, deps) ==
  LET scope1 == CHOOSE scope \in DOMAIN locks : locks[scope] = agent1
       scope2 == CHOOSE scope \in DOMAIN locks : locks[scope] = agent2
  IN scope1 \in deps[scope2] /\ scope2 \in deps[scope1]

\* Safety invariants
ContextConsistency ==
  /\ \A scope \in scopes : ValidYAML(context_files[scope])
  /\ \A scope \in scopes : NoCircularDependencies(dependencies[scope])
  /\ \A scope \in scopes : ValidScopeReferences(scope, scopes)

DependencyIntegrity ==
  /\ \A scope \in scopes : 
       \A dep \in dependencies[scope] : dep \in scopes
  /\ ~HasCircularDependencies(dependencies)

AgentCoordination ==
  /\ \A agent \in agents : agent_states[agent] \in {"idle", "working", "blocked", "completed"}
  /\ \A scope \in scopes : 
       Cardinality({agent \in agents : locks[scope] = agent}) <= MaxConcurrentAgents

\* All safety invariants
SafetyInvariants ==
  /\ ContextConsistency
  /\ DependencyIntegrity
  /\ AgentCoordination

\* Actions

\* Agent joins the system
AgentJoin(agent) ==
  /\ agent \in Agent
  /\ agent \notin agents
  /\ agents' = agents \cup {agent}
  /\ agent_states' = [agent_states EXCEPT ![agent] = "idle"]
  /\ UNCHANGED <<scopes, git_state, context_files, dependencies, locks, sync_status>>

\* Agent leaves the system
AgentLeave(agent) ==
  /\ agent \in agents
  /\ \A scope \in scopes : locks[scope] \neq agent
  /\ agents' = agents \ {agent}
  /\ agent_states' = [agent_states EXCEPT ![agent] = "idle"]
  /\ UNCHANGED <<scopes, git_state, context_files, dependencies, locks, sync_status>>

\* Context modification
ContextModify(agent, scope, file, content) ==
  /\ agent \in agents
  /\ scope \in scopes
  /\ file \in ContextFileTypes
  /\ locks[scope] = agent \/ locks[scope] = null
  /\ ValidYAML(content)
  /\ context_files' = [context_files EXCEPT ![scope][file] = content]
  /\ locks' = [locks EXCEPT ![scope] = agent]
  /\ agent_states' = [agent_states EXCEPT ![agent] = "working"]
  /\ UNCHANGED <<scopes, agents, git_state, dependencies, sync_status>>

\* Cross-scope synchronization
CrossScopeSync(scope) ==
  /\ scope \in scopes
  /\ sync_status[scope] = "idle"
  /\ \A dep \in dependencies[scope] : sync_status[dep] = "completed"
  /\ sync_status' = [sync_status EXCEPT ![scope] = "syncing"]
  /\ UNCHANGED <<scopes, agents, git_state, context_files, dependencies, locks, agent_states>>

\* Complete synchronization
CompleteSync(scope) ==
  /\ scope \in scopes
  /\ sync_status[scope] = "syncing"
  /\ sync_status' = [sync_status EXCEPT ![scope] = "completed"]
  /\ UNCHANGED <<scopes, agents, git_state, context_files, dependencies, locks, agent_states>>

\* Fail synchronization
FailSync(scope) ==
  /\ scope \in scopes
  /\ sync_status[scope] = "syncing"
  /\ sync_status' = [sync_status EXCEPT ![scope] = "failed"]
  /\ UNCHANGED <<scopes, agents, git_state, context_files, dependencies, locks, agent_states>>

\* Git operation
GitOperation(op_type, files) ==
  /\ op_type \in GitOperationTypes
  /\ files \subseteq RepositoryFiles
  /\ git_state' = [git_state EXCEPT ![file \in files] = "updated"]
  /\ \A scope \in scopes : 
       IF scope \in files 
       THEN sync_status'[scope] = "idle"
       ELSE UNCHANGED sync_status[scope]
  /\ UNCHANGED <<scopes, agents, context_files, dependencies, locks, agent_states>>

\* Next state relation
Next ==
  \/ \E agent \in Agent : AgentJoin(agent)
  \/ \E agent \in agents : AgentLeave(agent)
  \/ \E agent \in agents, scope \in scopes, file \in ContextFileTypes, content \in Content :
       ContextModify(agent, scope, file, content)
  \/ \E scope \in scopes : CrossScopeSync(scope)
  \/ \E scope \in scopes : CompleteSync(scope)
  \/ \E scope \in scopes : FailSync(scope)
  \/ \E op_type \in GitOperationTypes, files \in SUBSET(RepositoryFiles) :
       GitOperation(op_type, files)

\* Specification
Spec == Init /\ [][Next]_vars

\* Liveness properties
ContextSyncLiveness ==
  \A scope \in scopes : 
    \E sync \in {"syncing", "completed"} : sync_status[scope] = sync

AgentProgress ==
  \A agent \in agents : 
    agent_states[agent] \in {"idle", "working", "completed"} \/
    (agent_states[agent] = "blocked" => \E time \in Nat : time < MaxBlockTime)

\* Fairness conditions
WF_vars(ContextModify) == WF_vars(CrossScopeSync) == WF_vars(CompleteSync) == WF_vars(FailSync)

============================================================================= 