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

// Core Context Types

export interface WorkspaceContext {
  scopes: Scope[];
  contexts: Context[];
  todos: Todo[];
  insights: Insight[];
  patterns: Pattern[];
  decisions: Decision[];
  metadata: WorkspaceMetadata;
}

export interface Scope {
  id: string;
  name: string;
  description?: string;
  file: string;
  parent?: string;
  children: string[];
  metadata: ScopeMetadata;
}

export interface Context {
  id: string;
  name: string;
  description?: string;
  files: string[];
  scope: string;
  metadata: ContextMetadata;
}

export interface Todo {
  id: string;
  title: string;
  description?: string;
  priority: TodoPriority;
  status: TodoStatus;
  scope: string;
  assignee?: string;
  dueDate?: Date;
  metadata: TodoMetadata;
}

export interface Insight {
  id: string;
  title: string;
  description: string;
  type: InsightType;
  scope: string;
  confidence: number;
  metadata: InsightMetadata;
}

export interface Pattern {
  id: string;
  name: string;
  description: string;
  category: PatternCategory;
  scope: string;
  usage: number;
  metadata: PatternMetadata;
}

export interface Decision {
  id: string;
  title: string;
  description: string;
  rationale: string;
  scope: string;
  status: DecisionStatus;
  metadata: DecisionMetadata;
}

// Semantic Context Types

export interface SemanticContext {
  entities: SemanticEntity[];
  relationships: SemanticRelationship[];
  patterns: SemanticPattern[];
  topics: SemanticTopic[];
  metadata: SemanticMetadata;
}

export interface SemanticEntity {
  id: string;
  name: string;
  type: EntityType;
  confidence: number;
  source: string;
  metadata: EntityMetadata;
}

export interface SemanticRelationship {
  id: string;
  source: string;
  target: string;
  type: RelationshipType;
  strength: number;
  metadata: RelationshipMetadata;
}

export interface SemanticPattern {
  id: string;
  name: string;
  description: string;
  confidence: number;
  occurrences: number;
  metadata: PatternMetadata;
}

export interface SemanticTopic {
  id: string;
  name: string;
  description: string;
  relevance: number;
  entities: string[];
  metadata: TopicMetadata;
}

// Context Index Types

export interface ContextIndex {
  files: Map<string, FileIndex>;
  symbols: Map<string, SymbolIndex>;
  references: Map<string, ReferenceIndex>;
  dependencies: Map<string, DependencyIndex>;
  metadata: IndexMetadata;
}

export interface FileIndex {
  path: string;
  content: string;
  symbols: string[];
  dependencies: string[];
  lastModified: Date;
  metadata: FileMetadata;
}

export interface SymbolIndex {
  name: string;
  type: SymbolType;
  location: SymbolLocation;
  references: string[];
  metadata: SymbolMetadata;
}

export interface ReferenceIndex {
  symbol: string;
  locations: ReferenceLocation[];
  count: number;
  metadata: ReferenceMetadata;
}

export interface DependencyIndex {
  source: string;
  target: string;
  type: DependencyType;
  strength: number;
  metadata: DependencyMetadata;
}

// Context Suggestion Types

export interface ContextSuggestion {
  id: string;
  type: SuggestionType;
  title: string;
  description: string;
  relevance: number;
  confidence: number;
  source: SuggestionSource;
  action?: SuggestionAction;
  metadata: SuggestionMetadata;
}

export interface CompletionContext {
  query: string;
  document: string;
  position: Position;
  scope: string;
  history: UserAction[];
  metadata: CompletionMetadata;
}

// Cross-Scope Types

export interface ScopeDependencyMap {
  [scopeId: string]: ScopeDependency[];
}

export interface ScopeDependency {
  source: string;
  target: string;
  type: DependencyType;
  strength: number;
  metadata: DependencyMetadata;
}

export interface UnifiedWorkspaceContext {
  scopes: UnifiedScope[];
  relationships: ScopeRelationship[];
  metadata: UnifiedMetadata;
}

export interface UnifiedScope {
  id: string;
  name: string;
  contexts: Context[];
  dependencies: string[];
  metadata: UnifiedScopeMetadata;
}

