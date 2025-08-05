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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API documentation structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDocumentation {
    /// API version
    pub version: String,
    
    /// API title
    pub title: String,
    
    /// API description
    pub description: String,
    
    /// Base URL for the API
    pub base_url: String,
    
    /// Available endpoints
    pub endpoints: Vec<ApiEndpoint>,
    
    /// Data models
    pub models: Vec<ApiModel>,
    
    /// Error codes and descriptions
    pub error_codes: Vec<ApiErrorCode>,
    
    /// Authentication information
    pub authentication: Option<ApiAuthentication>,
    
    /// Rate limiting information
    pub rate_limiting: Option<ApiRateLimiting>,
    
    /// Examples
    pub examples: Vec<ApiExample>,
}

/// API endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    /// HTTP method
    pub method: String,
    
    /// Endpoint path
    pub path: String,
    
    /// Endpoint description
    pub description: String,
    
    /// Request parameters
    pub parameters: Vec<ApiParameter>,
    
    /// Request body schema
    pub request_body: Option<ApiSchema>,
    
    /// Response schemas
    pub responses: Vec<ApiResponse>,
    
    /// Authentication required
    pub auth_required: bool,
    
    /// Rate limiting information
    pub rate_limit: Option<String>,
    
    /// Examples
    pub examples: Vec<ApiExample>,
}

/// API parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub param_type: String,
    
    /// Parameter description
    pub description: String,
    
    /// Whether parameter is required
    pub required: bool,
    
    /// Default value
    pub default: Option<serde_yaml::Value>,
    
    /// Allowed values
    pub allowed_values: Option<Vec<serde_yaml::Value>>,
    
    /// Validation rules
    pub validation: Option<HashMap<String, serde_yaml::Value>>,
}

/// API schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchema {
    /// Schema type
    pub schema_type: String,
    
    /// Schema description
    pub description: String,
    
    /// Schema properties
    pub properties: Option<HashMap<String, ApiSchemaProperty>>,
    
    /// Required properties
    pub required: Option<Vec<String>>,
    
    /// Example value
    pub example: Option<serde_yaml::Value>,
}

/// API schema property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSchemaProperty {
    /// Property type
    pub property_type: String,
    
    /// Property description
    pub description: String,
    
    /// Whether property is required
    pub required: bool,
    
    /// Default value
    pub default: Option<serde_yaml::Value>,
    
    /// Allowed values
    pub allowed_values: Option<Vec<serde_yaml::Value>>,
    
    /// Validation rules
    pub validation: Option<HashMap<String, serde_yaml::Value>>,
}

/// API response definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    /// HTTP status code
    pub status_code: u16,
    
    /// Response description
    pub description: String,
    
    /// Response schema
    pub schema: Option<ApiSchema>,
    
    /// Response headers
    pub headers: Option<HashMap<String, String>>,
    
    /// Example response
    pub example: Option<serde_yaml::Value>,
}

/// API error code definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorCode {
    /// Error code
    pub code: String,
    
    /// HTTP status code
    pub status_code: u16,
    
    /// Error description
    pub description: String,
    
    /// Error message template
    pub message_template: String,
    
    /// Suggested resolution
    pub resolution: Option<String>,
    
    /// Example error response
    pub example: Option<serde_yaml::Value>,
}

/// API authentication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthentication {
    /// Authentication type
    pub auth_type: String,
    
    /// Authentication description
    pub description: String,
    
    /// Required headers
    pub headers: Vec<String>,
    
    /// Example authentication
    pub example: Option<serde_yaml::Value>,
}

/// API rate limiting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimiting {
    /// Rate limit description
    pub description: String,
    
    /// Requests per minute
    pub requests_per_minute: u32,
    
    /// Burst size
    pub burst_size: u32,
    
    /// Rate limit headers
    pub headers: Vec<String>,
    
    /// Rate limit response
    pub rate_limit_response: Option<serde_yaml::Value>,
}

