# Resolve Dependabot vulnerabilities

1. Upgrade all lockfile dependencies that have patched releases compatible with the workspace MSRV.
2. Move Tendermint RPC from Reqwest 0.11 to 0.12 so its TLS stack uses patched Rustls WebPKI.
3. Backport the upstream `time` recursion-depth fix without raising the workspace's Rust 1.75 MSRV.
4. Verify the resolved graph with Cargo audit, dependency-tree assertions, formatting, and the repository test matrix.
5. Publish a focused pull request and monitor its hosted checks.
