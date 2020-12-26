## Apify API Client in Rust
Work in progress!

Typed wrapper for [Apify API](https://docs.apify.com/api/v2). This client is not yet stabilized so expect a few breaking minor versions before updating to 1.0.

Currently implemented:
- Client 
    - Exponential backoff
    - Error types (not complete)
- Datasets
    - List datasets
    - Create dataset
    - Get dataset
    - Update dataset
    - Delete dataset
    - Get items
    - Put items