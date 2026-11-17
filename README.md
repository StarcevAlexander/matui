# matui

Material Design 3 UI components for Rust: typed builders that render styled, framework-agnostic HTML.

No JS runtime, no framework lock-in. Works with axum, actix-web, rocket, static site generators, or plain string output. Selection controls, sliders and text fields are real `<input>` elements and stay interactive without JavaScript.

## Usage

```rust
use matui::{Button, ButtonVariant, Card, Fab, Render, css};

fn main() {
    let card = Card::new()
        .title("Material for Rust")
        .body("Framework-agnostic HTML builders.")
        .elevated(true);

    let page = format!(
        "<html><head><style>{}</style></head><body>{}{}</body></html>",
        css::stylesheet(),
        Button::new("Get started").render(),
        card.render(),
    );
    println!("{page}");
}
```

## Components

| Category | Components |
|---|---|
| Actions | `Button`, `IconButton`, `Fab`, `SegmentedButtonGroup`, `Badge` |
| Text input | `TextField`, `SearchBar` |
| Selection | `Checkbox`, `Radio`, `Switch`, `Slider` |
| Communication | `Dialog`, `Snackbar`, `Banner`, `Tooltip`, `Menu` |
| Containment | `Card`, `Divider`, `List` (`ListItem`), `BottomSheet`, `Carousel` |
| Navigation | `TopAppBar`, `BottomAppBar`, `NavigationBar`, `NavigationRail`, `NavigationDrawer` |
| Feedback | `LinearProgress`, `Spinner` |
| Typography | `Text` with the full M3 type scale (`TextStyle`, 15 styles) |

Every component implements `Render`, returning an HTML string. All classes share the `matui-` prefix, so the bundled stylesheet never fights your own CSS. Design tokens are exposed as CSS custom properties (`--matui-primary`, `--matui-surface`, ...).

## License

Licensed under either of MIT or Apache-2.0, at your option.

Not affiliated with Google. Material Design is a trademark of Google LLC. The component set and visual language are inspired by Angular Material and Material Design 3 guidelines.