/// API example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExample {
    /// Example name
    pub name: String,
    
    /// Example description
    pub description: String,
    
    /// Example request
    pub request: Option<serde_yaml::Value>,
    
    /// Example response
    pub response: Option<serde_yaml::Value>,
    
    /// Example curl command
    pub curl_command: Option<String>,
}

/// API model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiModel {
    /// Model name
    pub name: String,
    
    /// Model description
    pub description: String,
    
    /// Model schema
    pub schema: ApiSchema,
    
    /// Model examples
    pub examples: Vec<ApiExample>,
}

impl ApiDocumentation {
    /// Generate comprehensive API documentation for Rhema
    pub fn generate_rhema_api_docs() -> Self {
        Self {
            version: "1.0.0".to_string(),
            title: "Rhema API".to_string(),
            description: "A comprehensive API for managing AI agent context through distributed YAML files in Git repositories".to_string(),
            base_url: "https://api.rhema.dev/v1".to_string(),
            endpoints: Self::generate_endpoints(),
            models: Self::generate_models(),
            error_codes: Self::generate_error_codes(),
            authentication: Some(Self::generate_authentication()),
            rate_limiting: Some(Self::generate_rate_limiting()),
            examples: Self::generate_examples(),
        }
    }

    /// Generate API endpoints
    fn generate_endpoints() -> Vec<ApiEndpoint> {
        vec![
            ApiEndpoint {
                method: "POST".to_string(),
                path: "/query".to_string(),
                description: "Execute a CQL query against the repository".to_string(),
                parameters: vec![
                    ApiParameter {
                        name: "query".to_string(),
                        param_type: "string".to_string(),
                        description: "The CQL query to execute".to_string(),
                        required: true,
                        default: None,
                        allowed_values: None,
                        validation: Some(HashMap::from([
                            ("min_length".to_string(), serde_yaml::Value::Number(1.into())),
                            ("max_length".to_string(), serde_yaml::Value::Number(10000.into())),
                        ])),
                    },
                ],
                request_body: Some(ApiSchema {
                    schema_type: "object".to_string(),
                    description: "Query request body".to_string(),
                    properties: Some(HashMap::from([
                        ("query".to_string(), ApiSchemaProperty {
                            property_type: "string".to_string(),
                            description: "The CQL query to execute".to_string(),
                            required: true,
                            default: None,
                            allowed_values: None,
                            validation: None,
                        }),
                        ("include_provenance".to_string(), ApiSchemaProperty {
                            property_type: "boolean".to_string(),
                            description: "Whether to include query provenance information".to_string(),
                            required: false,
                            default: Some(serde_yaml::Value::Bool(false)),
                            allowed_values: None,
                            validation: None,
                        }),
                    ])),
                    required: Some(vec!["query".to_string()]),
                    example: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                }),
                responses: vec![
                    ApiResponse {
                        status_code: 200,
                        description: "Query executed successfully".to_string(),
                        schema: Some(ApiSchema {
                            schema_type: "object".to_string(),
                            description: "Query response".to_string(),
                            properties: Some(HashMap::from([
                                ("data".to_string(), ApiSchemaProperty {
                                    property_type: "object".to_string(),
                                    description: "Query results".to_string(),
                                    required: true,
                                    default: None,
                                    allowed_values: None,
                                    validation: None,
                                }),
                                ("provenance".to_string(), ApiSchemaProperty {
                                    property_type: "object".to_string(),
                                    description: "Query provenance information".to_string(),
                                    required: false,
                                    default: None,
                                    allowed_values: None,
                                    validation: None,
                                }),
                            ])),
                            required: Some(vec!["data".to_string()]),
                            example: None,
                        }),
                        headers: None,
                        example: None,
                    },
                    ApiResponse {
                        status_code: 400,
                        description: "Invalid query syntax".to_string(),
                        schema: None,
                        headers: None,
                        example: None,
                    },
                ],
                auth_required: false,
                rate_limit: Some("1000 requests per minute".to_string()),
                examples: vec![
                    ApiExample {
                        name: "Basic Query".to_string(),
                        description: "Execute a simple CQL query".to_string(),
                        request: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                        response: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                        curl_command: Some("curl -X POST https://api.rhema.dev/v1/query -H 'Content-Type: application/json' -d '{\"query\": \"SELECT * FROM todos\"}'".to_string()),
                    },
                ],
            },
            ApiEndpoint {
                method: "GET".to_string(),
                path: "/scopes".to_string(),
                description: "Discover all scopes in the repository".to_string(),
                parameters: vec![],
                request_body: None,
                responses: vec![
                    ApiResponse {
                        status_code: 200,
                        description: "Scopes discovered successfully".to_string(),
                        schema: None,
                        headers: None,
                        example: None,
                    },
                ],
                auth_required: false,
                rate_limit: Some("100 requests per minute".to_string()),
                examples: vec![],
            },
        ]
    }

