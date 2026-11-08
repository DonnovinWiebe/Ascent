use iced::{Element, widget::row, Center, Fill};
use materialui::{components::{ButtonShapes, Spacing, TextSizes, panel_button, ui_string}, materials::{Depths, MaterialColors, MaterialStyle, Materials}};
use crate::application::{app::{App, Pages}, signal::{GeneralSignal, Signal}, state::ConnectionState};
use iced_font_awesome::fa_icon_solid as icon;

/// Holds the notifications for the `App`'s `connection_state`..
#[must_use]
pub fn server_notification<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    match app.get_app_state().connection_state() {
        ConnectionState::Error => { server_connection_error_notification(app) },
        
        ConnectionState::Unauthorized => { server_unauthorized_notification(app) },
        
        ConnectionState::OutOfSync => { server_out_of_sync_notification(app) },
        
        _ => {
            row!()
                .align_y(Center)
                .spacing(Spacing::Small.size())
                .into()
        },
    }
}

/// A clickable notification that there is a server connection issue.
#[must_use]
fn server_connection_error_notification<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::warning(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        row!(
            icon("plug-circle-xmark"),
            ui_string(app, "Can't reach server!", TextSizes::Interactable, MaterialColors::StrongText),
        )
        .align_y(Center)
        .spacing(Spacing::Small.size()),
        Signal::GeneralSignal(GeneralSignal::ChangePageTo(Pages::Settings)),
        true,
    )
}

/// A clickable notification that the client is not authorized to communicate with the server.
#[must_use]
fn server_unauthorized_notification<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::danger(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        row!(
            icon("ban"),
            ui_string(app, "Unauthorized!", TextSizes::Interactable, MaterialColors::StrongText),
        )
        .align_y(Center)
        .spacing(Spacing::Small.size()),
        Signal::GeneralSignal(GeneralSignal::ChangePageTo(Pages::Settings)),
        true,
    )
}

/// A clickable notification that the client and server are out of sync.
#[must_use]
fn server_out_of_sync_notification<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::warning(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        row!(
            icon("code-branch"),
            ui_string(app, "Out of sync!", TextSizes::Interactable, MaterialColors::StrongText),
        )
        .align_y(Center)
        .spacing(Spacing::Small.size()),
        Signal::GeneralSignal(GeneralSignal::ChangePageTo(Pages::Settings)),
        true,
    )
}