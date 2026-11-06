# Contracts

## Core

- Identifier: non-empty string wrapper.
- CoreError: foundational identifier validation error.

## API DTOs

- PageRequest: page and per_page; defaults to page 1 and 20 items.
- Page<T>: generic paginated response.
- ApiError: stable code, message, optional request_id.

## UI

- MoonButton: label string.
- MoonCard: title and description strings.

Serialized field names are public API. Treat changes to names or meanings as contract changes and document breaking changes.
