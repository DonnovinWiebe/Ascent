use std::iter;
use iced::widget::text_editor::Action;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::{row, column};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::Alignment;
use iced_font_awesome::fa_icon_solid as icon;
use crate::s31_bit_vault::bit_wallet::BitWallet;
use crate::s11_container::app::{App, Pages};
use crate::s11_container::signal::{AddBitWalletSignal, EditBitWalletSignal, EditTransactionSignal, GeneralSignal, KeybindSignal, Signal};
use materialui::components::{ButtonShapes, DatePickerModes, Directions, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, TransactionManagementTypes, Widths, header, panel, panel_button, panel_text_editor, panel_text_input, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};

/// The page used for adding `BitWallet`s.
#[must_use]
pub fn add_bit_wallet_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(bit_wallet_management_panel(app, TransactionManagementTypes::Adding)).center(Fill),
        header(app, Vec::new()),
    ]
}

/// The page used for editing `BitWallet`s.
#[must_use]
pub fn edit_bit_wallet_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(bit_wallet_management_panel(app, TransactionManagementTypes::Editing)).center(Fill),
        header(app, Vec::new()),
    ]
}



// components
/// A panel used to edit a `BitWallet`.
#[must_use]
fn bit_wallet_management_panel<'a>(
    app: &'a App,
    management_type: TransactionManagementTypes
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::LargeCard, height: Heights::Shrink },
        PaddingSizes::Small, {
            column![
                // title
                row![
                    ui_string(app, match management_type { TransactionManagementTypes::Adding => { "Adding Bit Wallet" } TransactionManagementTypes::Editing => { "Editing Bit Wallet" } }, TextSizes::LargeHeading, MaterialColors::StrongText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center),
    
    
    
                // name
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, "Name", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
                row![
                    name_field(app, management_type),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
                
                // coin
                spacer(Orientations::Vertical, Spacing::Large),
                row![
                    spacer(Orientations::Horizontal, Spacing::Small),
                    ui_string(app, "Coin", TextSizes::Body, MaterialColors::WeakText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),
                row![
                    coin_field(app, management_type),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center)
                .spacing(Spacing::None.size()),

                
    
                // buttons
                spacer(Orientations::Vertical, Spacing::Large),
                match management_type {
                    TransactionManagementTypes::Adding => {
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            save_button(app, management_type),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            cancel_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .align_y(Center)
                        .spacing(Spacing::None.size())
                    }
    
                    TransactionManagementTypes::Editing => {
                        row![
                            spacer(Orientations::Horizontal, Spacing::Fill),
                            save_button(app, management_type),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            delete_button(app),
                            spacer(Orientations::Horizontal, Spacing::Large),
                            cancel_button(app),
                            spacer(Orientations::Horizontal, Spacing::Fill),
                        ]
                        .align_y(Center)
                        .spacing(Spacing::None.size())
                    }
                },
            ]
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// A widget used to enter a `name`.
#[must_use]
fn name_field<'a>(
    app: &'a App,
    management_type: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let name_string = match management_type {
        TransactionManagementTypes::Adding => { app.get_new_bit_wallet_state().name_string() }
        TransactionManagementTypes::Editing => { app.get_edit_bit_wallet_state().name_string() }
    };
    let signal = move |str| match management_type {
        TransactionManagementTypes::Adding => { Signal::AddBitWalletSignal(AddBitWalletSignal::UpdateNewBitWalletNameString(str)) }
        TransactionManagementTypes::Editing => { Signal::EditBitWalletSignal(EditBitWalletSignal::UpdateEditBitWalletNameString(str)) }
    };
    let is_valid = BitWallet::is_name_valid(name_string);

    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::CardContent } else { MaterialColors::danger() },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "Name",
        name_string,
        signal,
        None,
        false,
    )
}

/// A widget used to enter a `Coin`.
#[must_use]
fn coin_field<'a>(
    app: &'a App,
    management_type: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let coin_string = match management_type {
        TransactionManagementTypes::Adding => { app.get_new_bit_wallet_state().coin_string() }
        TransactionManagementTypes::Editing => { app.get_edit_bit_wallet_state().coin_string() }
    };
    let signal = move |str| match management_type {
        TransactionManagementTypes::Adding => { Signal::AddBitWalletSignal(AddBitWalletSignal::UpdateNewBitWalletCoinString(str)) }
        TransactionManagementTypes::Editing => { Signal::EditBitWalletSignal(EditBitWalletSignal::UpdateEditBitWalletCoinString(str)) }
    };
    let is_valid = BitWallet::can_parse_as_coin(coin_string);

    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if is_valid { MaterialColors::CardContent } else { MaterialColors::danger() },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "Coin",
        coin_string,
        signal,
        None,
        false,
    )
}

/// Saves the `BitWallet`.
#[must_use]
fn save_button<'a>(
    app: &'a App,
    management_type: TransactionManagementTypes,
) -> Element<'a, Signal> {
    let signal = match management_type {
        TransactionManagementTypes::Adding => { Signal::AddBitWalletSignal(AddBitWalletSignal::AddBitWallet) }
        TransactionManagementTypes::Editing => { Signal::EditBitWalletSignal(EditBitWalletSignal::EditBitWallet) }
    };
    let name_string = match management_type {
        TransactionManagementTypes::Adding => { app.get_new_bit_wallet_state().name_string() }
        TransactionManagementTypes::Editing => { app.get_edit_bit_wallet_state().name_string() }
    };
    let coin_string = match management_type {
        TransactionManagementTypes::Adding => { app.get_new_bit_wallet_state().coin_string() }
        TransactionManagementTypes::Editing => { app.get_edit_bit_wallet_state().coin_string() }
    };
    let is_valid = BitWallet::are_raw_parts_valid(name_string, coin_string);

    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::success(),
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        icon("check"),
        signal,
        is_valid,
    )
}

/// Cancels.
#[must_use]
fn cancel_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardContent,
            depth: Depths::Proud,
        },
        ButtonShapes::Wide,
        icon("xmark"),
        Signal::GeneralSignal(GeneralSignal::ChangePageTo(Pages::BitWallets)),
        true,
    )
}

/// Deletes a `Transaction`.
#[must_use]
fn delete_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    let is_primed = app.get_edit_bit_wallet_state().is_delete_primed();

    if is_primed {
        row![
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::danger(),
                    depth: Depths::Proud,
                },
                ButtonShapes::Bloated,
                icon("trash"),
                Signal::EditBitWalletSignal(EditBitWalletSignal::RemoveBitWallet),
                true,
            ),
            spacer(Orientations::Horizontal, Spacing::Micro),
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::CardContent,
                    depth: Depths::Proud,
                },
                ButtonShapes::Bloated,
                icon("xmark"),
                Signal::EditBitWalletSignal(EditBitWalletSignal::UnprimeRemoveBitWallet),
                true,
            ),
        ]
        .spacing(Spacing::None.size())
        .align_y(Center)
        .into()
    }

    else {
        row![
            panel_button(
                app,
                MaterialStyle {
                    material: Materials::Plastic,
                    color: MaterialColors::danger(),
                    depth: Depths::Proud,
                },
                ButtonShapes::Wide,
                icon("trash"),
                Signal::EditBitWalletSignal(EditBitWalletSignal::PrimeRemoveBitWallet),
                true,
            ),
        ]
        .spacing(Spacing::None.size())
        .align_y(Center)
        .into()
    }
}