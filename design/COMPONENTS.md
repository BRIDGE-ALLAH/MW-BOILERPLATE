# Moon UI Component Contracts

**Status:** The first two components are implemented. The remaining APIs below are target contracts, not claims of shipped functionality. Exact Dioxus prop/event syntax must be verified against the pinned stable release during implementation.

## API conventions

- Component names use `Moon` prefix in Rust exports; CSS classes use `moon-` prefix.
- Prefer native HTML semantics and controlled state. A component emits events; the consuming page owns application state.
- Props should be explicit and typed. Avoid stringly typed variants when a Rust enum is practical.
- Defaults must be documented and stable. Avoid hidden global state.
- Shared UI must not call APIs, read product data, or own domain validation.
- CSS is owned by `crates/ui` and consumes semantic variables from the app's theme layer.
- Every interactive component documents accessible name, keyboard behavior, focus behavior, disabled state, and loading state if applicable.

## Implemented API

| Component | Current API | Current behavior |
|---|---|---|
| MoonButton | `label: String` | Native button with primary styling and text label |
| MoonCard | `title: String, description: String` | Section surface with heading and descriptive paragraph |

The current Button API does not yet expose variants, sizes, disabled/loading state, type, or callback props. The current Card API is a fixed title/description surface.

## Target component contracts

### Primitives

| Component | Target props | Defaults / behavior | Accessibility and keyboard |
|---|---|---|---|
| Button | `variant`, `size`, `disabled`, `loading`, `type`, `aria_label`, `onclick`, children | variant primary; size medium; type button; loading disables repeated activation | Native button; Enter/Space activate; visible focus; loading state announced without removing accessible name |
| Separator | `orientation`, `decorative` | horizontal, decorative | Use separator role/orientation only when semantic; decorative separators are hidden from accessibility tree |
| Spinner | `size`, `label` | medium; requires meaningful label when conveying progress | Expose status role and accessible name; decorative spinner is hidden from assistive tech |
| VisuallyHidden | children | visually hidden, remains in accessibility tree | Must not use display:none/visibility:hidden; no focusable content unless intended |

### Form

| Component | Target props | Defaults / behavior | Accessibility and keyboard |
|---|---|---|---|
| Label | `html_for`, children | explicit association | Native label association |
| Input | `value`, `type`, `disabled`, `required`, `invalid`, `help_text`, `error_text`, `oninput`, `onchange` | text input; controlled when value supplied | Label association; invalid state and described-by references; native keyboard editing |
| Textarea | `value`, `rows`, `disabled`, `required`, `invalid`, `help_text`, `error_text`, `oninput` | rows 3; controlled value | Label association; error/help descriptions; native keyboard editing |
| Checkbox | `checked`, `disabled`, `required`, `onchange` | unchecked | Native checkbox semantics; Space toggles |
| Switch | `checked`, `disabled`, `label`, `onchange` | off | role=switch only if native checkbox presentation is insufficient; Space toggles; state exposed |
| Select | `value`, `options`, `disabled`, `required`, `onchange` | native select preferred | Native select keyboard behavior and label association |

Validation messages are rendered by the consuming form or shared field wrapper; do not silently validate product-specific rules inside generic controls.

### Feedback

| Component | Target props | Defaults / behavior | Accessibility and keyboard |
|---|---|---|---|
| Alert | `variant`, `title`, children | neutral; persistent content | role=status for non-urgent updates; role=alert only for urgent messages |
| Badge | `variant`, children | neutral | Status must remain understandable without color |
| Tooltip | `content`, `delay`, trigger | opt-in, short descriptive content | Trigger remains keyboard reachable; Escape dismisses; tooltip does not contain interactive content |
| Dialog | `open`, `title`, `description`, `modal`, `onclose`, children | closed; modal only when explicitly required | Accessible name; focus entry/containment/return; Escape behavior; background interaction controlled when modal |
| Toast | `variant`, `title`, `description`, `duration`, `onclose` | short-lived only when safe | Announce appropriately; do not steal focus; pause/dismiss controls for actionable content; avoid disappearing critical errors |

### Layout

| Component | Target props | Defaults / behavior | Accessibility and keyboard |
|---|---|---|---|
| Card | `title`, `description`, children | neutral surface | Use section/article semantics only when content structure warrants it |
| Container | `size`, children | responsive max width | No semantic role by default |
| Stack | `direction`, `gap`, children | vertical | Layout-only wrapper |
| Grid | `columns`, `gap`, children | responsive columns | Preserve logical DOM reading order |
| Tabs | `value`, `items`, `onchange` | first tab only when a valid default exists | Full WAI-ARIA tab keyboard model; otherwise use ordinary links/buttons |

## Component implementation gates

1. Read the matching task in [.plan/TASK-INDEX.md](../.plan/TASK-INDEX.md).
2. Verify current Dioxus 0.7 API and native HTML semantics before writing RSX.
3. Add semantic-token CSS and responsive states.
4. Test accessible name, focus, keyboard, disabled, and error/loading behavior.
5. Add a real showcase usage; do not leave dead examples.
6. Update this inventory only after the exported component exists and compiles.

## Deferred

Popover, complex date/time pickers, command palettes, rich text, data tables, and product-specific form workflows are deliberately outside the initial component baseline. Add them only with concrete consumer demand and a self-contained task.
