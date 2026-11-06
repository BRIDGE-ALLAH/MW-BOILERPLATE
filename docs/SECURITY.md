# Security Model

This repository is a frontend foundation, not an authentication or backend implementation.

- Browser and WASM configuration is public; never compile secrets into it.
- Validate data at every trust boundary.
- Avoid persisting authentication credentials in local storage by default.
- Keep logs free of credentials and personal data.
- Review dependency advisories before releases.

A production backend must independently define session management, authorization, CSRF protections where applicable, rate limiting, and security headers.
