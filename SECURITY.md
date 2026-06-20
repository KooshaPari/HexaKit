# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| latest  | :white_check_mark: |
| <latest | :x:                |

The HexaKit project provides security updates for the latest release only.
Please upgrade before reporting a vulnerability against an older version.

## Reporting a Vulnerability

If you discover a security vulnerability, please report it **privately** so we
can investigate and ship a fix before public disclosure:

1. **Do not** open a public GitHub issue for the vulnerability.
2. Open a private [GitHub Security Advisory][adv] for this repository
   (`KooshaPari/HexaKit`).
3. Or, if you cannot use GitHub Advisories, email the maintainers at the
   address in the repository's `CODEOWNERS` / `package.json` / Cargo manifest.
4. Allow up to **90 days** for assessment and remediation before any public
   disclosure. We will coordinate disclosure timing with you.

Please include:

- A clear description of the vulnerability and its impact.
- Reproduction steps or a minimal proof-of-concept.
- Affected version(s) and commit(s).
- Your name / handle (for credit in the advisory, if desired).

[adv]: https://github.com/KooshaPari/HexaKit/security/advisories/new

## Response Process

1. **Acknowledgement** within 72 hours of report.
2. **Triage** and severity assessment (CVSS) within 7 days.
3. **Patch** shipped for `latest`; backport considered for `latest-1` if
   severity is High or Critical.
4. **Public advisory** published with credit, severity, fix version, and
   mitigation guidance.

## Severity Tiers

| Severity    | Response SLA | Disclosure Window |
|-------------|--------------|-------------------|
| Critical    | 24 hours     | 7 days            |
| High        | 72 hours     | 30 days           |
| Medium      | 7 days       | 90 days           |
| Low         | 30 days      | 90 days           |

## Scope

In scope: code in this repository, GitHub Actions workflows, and release
artifacts. Out of scope: third-party dependencies (please report upstream).

## Recognition

We follow a [hall of fame][hof] approach — reporters who follow responsible
disclosure are credited in release notes unless they prefer anonymity.

[hof]: https://github.com/KooshaPari/HexaKit/security/policy