    /// Generate API models
    fn generate_models() -> Vec<ApiModel> {
        vec![
            ApiModel {
                name: "Scope".to_string(),
                description: "A Rhema scope with its metadata and files".to_string(),
                schema: ApiSchema {
                    schema_type: "object".to_string(),
                    description: "Scope definition".to_string(),
                    properties: Some(HashMap::from([
                        ("path".to_string(), ApiSchemaProperty {
                            property_type: "string".to_string(),
                            description: "Path to the scope directory".to_string(),
                            required: true,
                            default: None,
                            allowed_values: None,
                            validation: None,
                        }),
                        ("definition".to_string(), ApiSchemaProperty {
                            property_type: "object".to_string(),
                            description: "Scope definition from rhema.yaml".to_string(),
                            required: true,
                            default: None,
                            allowed_values: None,
                            validation: None,
                        }),
                        ("files".to_string(), ApiSchemaProperty {
                            property_type: "object".to_string(),
                            description: "Available files in this scope".to_string(),
                            required: true,
                            default: None,
                            allowed_values: None,
                            validation: None,
                        }),
                    ])),
                    required: Some(vec!["path".to_string(), "definition".to_string(), "files".to_string()]),
                    example: None,
                },
                examples: vec![],
            },
        ]
    }

    /// Generate error codes
    fn generate_error_codes() -> Vec<ApiErrorCode> {
        vec![
            ApiErrorCode {
                code: "INVALID_QUERY".to_string(),
                status_code: 400,
                description: "Invalid query syntax".to_string(),
                message_template: "Invalid query syntax: {details}".to_string(),
                resolution: Some("Check the query syntax and ensure it follows CQL format".to_string()),
                example: None,
            },
            ApiErrorCode {
                code: "SCOPE_NOT_FOUND".to_string(),
                status_code: 404,
                description: "Scope not found".to_string(),
                message_template: "Scope not found: {scope_name}".to_string(),
                resolution: Some("Verify the scope name and ensure it exists in the repository".to_string()),
                example: None,
            },
            ApiErrorCode {
                code: "RATE_LIMIT_EXCEEDED".to_string(),
                status_code: 429,
                description: "Rate limit exceeded".to_string(),
                message_template: "Rate limit exceeded: {limit} requests per {period}".to_string(),
                resolution: Some("Wait before making additional requests or contact support for higher limits".to_string()),
                example: None,
            },
        ]
    }

    /// Generate authentication information
    fn generate_authentication() -> ApiAuthentication {
        ApiAuthentication {
            auth_type: "API Key".to_string(),
            description: "Authentication using API key in Authorization header".to_string(),
            headers: vec!["Authorization: Bearer <api_key>".to_string()],
            example: None,
        }
    }

