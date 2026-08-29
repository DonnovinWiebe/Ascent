use iced::Length::FillPortion;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, image, mouse_area, responsive, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_font_awesome::fa_icon_solid as icon;
use crate::bit_vault::bit_wallet::BitWallet;
use crate::container::app::{App, Pages};
use crate::container::signal::{BitWalletsPageSignal, Signal, TransactionsPageSignal};
use crate::vault_pages::filter_ui::{advance_filter_month_panel, advance_filter_year_panel, filter_mode_toggle_button, filter_tags, recede_filter_month_panel, recede_filter_year_panel, search_bar, search_terms, toggle_filter_month_panel, toggle_filter_year_panel};
use crate::error_pages::warnings_page::warning_flag_button;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, ThemeProvider, Widths, header, navigation_panel, pad, panel, panel_button, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};
use crate::vault::bank::{CurrencyExchange, Filters};
use crate::vault::parse::CashFlow;
use crate::vault::ring_parse::RingParse;
use crate::vault::transaction::{Tag, TagStyles, Transaction};
use schrod::Schrod::{self, Fail, Pass};

/// The page used to display `BitWallet`s.
#[must_use]
pub fn bit_wallets_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    let wallets: &[BitWallet] = app.get_bit_bank().get_wallets();
    
    stack![
        row![
            navigation_panel(app, Pages::page_pointers(app)),
            stack![
                container(wallet_list(app, wallets)).center_x(Fill),
            ].width(FillPortion(4)),
        ],
        header(app, vec![warning_flag_button(app), add_wallet_button(app)]),
    ]
}



// components
/// A displayed list of `BitWallet`s.
#[must_use]
fn wallet_list<'a>(
    app: &'a App,
    wallets: &[BitWallet],
)  -> Element<'a, Signal> {
    let mut first_half = Vec::new();
    let mut second_half = Vec::new();
    for (i, wallet) in wallets.iter().enumerate() {
        if i % 2 == 0 { first_half.push(wallet); }
        else { second_half.push(wallet); }
    }
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),

            row![
                column(first_half.into_iter().map(|wallet| { wallet_panel(app, wallet) }))
                .spacing(Spacing::Micro.size()),

                column(second_half.into_iter().map(|wallet| { wallet_panel(app, wallet) }))
                .spacing(Spacing::Micro.size()),
            ]
            .spacing(Spacing::Small.size()),
            
            spacer(Orientations::Vertical, Spacing::ManagementPanelSpace),
        ]
        .spacing(Spacing::None.size())
    )
    .direction(Direction::Vertical(Scrollbar::hidden()))
    .width(Widths::SmallCard.size() * 2.0 + Spacing::Small.size() * 3.0)
    .height(Fill)
    .into()
}

/// A panel that displays an individual `BitWallet`.
#[must_use]
fn wallet_panel<'a>(
    app: &'a App,
    wallet: &BitWallet,
) -> Element<'a, Signal> {
    let name = wallet.get_name();
    let coin = wallet.get_coin_string();
    let color = wallet.get_color();

    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color,
            depth: Depths::Proud,
        },
        PanelSize {
            width: Widths::SmallCard,
            height: Heights::Shrink
        },
        PaddingSizes::Medium, {
        column![
            row![
                ui_string(app, name, TextSizes::SmallHeading, MaterialColors::StrongText),
                spacer(Orientations::Horizontal, Spacing::Fill),
            ]
            .spacing(0)
            .align_y(Center),
            
            row![
                ui_string(app, coin, TextSizes::SmallHeading, MaterialColors::StrongText),
                spacer(Orientations::Horizontal, Spacing::Fill),
                open_wallet_button(app, wallet),
                edit_wallet_button(app, wallet),
            ]
            .spacing(Spacing::Small.size())
            .align_y(Center),
        ]
        .spacing(0)
        .into()
    })
}

/// A button that allows the user to open a `BitWallet`.
#[must_use]
fn open_wallet_button<'a>(
    app: &'a App,
    wallet: &BitWallet,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        icon("wallet"),
        Signal::BitWalletsPageSignal(BitWalletsPageSignal::OpenBitWallet(wallet.get_id())),
        true,
    )
}

/// A button that allows the user to edit a `BitWallet`.
#[must_use]
fn edit_wallet_button<'a>(
    app: &'a App,
    wallet: &BitWallet,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Bloated,
        icon("pencil"),
        Signal::BitWalletsPageSignal(BitWalletsPageSignal::StartEditingBitWallet(wallet.get_id())),
        true,
    )
}

/// A panel that displays a `Coin`.
#[must_use]
pub fn coin_panel<'a>(
    app: &'a App,
    wallet: &BitWallet,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::Shrink, height: Heights::Shrink },
        PaddingSizes::Small, {
            ui_string(app, wallet.get_coin().to_string(), TextSizes::Interactable, MaterialColors::StrongText)
        }
    )
}

/// Allows a user to start ading a `BitWallet`.
#[must_use]
fn add_wallet_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        icon("plus"),
        Signal::BitWalletsPageSignal(BitWalletsPageSignal::StartAddingBitWallet),
        true,
    )
}