export interface ScopeRelationship {
  id: string;
  source: string;
  target: string;
  type: RelationshipType;
  strength: number;
  metadata: RelationshipMetadata;
}

// AI Context Types

export interface SemanticAnalysis {
  entities: SemanticEntity[];
  relationships: SemanticRelationship[];
  topics: SemanticTopic[];
  confidence: number;
  metadata: AnalysisMetadata;
}

export interface UserAction {
  id: string;
  type: ActionType;
  timestamp: Date;
  context: string;
  metadata: ActionMetadata;
}

export interface PredictedContext {
  id: string;
  type: PredictionType;
  confidence: number;
  context: string;
  metadata: PredictionMetadata;
}

export interface UserFeedback {
  id: string;
  suggestionId: string;
  rating: number;
  comment?: string;
  timestamp: Date;
  metadata: FeedbackMetadata;
}

// Performance Types

export interface ContextTask {
  id: string;
  type: TaskType;
  priority: TaskPriority;
  changes?: FileChange[];
  metadata: TaskMetadata;
}

export interface ProgressInfo {
  taskId: string;
  progress: number;
  status: TaskStatus;
  message: string;
  metadata: ProgressMetadata;
}

export interface CacheMetrics {
  hitRate: number;
  missRate: number;
  totalRequests: number;
  averageResponseTime: number;
  metadata: MetricsMetadata;
}

// File Change Types

export interface FileChange {
  file: string;
  type: ChangeType;
  timestamp: Date;
  metadata?: ChangeMetadata;
}

// Metadata Types

export interface WorkspaceMetadata {
  analyzedAt: Date;
  totalScopes: number;
  totalContexts: number;
  totalTodos: number;
  totalInsights: number;
  totalPatterns: number;
  totalDecisions: number;
}

export interface ScopeMetadata {
  created: Date;
  modified: Date;
  size: number;
  complexity: number;
}

export interface ContextMetadata {
  created: Date;
  modified: Date;
  fileCount: number;
  relevance: number;
}

export interface TodoMetadata {
  created: Date;
  modified: Date;
  estimatedTime?: number;
  tags: string[];
}

export interface InsightMetadata {
  created: Date;
  modified: Date;
  source: string;
  tags: string[];
}

export interface PatternMetadata {
  created: Date;
  modified: Date;
  category: string;
  tags: string[];
}

export interface DecisionMetadata {
  created: Date;
  modified: Date;
  stakeholders: string[];
  impact: DecisionImpact;
}

export interface SemanticMetadata {
  analyzedAt: Date;
  totalFiles: number;
  totalEntities: number;
  confidence: number;
}

export interface EntityMetadata {
  created: Date;
  modified: Date;
  source: string;
  confidence: number;
}

export interface RelationshipMetadata {
  created: Date;
  modified: Date;
  confidence: number;
  bidirectional: boolean;
}

export interface TopicMetadata {
  created: Date;
  modified: Date;
  relevance: number;
  entityCount: number;
}

export interface IndexMetadata {
  indexedAt: Date;
  totalFiles: number;
  totalSymbols: number;
}

export interface FileMetadata {
  size: number;
  lastModified: Date;
  checksum: string;
}

export interface SymbolMetadata {
  created: Date;
  modified: Date;
  scope: string;
  visibility: SymbolVisibility;
}

export interface ReferenceMetadata {
  created: Date;
  modified: Date;
  count: number;
}

export interface DependencyMetadata {
  created: Date;
  modified: Date;
  strength: number;
  bidirectional: boolean;
}

export interface SuggestionMetadata {
  created: Date;
  modified: Date;
  source: string;
  tags: string[];
}

export interface CompletionMetadata {
  timestamp: Date;
  context: string;
  scope: string;
}

export interface UnifiedMetadata {
  unifiedAt: Date;
  totalScopes: number;
}

export interface UnifiedScopeMetadata {
  created: Date;
  modified: Date;
  contextCount: number;
  dependencyCount: number;
}

export interface AnalysisMetadata {
  analyzedAt: Date;
  confidence: number;
  source: string;
}

export interface ActionMetadata {
  timestamp: Date;
  context: string;
  scope: string;
}

