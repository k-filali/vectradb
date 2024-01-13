## VectraDB
Web-optimized vector database for AI applications.
## Features

1. Rust API (using native filesystem, or a transient in-memory filesystem)
2. Web API (Using the [Private Origin File System](https://web.dev/origin-private-file-system/))
3. Optimized Vector Storage
4. PCA for vector compression when storage space is low


#### Installation

```
cargo add Vectra-db
```

#### Usage

```rust
use std::path::PathBuf;

use Vectra_db::native::Db;

let _ = std::fs::create_dir("./Vectra_test_data");
let mut Vectra = Db::new(PathBuf::from("./Vectra_test_data"));

Vectra.clear_db().await.unwrap();

Vectra
    .write(
        "Test Vector 1",
        vec![1.0, 0.0, 0.0],
        vec!["Test".to_string()],
    )
    .await;
Vectra
    .write(
        "Test Vector 2",
        vec![0.0, 1.0, 0.0],
        vec!["Test".to_string()],
    )
    .await;

// read the 10 closest results from Vectra that are tagged with "tags"
// (only 2 will be returned because we only inserted two embeddings)
let nearest = Vectra
   .find_nearest_neighbors(vec![0.9, 0.0, 0.0], vec!["Test".to_string()], 10)
   .await
   .first()
   .unwrap()
   .content
   .clone();
assert_eq!(nearest, "Test Vector 1".to_string());
```
