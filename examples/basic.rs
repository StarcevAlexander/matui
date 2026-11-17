use matui::{
    Badge, Banner, BottomSheet, Button, ButtonVariant, Card, Carousel, Checkbox, Chip, Color,
    Dialog, Divider, Fab, IconButton, IconButtonVariant, LinearProgress, ListItem, Menu, MenuItem,
    NavItem, NavigationBar, Radio, Render, SearchBar, SegmentOption, SegmentedButtonGroup, Slider,
    Snackbar, Spinner, Switch, Text, TextField, TextFieldVariant, TextStyle, Tooltip, TopAppBar,
    css,
};

fn main() {
    let mut page = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>matui gallery</title><style>",
    );
    page.push_str(&css::stylesheet());
    page.push_str("body { margin: 0; background: var(--matui-surface); }");
    page.push_str(
        ".demo { padding: 32px 40px 120px; display: grid; gap: 20px; max-width: 760px; }",
    );
    page.push_str(".demo-row { display: flex; gap: 12px; flex-wrap: wrap; align-items: center; }");
    page.push_str("</style></head><body>");

    page.push_str(
        &TopAppBar::new("matui")
            .actions(format!(
                "{}{}",
                IconButton::new("⌕").aria("Search").render(),
                IconButton::new("⚙").aria("Settings").render()
            ))
            .render(),
    );

    page.push_str("<div class=\"demo\">");

    page.push_str(
        &Text::new("Material Design 3, fully in Rust", TextStyle::HeadlineLarge).render(),
    );
    page.push_str(
        &Text::new(
            "Every component below is a typed builder that renders framework-agnostic HTML.",
            TextStyle::BodyMedium,
        )
        .render(),
    );

    page.push_str("<p class=\"matui-overline\">Typography</p>");
    page.push_str(&Text::new("Display large", TextStyle::DisplayLarge).render());
    page.push_str(&Text::new("Headline medium", TextStyle::HeadlineMedium).render());
    page.push_str(&Text::new("Title medium", TextStyle::TitleMedium).render());
    page.push_str(
        &Text::new(
            "Body medium: the quick brown fox jumps over the lazy dog.",
            TextStyle::BodyMedium,
        )
        .render(),
    );
    page.push_str(&Text::new("Label small", TextStyle::LabelSmall).render());

    page.push_str(
        "<p class=\"matui-overline\">Buttons and segmented button</p><div class=\"demo-row\">",
    );
    page.push_str(&Button::new("Get started").render());
    page.push_str(
        &Button::new("Learn more")
            .variant(ButtonVariant::Outlined)
            .render(),
    );
    page.push_str(
        &Button::new("Add to favorites")
            .variant(ButtonVariant::Tonal)
            .color(Color::Secondary)
            .render(),
    );
    page.push_str(
        &Button::new("Delete")
            .variant(ButtonVariant::Text)
            .color(Color::Error)
            .render(),
    );
    page.push_str("</div><div class=\"demo-row\">");
    page.push_str(
        &SegmentedButtonGroup::new()
            .option(SegmentOption::new("Days"))
            .option(SegmentOption::new("Months").icon("📅"))
            .option(SegmentOption::new("Years"))
            .selected(0)
            .render(),
    );
    page.push_str("</div>");

    page.push_str(
        &Card::new()
            .title("Cards and elevation")
            .body("Cards use M3 shapes and elevation levels. Hover to see the state change.")
            .elevated(true)
            .render(),
    );

    page.push_str(
        "<p class=\"matui-overline\">Icon buttons and badges</p><div class=\"demo-row\">",
    );
    page.push_str(
        &IconButton::new("✎")
            .variant(IconButtonVariant::Filled)
            .aria("Edit")
            .render(),
    );
    page.push_str(
        &IconButton::new("♥")
            .variant(IconButtonVariant::Tonal)
            .color(Color::Error)
            .aria("Like")
            .render(),
    );
    page.push_str(
        &IconButton::new("⌕")
            .variant(IconButtonVariant::Outlined)
            .aria("Search")
            .render(),
    );
    page.push_str(&IconButton::new("✉").aria("Mail").render());
    page.push_str(&Tooltip::new(Badge::new("4").render(), "4 unread notifications").render());
    page.push_str(&Badge::dot().render());
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">Text fields</p>");
    page.push_str(
        &TextField::new("Email")
            .name("email")
            .helper("We never share it")
            .render(),
    );
    page.push_str(
        &TextField::new("Password")
            .variant(TextFieldVariant::Outlined)
            .render(),
    );
    page.push_str(
        &TextField::new("Username")
            .error("This name is taken")
            .render(),
    );

    page.push_str("<p class=\"matui-overline\">Selection controls</p><div class=\"demo-row\">");
    page.push_str(&Checkbox::new("Remember me").checked(true).render());
    page.push_str(&Checkbox::new("Subscribe").indeterminate(true).render());
    page.push_str(&Switch::new().label("Dark mode").checked(true).render());
    page.push_str("</div><div class=\"demo-row\">");
    page.push_str(
        &Radio::new("framework", "axum", "axum")
            .checked(true)
            .render(),
    );
    page.push_str(&Radio::new("framework", "actix", "actix-web").render());
    page.push_str(
        &Radio::new("framework", "rocket", "Rocket")
            .disabled(true)
            .render(),
    );
    page.push_str("</div><div class=\"demo-row\">");
    page.push_str(&Slider::new("volume").range(0.0, 100.0).value(35.0).render());
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">Chips</p><div class=\"demo-row\">");
    page.push_str(&Chip::new("Rust").icon("✓").selected(true).render());
    page.push_str(&Chip::new("Cargo").render());
    page.push_str(&Chip::new("Embedded").render());
    page.push_str(&Chip::new("Deleted").icon("✕").disabled(true).render());
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">Progress</p><div class=\"demo-row\">");
    page.push_str(&LinearProgress::new().render());
    page.push_str(&LinearProgress::new().value(0.65).render());
    page.push_str(&Spinner::new().size(28).render());
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">Banner</p>");
    page.push_str(
        &Banner::new("A new version of matui is available.")
            .icon("⬆")
            .actions(Button::new("Install").variant(ButtonVariant::Text).render())
            .render(),
    );

    page.push_str("<p class=\"matui-overline\">Search and menu</p>");
    page.push_str(&SearchBar::new("Search crates").render());
    page.push_str("<div style=\"position: relative; display: inline-block;\">");
    page.push_str(
        &Button::new("Open menu")
            .variant(ButtonVariant::Outlined)
            .render(),
    );
    page.push_str(
        &Menu::new()
            .item(MenuItem::new("Copy").icon("⧉"))
            .item(MenuItem::new("Move").icon("➜"))
            .item(MenuItem::new("Delete").icon("🗑"))
            .open(true)
            .render(),
    );
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">List</p>");
    page.push_str(
        &ListItem::new("Inbox")
            .supporting("12 unread")
            .leading("✉")
            .trailing("›")
            .render(),
    );
    page.push_str(&Divider::new().inset(true).render());
    page.push_str(
        &ListItem::new("Drafts")
            .supporting("No messages")
            .leading("📝")
            .trailing(Badge::new("2").render())
            .render(),
    );
    page.push_str(&Divider::new().render());
    page.push_str(&ListItem::new("Sent").leading("➤").render());

    page.push_str("<p class=\"matui-overline\">Carousel</p>");
    page.push_str(
        &Carousel::new()
            .item(
                Card::new()
                    .title("Card one")
                    .body("Scrolls with snap.")
                    .render(),
            )
            .item(
                Card::new()
                    .title("Card two")
                    .body("Swipe on touch devices.")
                    .render(),
            )
            .item(
                Card::new()
                    .title("Card three")
                    .body("Pure CSS scroll-snap.")
                    .render(),
            )
            .render(),
    );

    page.push_str("<p class=\"matui-overline\">FABs</p><div class=\"demo-row\">");
    page.push_str(&Fab::new("+").label("Create").render());
    page.push_str(&Fab::new("✎").color(Color::Tertiary).render());
    page.push_str(&Fab::new("🗑").color(Color::Error).disabled(true).render());
    page.push_str("</div>");

    page.push_str("<p class=\"matui-overline\">Dialog (open)</p>");
    page.push_str(
        &Dialog::new("Delete draft?", "This action cannot be undone.")
            .actions(format!(
                "{}{}",
                Button::new("Cancel").variant(ButtonVariant::Text).render(),
                Button::new("Delete")
                    .variant(ButtonVariant::Text)
                    .color(Color::Error)
                    .render()
            ))
            .open(true)
            .render(),
    );

    page.push_str("<p class=\"matui-overline\">Snackbar</p>");
    page.push_str(
        &Snackbar::new("Draft saved")
            .action("Undo")
            .open(true)
            .render(),
    );

    page.push_str("<p class=\"matui-overline\">Bottom sheet (open)</p>");
    page.push_str(
        &BottomSheet::new()
            .content(format!(
                "<div class=\"matui-title-medium\" style=\"margin-bottom: 12px;\">Filters</div>{}",
                Checkbox::new("Only stable versions").checked(true).render()
            ))
            .open(true)
            .render(),
    );

    page.push_str("</div>");

    page.push_str(
        &NavigationBar::new()
            .item(NavItem::new("🏠", "Home").active(true))
            .item(NavItem::new("🔍", "Search").badge("3"))
            .item(NavItem::new("➕", "Create"))
            .item(NavItem::new("👤", "Profile"))
            .render(),
    );

    println!("{page}");
}
