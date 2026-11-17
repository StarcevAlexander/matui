use matui::{
    Badge, Banner, BottomAppBar, BottomSheet, Button, ButtonVariant, Card, Carousel, Checkbox,
    Chip, Color, Dialog, Divider, Fab, IconButton, IconButtonVariant, LinearProgress, ListItem,
    Menu, MenuItem, NavItem, NavigationBar, NavigationDrawer, NavigationRail, Radio, Render,
    SearchBar, SegmentOption, SegmentedButtonGroup, Slider, Snackbar, Spinner, Switch, Text,
    TextField, TextStyle, Tooltip, TopAppBar, css,
};

#[test]
fn button_renders_label_and_classes() {
    let html = Button::new("Save").variant(ButtonVariant::Tonal).render();
    assert!(html.contains("matui-btn--tonal"));
    assert!(html.contains(">Save</button>"));
}

#[test]
fn disabled_button_carries_attribute_and_class() {
    let html = Button::new("Save").disabled(true).render();
    assert!(html.contains(" disabled"));
    assert!(html.contains("matui-btn--disabled"));
}

#[test]
fn linked_button_renders_anchor() {
    let html = Button::new("Docs").link("https://example.com").render();
    assert!(html.starts_with("<a "));
    assert!(html.contains("href=\"https://example.com\""));
}

#[test]
fn card_renders_title_and_elevation() {
    let html = Card::new()
        .title("Hello")
        .body("World")
        .elevated(true)
        .render();
    assert!(html.contains("matui-card--elevated"));
    assert!(html.contains("matui-card__title\">Hello</h3>"));
}

#[test]
fn fab_renders_icon_and_label() {
    let html = Fab::new("+")
        .label("Create")
        .color(Color::Tertiary)
        .render();
    assert!(html.contains("matui-fab--tertiary"));
    assert!(html.contains("aria-label=\"+\""));
}

#[test]
fn icon_button_renders_variant_and_aria() {
    let html = IconButton::new("✎")
        .variant(IconButtonVariant::Filled)
        .color(Color::Tertiary)
        .aria("Edit")
        .render();
    assert!(html.contains("matui-icon-btn--filled"));
    assert!(html.contains("matui-icon-btn--tertiary"));
    assert!(html.contains("aria-label=\"Edit\""));
}

#[test]
fn text_field_renders_floating_label_and_error() {
    let html = TextField::new("Email")
        .name("email")
        .helper("We never share it")
        .render();
    assert!(html.contains("matui-textfield--filled"));
    assert!(html.contains(">Email</label>"));
    assert!(html.contains("matui-textfield__helper"));

    let errored = TextField::new("Email").error("Invalid address").render();
    assert!(errored.contains("matui-textfield--error"));
}

#[test]
fn checkbox_state_attributes() {
    let on = Checkbox::new("Remember me").checked(true).render();
    assert!(on.contains(" checked"));
    let off = Checkbox::new("Remember me").render();
    assert!(!off.contains(" checked"));
}

#[test]
fn switch_renders_track_and_thumb() {
    let html = Switch::new().label("Dark mode").checked(true).render();
    assert!(html.contains("matui-switch__track"));
    assert!(html.contains("matui-switch__thumb"));
}

#[test]
fn chip_selected_state() {
    let plain = Chip::new("Rust").render();
    assert!(!plain.contains("matui-chip--selected"));
    let selected = Chip::new("Rust").icon("✓").selected(true).render();
    assert!(selected.contains("matui-chip--selected"));
}

#[test]
fn divider_classes() {
    assert!(
        Divider::new()
            .inset(true)
            .render()
            .contains("matui-divider--inset")
    );
    assert!(
        Divider::new()
            .vertical(true)
            .render()
            .contains("matui-divider--vertical")
    );
}

