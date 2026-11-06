# Security Policy

## Reporting

Do not disclose vulnerabilities in public issues. Contact the repository maintainers through a private GitHub security advisory.

## Baseline

- Never commit credentials or production secrets.
- Treat browser/WASM configuration as public.
- Validate untrusted input at system boundaries.
- Keep dependencies minimal and review updates.
- Avoid logging tokens, personal data, or secret configuration.
- Do not store authentication credentials in browser local storage by default.

This starter does not provide an authentication backend or claim a complete production security review.
