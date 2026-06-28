# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Email or DM the maintainer privately with:

1. Description of the vulnerability
2. Steps to reproduce
3. Impact assessment
4. Suggested fix (if any)

We aim to acknowledge reports within 72 hours and provide a fix or mitigation plan as soon as possible.

## Scope

shipflow is a local-first CLI with **no network calls by default**. Security concerns are primarily limited to:

- Local file read/write paths (`.shipflow/tasks.json`, global config)
- Git subprocess invocation (user-controlled repo context)
- Terminal escape sequences in output

Out of scope: third-party git hosting, crates.io infrastructure, GitHub Actions supply chain (report those to the respective vendors).