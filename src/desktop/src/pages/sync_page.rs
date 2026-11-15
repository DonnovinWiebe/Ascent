use iced_font_awesome::fa_icon_solid as icon;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::application::app::{App, Pages};
use crate::application::signal::{GeneralSignal, SaveDataSignal, SettingsSignal, Signal};
use crate::pages::settings_page::setting_heading;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, ThemeProvider, Widths, header, navigation_panel, panel, panel_button, panel_text_input, spacer, ui_string};
use materialui::materials::{MaterialThemes, Depths, MaterialColors, MaterialStyle, Materials};
use nexus::standard::bank::{CurrencyExchange, ExchangeRate, ExchangeRateStatus};
use nexus::standard::parse::FlowTypes;
use nexus::standard::transaction::Transaction;

/// The page used to display sync related settings for the `App`.
#[must_use]
pub fn sync_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app, Pages::page_pointers(app)),
            container(parameters_list(app)).center_x(Fill),
        ],
    ]
}

/// The list of sync parameters.
#[must_use]
fn parameters_list<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),

            info(app),
            toggle_sync_button(app),
            
            spacer(Orientations::Vertical, Spacing::Small),
            setting_heading(app, "Server Address".to_string()),
            update_address_field(app, app.get_app_state().server_address()),
            
            spacer(Orientations::Vertical, Spacing::Small),
            setting_heading(app, "Access Token".to_string()),
            update_token_field(app, app.get_app_state().new_token_string()),
            
            // trailing space
            spacer(Orientations::Vertical, Spacing::Large),
        ]
        .spacing(Spacing::Medium.size())
    )
    .direction(Direction::Vertical(Scrollbar::hidden()))
    .width(Widths::LargeCard.size())
    .height(Fill)
    .into()
}

/// Displays some information about the sync feature.
#[must_use]
fn info<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    ui_string(
        app,

        "The sync feature is entirely optional and only works with self hosted servers. If this is turned off, no network requests of any kind are made and no data ever leaves your device. If this is turned on, data only goes to your own server at the address that you set.\n\nIMPORTANT NOTE\nThe data itself being sent to and from the server is unencrypted (including your access token!), so only use safe connections. It is recommended to only connect over our home network or use Wireguard or TailScale for long distance connections.",
        
        TextSizes::Interactable,
        MaterialColors::StrongText
    )
}

/// Travels back to the settings page.
#[must_use]
pub fn go_back_to_settings_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Standard,
        row![
            icon("chevron-left"),
            ui_string(app, "Go Back", TextSizes::Interactable, MaterialColors::StrongText),
        ]
        .align_y(Center)
        .spacing(Spacing::Small.size()),
        Signal::GeneralSignal(GeneralSignal::ChangePageTo(Pages::Settings)),
        true,
    ).into()
}

/// The developer mode toggle button.
#[must_use]
fn toggle_sync_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    let icon = if app.get_app_state().sync() { icon("toggle-on") } else { icon("toggle-off") };
    let color = if app.get_app_state().sync() { MaterialColors::accent(app.material_theme()) } else { MaterialColors::Card };
    
    row![
        ui_string(app, "Enable sync", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon,
            Signal::SettingsSignal(SettingsSignal::ToggleSync),
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// Updates the sync server address.
#[must_use]
fn update_address_field<'a>(
    app: &'a App,
    address: String,
) -> Element<'a, Signal> {
    let on_change = |new_address: String| Signal::SettingsSignal(SettingsSignal::UpdateSyncServerAddress(new_address));
    
    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        Widths::MediumField,
        "Only use safe connections!",
        &address,
        on_change,
        None,
        true,
    )
}

/// Updates the sync access token.
#[must_use]
fn update_token_field<'a>(
    app: &'a App,
    new_token_string: String,
) -> Element<'a, Signal> {
    let on_change = |new_token_string: String| Signal::SettingsSignal(SettingsSignal::UpdateNewTokenString(new_token_string));
    let on_submit_option = Some(Signal::SettingsSignal(SettingsSignal::UpdateToken));
    
    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        Widths::MediumField,
        "New Token",
        &new_token_string,
        on_change,
        on_submit_option,
        true,
    )
}
