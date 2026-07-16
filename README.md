# mk — DevSecOps CLI

![Rust CI](https://github.com/monokkai/devsecops-cli/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue)
![Version](https://img.shields.io/badge/version-0.1.0-orange)

Fast Rust-powered CLI for git workflows, Docker, security scanning, and HTTP — all in one tool.

## Installation

```bash
cargo install --path .
```

---

## Git

```bash
mk git "my message"       # stage all, commit, push
mk git --ap               # commitizen interactive flow + stage + push
mk git --pull             # pull
mk git --pull --rebase    # pull with rebase
mk log                    # pretty commit history
mk log --graph -l 10      # graph view, last 10 commits
mk log --compact          # one line per commit
```

## Docker

```bash
mk docker scan --image alpine:latest          # security scan with Trivy
mk docker push --image myapp --tag v1.0       # build and push to DockerHub
```

Requires `DOCKERHUB_USER` in `.env`.

## Security Scanning

```bash
mk scan -p ./path/to/scan    # scan directory for vulnerabilities
```

## Auth

```bash
mk auth jwt --token "your.jwt.token"      # validate JWT
mk auth github --token "ghp_yourtoken"   # verify GitHub token
```

## HTTP

```bash
mk http get https://example.com
mk http post https://api.example.com/resource --body '{"key":"value"}' --headers "Authorization: Bearer token"
mk http delete https://api.example.com/resource/1 --headers "Authorization: Bearer token"
mk http head https://api.example.com
mk http options https://api.example.com
```
