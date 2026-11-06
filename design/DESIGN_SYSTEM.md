# Moonbase Design System

## Purpose

A neutral, product-independent visual foundation. Consuming products may layer their own brand identity without forking component behavior or changing shared contracts.

## Existing implemented tokens

The current stylesheet defines these CSS custom properties:

| Token | Purpose |
|---|---|
| `--bg` | Application background |
| `--surface` | Primary surface |
| `--surface-muted` | Muted surface |
| `--text` | Primary text |
| `--muted` | Secondary text |
| `--border` | Default border |
| `--accent` | Interactive accent |
| `--accent-contrast` | Text/icon color on accent |
| `--radius` | Default surface radius |
| `--shadow` | Default surface elevation |

Light theme is the `:root` default. Dark theme is selected by `:root[data-theme="dark"]`. The theme toggle currently sets the document root attribute in the web app.

## Token categories and target naming

Extend only when a real component consumes the token.

| Category | Suggested semantic tokens | Status |
|---|---|---|
| Color | `--bg`, `--surface`, `--surface-muted`, `--text`, `--muted`, `--border`, `--accent`, `--accent-contrast`, `--danger`, `--success`, `--warning`, `--info` | Base subset implemented; status colors deferred |
| Typography | `--font-sans`, `--font-mono`, `--text-xs` through `--text-2xl`, `--line-height` | Partially implemented via root font stack and element styles |
| Spacing | `--space-1` through `--space-8` | Not yet tokenized |
| Radius | `--radius-sm`, `--radius-md`, `--radius-lg`, `--radius-pill` | Single `--radius` exists |
| Border | `--border-width`, `--border-style` | Browser defaults + color token |
| Shadow | `--shadow-sm`, `--shadow-md`, `--shadow-lg` | Single `--shadow` exists |
| Motion | `--duration-fast`, `--duration-normal`, `--ease-standard` | Reduced-motion rule exists; semantic timing tokens deferred |
| Z-index | `--z-dropdown`, `--z-overlay`, `--z-toast` | Not yet required |
| Breakpoints | CSS media-query conventions | 760px responsive breakpoint currently exists in app CSS |

The names above are a design contract proposal, not all existing CSS variables. Do not claim a token is implemented until it exists in the stylesheet and has a consumer.

## Theme contract

- Theme is selected through the document root `data-theme` attribute.
- CSS custom properties are the runtime source of truth.
- Theme preference persistence is not currently implemented.
- Components must not hardcode brand-specific colors.
- Every semantic token used for text or controls must be checked for contrast in both themes.

## Responsive and motion behavior

- Prefer fluid widths and CSS grid/flex layouts.
- Preserve logical DOM order at every viewport.
- Test navigation and form controls on narrow viewports.
- Honor `prefers-reduced-motion: reduce`; do not rely on animation to communicate state.

## Ownership

App-wide tokens live in the web stylesheet until a second consumer justifies extraction into a dedicated theme/token crate. Shared components consume semantic variables and do not own product-specific palettes.