export interface PredictionMetadata {
  created: Date;
  confidence: number;
  source: string;
}

export interface FeedbackMetadata {
  timestamp: Date;
  userId?: string;
  context: string;
}

export interface TaskMetadata {
  created: Date;
  priority: TaskPriority;
  estimatedTime?: number;
}

export interface ProgressMetadata {
  timestamp: Date;
  taskId: string;
  status: TaskStatus;
}

export interface MetricsMetadata {
  timestamp: Date;
  period: string;
  source: string;
}

export interface ChangeMetadata {
  size?: number;
  checksum?: string;
  author?: string;
}

// Enums

export enum TodoPriority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

export enum TodoStatus {
  Pending = 'pending',
  InProgress = 'in_progress',
  Completed = 'completed',
  Cancelled = 'cancelled'
}

export enum InsightType {
  Performance = 'performance',
  Security = 'security',
  Architecture = 'architecture',
  CodeQuality = 'code_quality',
  Business = 'business'
}

export enum PatternCategory {
  Architectural = 'architectural',
  Design = 'design',
  Implementation = 'implementation',
  Testing = 'testing',
  Deployment = 'deployment'
}

export enum DecisionStatus {
  Proposed = 'proposed',
  Approved = 'approved',
  Rejected = 'rejected',
  Implemented = 'implemented',
  Deprecated = 'deprecated'
}

export enum EntityType {
  Class = 'class',
  Function = 'function',
  Variable = 'variable',
  Module = 'module',
  Package = 'package',
  File = 'file',
  Directory = 'directory'
}

export enum RelationshipType {
  Inheritance = 'inheritance',
  Composition = 'composition',
  Aggregation = 'aggregation',
  Dependency = 'dependency',
  Association = 'association',
  Import = 'import',
  Reference = 'reference'
}

export enum SymbolType {
  Class = 'class',
  Function = 'function',
  Variable = 'variable',
  Constant = 'constant',
  Interface = 'interface',
  Type = 'type',
  Namespace = 'namespace'
}

export enum DependencyType {
  Import = 'import',
  Reference = 'reference',
  Inheritance = 'inheritance',
  Composition = 'composition',
  Aggregation = 'aggregation'
}

export enum SuggestionType {
  Completion = 'completion',
  Refactoring = 'refactoring',
  Optimization = 'optimization',
  Documentation = 'documentation',
  Testing = 'testing'
}

export enum SuggestionSource {
  AI = 'ai',
  Cache = 'cache',
  CrossScope = 'cross_scope',
  Pattern = 'pattern',
  History = 'history'
}

export enum ActionType {
  Completion = 'completion',
  Navigation = 'navigation',
  Refactoring = 'refactoring',
  Search = 'search',
  Edit = 'edit'
}

export enum PredictionType {
  NextAction = 'next_action',
  ContextNeed = 'context_need',
  Refactoring = 'refactoring',
  Documentation = 'documentation'
}

export enum TaskType {
  ContextUpdate = 'context_update',
  Indexing = 'indexing',
  Analysis = 'analysis',
  Optimization = 'optimization'
}

export enum TaskPriority {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

export enum TaskStatus {
  Pending = 'pending',
  Running = 'running',
  Completed = 'completed',
  Failed = 'failed',
  Cancelled = 'cancelled'
}

export enum ChangeType {
  Created = 'created',
  Modified = 'modified',
  Deleted = 'deleted',
  Renamed = 'renamed'
}

export enum SymbolVisibility {
  Public = 'public',
  Private = 'private',
  Protected = 'protected',
  Internal = 'internal'
}

export enum DecisionImpact {
  Low = 'low',
  Medium = 'medium',
  High = 'high',
  Critical = 'critical'
}

// Position and Location Types

export interface Position {
  line: number;
  character: number;
}

export interface SymbolLocation {
  file: string;
  range: Range;
}

export interface ReferenceLocation {
  file: string;
  range: Range;
}

export interface Range {
  start: Position;
  end: Position;
}

// Suggestion Action Types

export interface SuggestionAction {
  type: ActionType;
  title: string;
  command: string;
  arguments?: any[];
  metadata?: any;
} 