#[test]
fn progress_indeterminate_and_determinate() {
    let indeterminate = LinearProgress::new().render();
    assert!(!indeterminate.contains("matui-progress--determinate"));
    let determinate = LinearProgress::new().value(0.65).render();
    assert!(determinate.contains("matui-progress--determinate"));
    assert!(determinate.contains("aria-valuenow=\"65\""));
}

#[test]
fn spinner_renders_with_size() {
    let html = Spinner::new().size(24).render();
    assert!(html.contains("width: 24px"));
}

#[test]
fn list_item_renders_all_slots() {
    let html = ListItem::new("Inbox")
        .supporting("12 unread")
        .leading("✉")
        .trailing("12")
        .render();
    assert!(html.contains("matui-list-item__headline\">Inbox</div>"));
    assert!(html.contains("matui-list-item__trailing\">12"));
}

#[test]
fn badge_text_and_dot() {
    assert!(Badge::new("4").render().contains(">4</span>"));
    assert!(Badge::dot().render().contains("matui-badge--dot"));
}

#[test]
fn top_app_bar_with_actions() {
    let html = TopAppBar::new("Inbox")
        .actions(IconButton::new("⌕").render())
        .render();
    assert!(html.contains("matui-topbar__title\">Inbox</h1>"));
    assert!(html.contains("matui-topbar__actions"));
}

#[test]
fn dialog_open_and_closed() {
    let closed = Dialog::new("Title", "Body").render();
    assert!(!closed.contains("matui-dialog-scrim--open"));
    let open = Dialog::new("Title", "Body").open(true).render();
    assert!(open.contains("matui-dialog-scrim--open"));
}

#[test]
fn stylesheet_contains_all_component_rules() {
    let sheet = css::stylesheet();
    for class in [
        ".matui-btn",
        ".matui-segmented",
        ".matui-icon-btn",
        ".matui-card",
        ".matui-fab",
        ".matui-topbar",
        ".matui-bottom-appbar",
        ".matui-textfield",
        ".matui-checkbox",
        ".matui-radio",
        ".matui-switch",
        ".matui-slider",
        ".matui-chip",
        ".matui-divider",
        ".matui-progress",
        ".matui-spinner",
        ".matui-list-item",
        ".matui-badge",
        ".matui-dialog",
        ".matui-snackbar",
        ".matui-bottom-sheet",
        ".matui-navbar",
        ".matui-rail",
        ".matui-drawer",
        ".matui-search",
        ".matui-tooltip",
        ".matui-menu",
        ".matui-banner",
        ".matui-carousel",
        ".matui-display-large",
        ".matui-label-small",
        ".matui-elevation-3",
    ] {
        assert!(sheet.contains(class), "stylesheet missing {class}");
    }
}

#[test]
fn text_renders_typography_class() {
    let html = Text::new("Hello", TextStyle::HeadlineMedium).render();
    assert!(html.contains("matui-headline-medium"));
    assert!(html.contains(">Hello</span>"));
}

#[test]
fn segmented_group_marks_selection() {
    let html = SegmentedButtonGroup::new()
        .option(SegmentOption::new("Days"))
        .option(SegmentOption::new("Months").icon("📅"))
        .option(SegmentOption::new("Years"))
        .selected(1)
        .render();
    assert!(html.contains("matui-segmented__btn--selected"));
    assert!(html.contains("aria-pressed=\"true\""));
    assert_eq!(html.matches("aria-pressed").count(), 3);
}

#[test]
fn radio_renders_group_name_and_checked() {
    let html = Radio::new("size", "m", "Medium").checked(true).render();
    assert!(html.contains("name=\"size\""));
    assert!(html.contains("value=\"m\""));
    assert!(html.contains(" checked"));
    assert!(html.contains("matui-radio__circle"));
}

#[test]
fn slider_renders_range_and_fill() {
    let html = Slider::new("volume").range(0.0, 10.0).value(3.0).render();
    assert!(html.contains("type=\"range\""));
    assert!(html.contains("min=\"0\" max=\"10\""));
    assert!(html.contains("--matui-slider-value: 30%"));
}

