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

use rhema_core::RhemaResult;
use serde::{Deserialize, Serialize};

/// Compression algorithm types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// LZ4 compression
    Lz4,
    /// Gzip compression
    Gzip,
    /// Zstd compression
    Zstd,
    /// Snappy compression
    Snappy,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level (1-9)
    pub level: u8,
    /// Minimum size threshold for compression
    pub threshold_bytes: u64,
    /// Enable adaptive compression
    pub enable_adaptive: bool,
    /// Enable compression metrics
    pub enable_metrics: bool,
}

/// Message compressor
pub struct MessageCompressor {
    config: CompressionConfig,
}

impl MessageCompressor {
    /// Create a new message compressor
    pub async fn new(config: CompressionConfig) -> RhemaResult<Self> {
        Ok(Self { config })
    }

    /// Compress data
    pub async fn compress(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        // Check if data meets compression threshold
        if data.len() < self.config.threshold_bytes as usize {
            return Ok(data.to_vec());
        }

        match self.config.algorithm {
            CompressionAlgorithm::Lz4 => {
                // In a real implementation, this would use LZ4 compression
                let mut compressed = vec![0x01]; // Compression header
                compressed.extend_from_slice(data);
                Ok(compressed)
            }
            CompressionAlgorithm::Gzip => {
                // In a real implementation, this would use Gzip compression
                let mut compressed = vec![0x02]; // Compression header
                compressed.extend_from_slice(data);
                Ok(compressed)
            }
            CompressionAlgorithm::Zstd => {
                // In a real implementation, this would use Zstd compression
                let mut compressed = vec![0x03]; // Compression header
                compressed.extend_from_slice(data);
                Ok(compressed)
            }
            CompressionAlgorithm::Snappy => {
                // In a real implementation, this would use Snappy compression
                let mut compressed = vec![0x04]; // Compression header
                compressed.extend_from_slice(data);
                Ok(compressed)
            }
        }
    }

    /// Decompress data
    pub async fn decompress(&self, data: &[u8]) -> RhemaResult<Vec<u8>> {
        if data.is_empty() {
            return Err(rhema_core::RhemaError::InvalidInput(
                "Empty data".to_string(),
            ));
        }

        match data[0] {
            0x01 => {
                // LZ4 decompression
                Ok(data[1..].to_vec())
            }
            0x02 => {
                // Gzip decompression
                Ok(data[1..].to_vec())
            }
            0x03 => {
                // Zstd decompression
                Ok(data[1..].to_vec())
            }
            0x04 => {
                // Snappy decompression
                Ok(data[1..].to_vec())
            }
            _ => Err(rhema_core::RhemaError::InvalidInput(
                "Unknown compression format".to_string(),
            )),
        }
    }
}
