# Security Model

## Current trust boundary

This repository is a static frontend foundation, not an authentication provider, backend, or secret store.

- Browser and WASM configuration is public. Never compile secrets or private API keys into it.
- No authentication, session, authorization, or credential storage is implemented.
- Validate untrusted data at boundaries before use.
- Keep diagnostic logs free of credentials and personal data.
- Review dependency advisories before releases.
- Avoid adding browser permissions or persistent credential storage without a documented threat model.

## Current deployment security

The current Docker deployment serves static files through Nginx. The repository's Dockerfile does not currently include a custom Nginx security-header configuration or a backend security boundary. Do not claim CSP, HSTS, CSRF, rate limiting, or authentication protections are configured by this frontend.

Before public production use, evaluate and configure appropriate headers at the trusted Nginx/Coolify proxy layer, including a CSP compatible with the generated Dioxus assets, frame restrictions, MIME sniffing protection, and referrer policy. Enable HSTS only after HTTPS is correctly enforced for the deployment domain. Test policy compatibility in a browser before enforcement.

## Future backend integration

A consuming backend must independently define:
- Authentication/session lifecycle and authorization.
- CSRF protections where cookie-based sessions require them.
- Rate limiting and abuse controls.
- Input validation and safe error responses.
- TLS termination and security headers.
- Secret storage and rotation.
- Audit/logging policy and retention.

Tokens must not be casually exposed to JavaScript or browser local storage. A future API client must not invent authentication behavior; it must consume a reviewed contract owned by the product/backend.

## Verification

Run dependency audit and inspect generated web assets for accidental secret exposure. Review workflow permissions and untrusted pull-request boundaries. Docker/runtime and browser security checks are required before production acceptance; see [.plan/MASTER-PLAN.md](../.plan/MASTER-PLAN.md).
