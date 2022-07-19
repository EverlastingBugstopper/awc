# .xtask

The Cargo.toml in this directory is copied into our Docker builder image before our other binary. This allows the layer to only be rebuilt when xtask source changes (which in theory should change less than the actual source code).
