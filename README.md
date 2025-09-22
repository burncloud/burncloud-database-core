# burncloud-database-core

Core database abstractions and traits for the BurnCloud AI management system.

## Overview

`burncloud-database-core` provides the foundational traits, types, and data models for building database-agnostic AI model management systems. It defines the core abstractions that allow different database backends to be used interchangeably.

## Features

- **Database-agnostic traits**: Core traits for database connections, query execution, and transactions
- **AI Model data models**: Complete data structures for AI model management, deployments, monitoring, and security
- **Error handling**: Comprehensive error types for database operations
- **Serialization support**: Full serde support for all data types
- **Async/await support**: Built for modern async Rust applications

## Core Traits

- `DatabaseConnection`: Manages database connections and basic operations
- `QueryExecutor`: Executes queries and handles parameters
- `TransactionManager`: Manages database transactions
- `Repository`: Generic repository pattern for data access
- `MigrationManager`: Handles database schema migrations

## Data Models

### AI Model Management
- `AiModel`: Represents AI models with metadata, requirements, and status
- `ModelDeployment`: Manages model deployments with configuration and resource settings
- `ModelType`: Enum for different types of AI models (ChatCompletion, TextGeneration, etc.)

### Monitoring & Metrics
- `SystemMetrics`: System-level performance metrics
- `ModelMetrics`: Model-specific performance metrics
- `RequestLog`: API request logging
- `SystemLog`: System event logging

### User & Security
- `UserSettings`: User preferences and configuration
- `SecurityConfig`: Security policies and access controls
- `ApiKey`: API key management with permissions
- `FirewallRule`: Network access control rules

## Example Usage

```rust
use burncloud_database_core::*;

// Define a custom repository
struct MyModelRepository {
    // Implementation details
}

#[async_trait]
impl Repository<AiModel> for MyModelRepository {
    async fn find_by_id(&self, id: &str, context: &QueryContext) -> DatabaseResult<Option<AiModel>> {
        // Implementation
    }

    // Other repository methods...
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
burncloud-database-core = "0.1"
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions."# burncloud-database-core" 