    /// Generate rate limiting information
    fn generate_rate_limiting() -> ApiRateLimiting {
        ApiRateLimiting {
            description: "Rate limiting is applied per API key".to_string(),
            requests_per_minute: 1000,
            burst_size: 100,
            headers: vec![
                "X-RateLimit-Limit".to_string(),
                "X-RateLimit-Remaining".to_string(),
                "X-RateLimit-Reset".to_string(),
            ],
            rate_limit_response: None,
        }
    }

    /// Generate examples
    fn generate_examples() -> Vec<ApiExample> {
        vec![
            ApiExample {
                name: "Query Todos".to_string(),
                description: "Query all todos in the repository".to_string(),
                request: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                response: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                curl_command: Some("curl -X POST https://api.rhema.dev/v1/query -H 'Content-Type: application/json' -d '{\"query\": \"SELECT * FROM todos\"}'".to_string()),
            },
            ApiExample {
                name: "Query with Conditions".to_string(),
                description: "Query todos with specific conditions".to_string(),
                request: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                response: Some(serde_yaml::Value::Mapping(serde_yaml::Mapping::new())),
                curl_command: Some("curl -X POST https://api.rhema.dev/v1/query -H 'Content-Type: application/json' -d '{\"query\": \"SELECT * FROM todos WHERE priority = 'high'\"}'".to_string()),
            },
        ]
    }

    /// Export documentation as OpenAPI/Swagger specification
    pub fn to_openapi(&self) -> serde_yaml::Value {
        // This would generate a complete OpenAPI 3.0 specification
        // For now, return a basic structure
        serde_yaml::Value::Mapping(serde_yaml::Mapping::new())
    }

    /// Export documentation as Markdown
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        
        markdown.push_str(&format!("# {}\n\n", self.title));
        markdown.push_str(&format!("**Version:** {}\n\n", self.version));
        markdown.push_str(&format!("{}\n\n", self.description));
        
        // Endpoints
        markdown.push_str("## Endpoints\n\n");
        for endpoint in &self.endpoints {
            markdown.push_str(&format!("### {} {}\n\n", endpoint.method, endpoint.path));
            markdown.push_str(&format!("{}\n\n", endpoint.description));
            
            if !endpoint.parameters.is_empty() {
                markdown.push_str("#### Parameters\n\n");
                for param in &endpoint.parameters {
                    markdown.push_str(&format!("- **{}** (`{}`) - {}\n", 
                        param.name, param.param_type, param.description));
                }
                markdown.push_str("\n");
            }
            
            for response in &endpoint.responses {
                markdown.push_str(&format!("- **{}** - {}\n", 
                    response.status_code, response.description));
            }
            markdown.push_str("\n");
        }
        
        // Error codes
        markdown.push_str("## Error Codes\n\n");
        for error in &self.error_codes {
            markdown.push_str(&format!("### {}\n\n", error.code));
            markdown.push_str(&format!("**Status Code:** {}\n\n", error.status_code));
            markdown.push_str(&format!("{}\n\n", error.description));
            if let Some(ref resolution) = error.resolution {
                markdown.push_str(&format!("**Resolution:** {}\n\n", resolution));
            }
        }
        
        markdown
    }
}

/// API documentation generator
pub struct ApiDocGenerator;

impl ApiDocGenerator {
    /// Generate and save API documentation
    pub fn generate_docs(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let docs = ApiDocumentation::generate_rhema_api_docs();
        
        // Save as YAML
        let yaml_content = serde_yaml::to_string(&docs)?;
        std::fs::write(format!("{}.yaml", output_path), yaml_content)?;
        
        // Save as Markdown
        let markdown_content = docs.to_markdown();
        std::fs::write(format!("{}.md", output_path), markdown_content)?;
        
        // Save as OpenAPI
        let openapi_content = serde_yaml::to_string(&docs.to_openapi())?;
        std::fs::write(format!("{}-openapi.yaml", output_path), openapi_content)?;
        
        Ok(())
    }
} 