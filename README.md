# DevSecOps CLI (mk) Toolkit 🔒

![Rust CI](https://github.com/monokkai/devsecops-cli/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue)
![Version](https://img.shields.io/badge/version-0.1.0-orange)

A Swiss Army knife for modern DevSecOps workflows, combining security scanning, container management, and automation in
one fast Rust-powered CLI.

## 🚀 Features

- **Security First**:

  - Code vulnerability scanning
  - JWT/GitHub token validation
  - Docker image security audits

- **DevOps Automation**:

  - Smart Git workflows
  - Docker build/push pipelines
  - CI/CD ready architecture

- **Enterprise Ready**:
  - Async I/O where it matters
  - Proper error handling
  - Configurable via ENV

## 📦 Installation

### From Source

```bash
cargo install --path .
```

# Commands

```zsh
# Security Scanning
mk scan -p ./path/to/scan  # Scan directory for vulnerabilities

# Docker Operations
mk docker scan --image alpine:latest  # Scan Docker image
mk docker push --image myapp --tag v1.0  # Build and push to DockerHub

# Authentication
mk auth jwt --token "your.jwt.token"  # Validate JWT
mk auth github --token "ghp_yourtoken"  # Verify GitHub token

# Git Automation
mk git -m "commit message" [--push]  # Commit (and optionally push)
mk git -a -m "My commit message" --push # Already contains 'git add .' flag
mk git --pull --rebase -a -m "message" -p
mk git --pull -m "message"
mk log # Default history of commits
mk log --graph -l 5 # The last 5 (for example)
mk log --compact # Hash and message
mk log --graph # With graph (for example if u're commiting with others)

# Git CLI like commitizen interface
# This git line tool only made for make work easier and won't gonna be sold or paid!
mk git cz # Basic commit
mk git cz -a -p # With git add . and git push
mk git cz -a # only commit
mk git cz --pull # with pull
```

# Http simple requests

```zsh
mk http get https://github.com/monokkai
mk http get https://example.com
mk http delete https://api.example.com/resource/1 --headers "Authorization: Bearer token"
mk http head https://api.example.com --headers "Accept: application/json"
mk http options https://api.example.com
```
