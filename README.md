# CPAL Mixer Performance Regression Demo

This repository demonstrates a severe performance regression when using CPAL git version (commit a8269d3) versus crates.io version 0.16.0.

## Problem
- **crates.io CPAL 0.16.0**: All sounds play instantly
- **git CPAL (commit a8269d3)**: First sound plays instantly, subsequent sounds have 10-15 second delays

## Root Cause
The `mixer().clone()` call on line 45 of `audio.rs` triggers expensive device enumeration in the git version, blocking for 10-15 seconds on each call after the first.

## Reproduction
1. Clone this repo
2. Run with `cargo run --bin mini-client`
3. Press hotkey after startup - observe 10-15s delay
4. Change `Cargo.toml` to use `cpal = "0.16.0"`
5. Run again - observe instant playback

## Technical Details
The bottleneck occurs at:
```
stream_lock.as_ref().unwrap().mixer().clone() // Line 45 - blocks 10-15s with git CPAL
```

This suggests the git version performs device validation/enumeration on every mixer access, while crates.io version caches this information.
