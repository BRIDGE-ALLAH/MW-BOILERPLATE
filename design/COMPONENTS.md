# Moon UI Components

## Implemented

| Component | Current API | Behavior |
|---|---|---|
| MoonButton | `MoonButton { label: String }` | Native button with primary styling and text label |
| MoonCard | `MoonCard { title: String, description: String }` | Section surface with heading and descriptive paragraph |

These APIs are minimal. No variants, sizes, event callback props, loading states, or disabled props are currently exposed.

## Deferred / planned

Dialog, popover, tooltip, toast, tabs, select, checkbox, switch, input/textarea primitives, alert, badge, spinner, separator, and visually-hidden utility are not currently exported as reusable Moon UI components. The web app's Input page currently uses a native controlled input as a demonstration.

Implement components incrementally according to [.plan/MASTER-PLAN.md](../.plan/MASTER-PLAN.md). Every component needs a documented public API, defaults, variants/sizes where relevant, disabled/loading behavior, keyboard interaction, accessible naming, focus behavior, responsive behavior, CSS ownership, usage example, and tests.

Dialog is specifically gated on focus entry/return, Escape behavior, accessible naming, and keyboard tests. Do not replace the existing honest placeholder with a non-functional visual mock.
