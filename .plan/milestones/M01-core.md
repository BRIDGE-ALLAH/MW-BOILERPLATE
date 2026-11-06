# M01 — Core Contracts and Integration

**Priority:** P1  
**Objective:** Add only the missing integration foundations needed by a reusable frontend, without inventing a backend.

## Tasks

- M01-001 public client configuration boundary.
- M01-002 typed API client.
- M01-003 typed error mapping and recovery.

## Boundaries

Contracts remain transport-independent. The client owns HTTP serialization, timeout, response parsing, and typed transport failures. Browser-visible configuration is public; secrets remain server-side. No endpoint or auth flow is created without a consumer-owned contract.

## Exit criteria

Unit tests cover valid/invalid config, serialization, HTTP success/error/timeout/decode mapping, and safe error display. Native and WASM dependency graphs are checked.