#[test]
fn snackbar_visibility_and_action() {
    let hidden = Snackbar::new("Saved").action("Undo").render();
    assert!(hidden.contains("matui-snackbar--hidden"));
    assert!(hidden.contains(">Undo</button>"));
    let shown = Snackbar::new("Saved").open(true).render();
    assert!(!shown.contains("matui-snackbar--hidden"));
}

#[test]
fn bottom_sheet_open_state() {
    let open = BottomSheet::new().content("Filters").open(true).render();
    assert!(!open.contains("matui-bottom-sheet--hidden"));
    assert!(open.contains("matui-bottom-sheet__handle"));
    let closed = BottomSheet::new().content("Filters").render();
    assert!(closed.contains("matui-bottom-sheet--hidden"));
}

#[test]
fn search_bar_renders_input() {
    let html = SearchBar::new("Search crates").render();
    assert!(html.contains("matui-search"));
    assert!(html.contains("placeholder=\"Search crates\""));
    assert!(html.contains("type=\"search\""));
}

#[test]
fn tooltip_renders_bubble() {
    let html = Tooltip::new("Save", "Ctrl+S").render();
    assert!(html.contains("matui-tooltip__bubble"));
    assert!(html.contains(">Save<"));
    assert!(html.contains(">Ctrl+S</span>"));
}

#[test]
fn menu_open_and_items() {
    let open = Menu::new()
        .item(MenuItem::new("Copy").icon("⧉"))
        .item(MenuItem::new("Paste"))
        .open(true)
        .render();
    assert!(open.contains("matui-menu__item"));
    assert!(!open.contains("matui-menu--hidden"));
    let closed = Menu::new().item(MenuItem::new("Copy")).render();
    assert!(closed.contains("matui-menu--hidden"));
}

#[test]
fn navigation_bar_marks_active_item() {
    let html = NavigationBar::new()
        .item(NavItem::new("🏠", "Home").active(true))
        .item(NavItem::new("🔍", "Search").badge("3"))
        .render();
    assert!(html.contains("matui-navbar__item--active"));
    assert!(html.contains("matui-badge\">3</sup>"));
}

#[test]
fn navigation_rail_renders_header_and_items() {
    let html = NavigationRail::new()
        .header("M")
        .item(NavItem::new("✉", "Mail").active(true))
        .render();
    assert!(html.contains("matui-rail__header\">M</span>"));
    assert!(html.contains("matui-rail__item--active"));
}

#[test]
fn navigation_drawer_open_and_closed() {
    let open = NavigationDrawer::new()
        .header("matui")
        .item(NavItem::new("🏠", "Home").active(true))
        .render();
    assert!(open.contains("matui-drawer__header"));
    assert!(open.contains("matui-drawer__item--active"));
    assert!(!open.contains("matui-drawer--closed"));
    let closed = NavigationDrawer::new().open(false).render();
    assert!(closed.contains("matui-drawer--closed"));
}

#[test]
fn bottom_app_bar_renders_sections() {
    let html = BottomAppBar::new()
        .leading(IconButton::new("☰").render())
        .trailing(IconButton::new("⚙").render())
        .render();
    assert!(html.contains("matui-bottom-appbar__section"));
}

#[test]
fn banner_renders_icon_and_actions() {
    let html = Banner::new("Update available")
        .icon("⬆")
        .actions(Button::new("Install").variant(ButtonVariant::Text).render())
        .render();
    assert!(html.contains("matui-banner__icon"));
    assert!(html.contains("matui-banner__actions"));
    assert!(html.contains("Update available"));
}

#[test]
fn carousel_wraps_items() {
    let html = Carousel::new()
        .item(Card::new().title("One").render())
        .item(Card::new().title("Two").render())
        .render();
    assert_eq!(html.matches("matui-carousel__item").count(), 2);
}
