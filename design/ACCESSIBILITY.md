# Accessibility Contract

Accessibility is a component acceptance requirement, not an optional visual polish phase.

## Baseline

- Use native semantic HTML before custom ARIA widgets.
- Every interactive control has an accessible name.
- Every form control has a programmatically associated label.
- Keyboard users can reach and operate all interactive controls.
- Focus is visible and is not obscured by sticky or overlay content.
- Disabled, invalid, selected, expanded, and loading states are exposed semantically.
- Color is never the only way to convey status or errors.
- Text and interactive states are checked for contrast in light and dark themes.
- Responsive layout preserves reading order and does not require horizontal scrolling for ordinary content.
- Honor reduced-motion preferences.

## Keyboard patterns

| Control | Required behavior |
|---|---|
| Button | Tab to focus; Enter and Space activate |
| Link | Tab to focus; Enter navigates |
| Checkbox/Switch | Tab to focus; Space toggles |
| Native Select | Preserve platform keyboard interaction |
| Tabs | Implement complete ARIA tab keyboard model before using tab roles |
| Tooltip | Trigger by focus and hover; Escape dismisses; no interactive descendants |
| Modal Dialog | Focus enters; remains appropriately contained; Escape closes when allowed; focus returns to trigger |

## Forms

- Associate labels with controls using native label/for semantics.
- Connect help and error text through described-by relationships.
- Expose invalid state with native validation/ARIA only when justified.
- Place error text near the affected field and describe how to correct it.
- Do not clear user-entered data on validation failure.

## Dynamic feedback

- Use status/live-region semantics for non-urgent updates.
- Reserve alert semantics for urgent information.
- Do not move focus to a toast or transient message.
- Avoid auto-dismiss for critical or actionable messages.

## Verification

Automated accessibility scanning supplements, but does not replace, keyboard and assistive-technology review. The browser E2E task in [.plan/TASK-INDEX.md](../.plan/TASK-INDEX.md) must cover representative routes, forms, theme, dialog focus behavior, and responsive layout. Record tool versions and unresolved violations in [.plan/PROGRESS.md](../.plan/PROGRESS.md).
