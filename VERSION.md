# Version Bump Guidelines

Greetings. This document explains how this repository handles software versions.

This document is also to keep a list of files that must be updated on each version bump. Thus, reminding the maintainer to update version files as necessary on each version bump.

## Introduction

Since the project uses Git, one decided to version each software with git tags - `v*` - to denote each version. Tags that start with `v` denote versions. On each tag creation, the CI/CD is to start building release binaries.

Example: `v2026.1.1-0`

## Versioning

This project has adopted a custom Calendar Versioning (`CalVer`) approach. Each version also indicates the date of version release.

`YYYY.MM.DD-PATCH`

`PATCH` is basically the count of versions released on the day

### Examples

- First update released on 2026 September 20: `2026.9.20-0`
- Second update released on 2026 September 20: `2026.9.20-1`
- Third update released on Christmas 2026: `2026.12.25-3`
- First update released on April 1 2027: `2027.4.1-0`

## Files that track Versions

This is a list of files in the repository that has something to do with versioning and must be bumped on each version release.

- Cargo.toml
- desktop/in.org.dawn.puppet.appdata.xml

## About 🐈‍⬛

```txt
puppet
Last Updated: 20 Sep 2026

DAWN/ペンギン
```
