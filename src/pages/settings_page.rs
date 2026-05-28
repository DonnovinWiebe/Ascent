use iced_font_awesome::fa_icon_solid as icon;
use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, panel, panel_button, panel_text_input, spacer, ui_string};
use crate::ui::material::{AppThemes, Depths, MaterialColors, MaterialStyle, Materials};
use crate::vault::bank::{ExchangeRate, ExchangeRateStatus};
use crate::vault::parse::FlowTypes;
use crate::vault::transaction::Transaction;

/// The page used to display settings for the `App`.
#[must_use]
pub fn settings_page<'a>(
    app: &'a App
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app),
            container(settings_list(app)).center_x(Fill),
        ],
        header(app, Vec::new()),
    ]
}

/// The list of settings
#[must_use]
fn settings_list<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    scrollable(
        column![
            spacer(Orientations::Vertical, Spacing::HeaderSpace),
            
            // appearance
            setting_heading(app, "Appearance".to_string()),
            theme_setting(app),
            
            // save data
            spacer(Orientations::Vertical, Spacing::Large),
            setting_heading(app, "Save Data".to_string()),
            backup_button(app),
            save_data_import_button(app),
            legacy_save_data_import_button(app),

            // currency exchange
            spacer(Orientations::Vertical, Spacing::Large),
            setting_heading(app, "Currency Exchange".to_string()),
            flow_type_setting(app),
            main_currency_overlay(app),
            exchange_rate_panel_overlay(app),
        ]
        .spacing(Spacing::Medium.size())
    )
    .direction(Direction::Vertical(Scrollbar::hidden()))
    .width(Widths::LargeCard.size())
    .height(Fill)
    .into()
}

/// Provides a label to group related settings.
#[must_use]
fn setting_heading<'a>(
    app: &'a App,
    label: String,
) -> Element<'a, Signal> {
    row![
        ui_string(app, label, TextSizes::LargeHeading, MaterialColors::StrongText),
        spacer(Orientations::Horizontal, Spacing::Fill),
    ]
    .into()
}

/// The theme selection setting.
#[must_use]
fn theme_setting<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Theme", TextSizes::SmallHeading, MaterialColors::StrongText),
        // peach
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::Peach {
                    MaterialColors::accent(AppThemes::Peach)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::Peach.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::Peach),
            true,
        ),
        
        // sunrise
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::Sunrise {
                    MaterialColors::accent(AppThemes::Sunrise)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::Sunrise.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::Sunrise),
            true,
        ),

        // midnight
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::Midnight {
                    MaterialColors::accent(AppThemes::Midnight)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::Midnight.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::Midnight),
            true,
        ),
        
        // dark forest
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: if app.theme_selection == AppThemes::DarkForest {
                    MaterialColors::accent(AppThemes::DarkForest)
                }
                else { MaterialColors::Card },
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            ui_string(app, AppThemes::DarkForest.name(), TextSizes::Interactable, MaterialColors::StrongText),
            Signal::ChangeTheme(AppThemes::DarkForest),
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The save data backup button.
#[must_use]
fn backup_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Create Backup", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("floppy-disk"),
            Signal::Backup,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The save data import button.
#[must_use]
fn save_data_import_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Import Save Data", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("file-import"),
            Signal::OpenImportFilePicker,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// The legacy save data import button.
#[must_use]
fn legacy_save_data_import_button<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Import Legacy Save Data", TextSizes::SmallHeading, MaterialColors::StrongText),
        panel_button(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            ButtonShapes::Standard,
            icon("file-import"),
            Signal::OpenLegacyImportFilePicker,
            true,
        ),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// Position the main `Currency` panel and input.
#[must_use]
fn main_currency_overlay<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        main_currency_panel(app),
        main_currency_input(app),
    ]
    .spacing(Spacing::Medium.size())
    .align_y(Center)
    .into()
}

/// Displays the current main `Currency`.
#[must_use]
fn main_currency_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Main Currency", TextSizes::SmallHeading, MaterialColors::StrongText),
        spacer(Orientations::Horizontal, Spacing::Medium),
        panel(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Card,
                depth: Depths::Proud,
            },
            PanelSize { width: Widths::Shrink, height: Heights::Shrink },
            PaddingSizes::Small, {
                let main_currency = app.bank.currency_exchange.get_main_currency();
                ui_string(app, &format!("{} {}", main_currency.symbol, main_currency.to_string()), TextSizes::Interactable, MaterialColors::StrongText)
            }
        )
    ]
    .align_y(Center)
    .spacing(0)
    .into()
}

/// Allows the input of a new main `Currency`.
#[must_use]
fn main_currency_input<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    let error = app.new_main_currency_string != "".to_string() && !Transaction::is_currency_string_valid(&app.new_main_currency_string);
    
    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if error { MaterialColors::danger() } else { MaterialColors::Card },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "New Currency",
        &app.new_main_currency_string,
        Signal::UpdateNewMainCurrencyString,
        Some(Signal::SetMainCurrency),
        true,
    )
}

/// Holds the flow type options.
#[must_use]
fn flow_type_setting<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        ui_string(app, "Flow Type", TextSizes::SmallHeading, MaterialColors::StrongText),
        flow_typelet(app, FlowTypes::Collected),
        flow_typelet(app, FlowTypes::Unified),
        flow_typelet(app, FlowTypes::Time),
    ]
    .spacing(Spacing::Small.size())
    .align_y(Center)
    .into()
}

/// Selects a new flow type.
#[must_use]
fn flow_typelet<'a>(
    app: &'a App,
    flow_type: FlowTypes,
) -> Element<'a, Signal> {
    let color = if app.bank.currency_exchange.get_flow_type() == flow_type { MaterialColors::accent(app.theme_selection) } else { MaterialColors::Card };
    let label = match flow_type {
        FlowTypes::Collected => "Collected",
        FlowTypes::Unified => "Unified",
        FlowTypes::Time => "Time",
    };
    
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: color,
            depth: Depths::Proud,
        },
        ButtonShapes::Minimal,
        label,
        Signal::SetFlowType(flow_type),
        true,
    )
}

/// Positions the exchange rate panel.
#[must_use]
fn exchange_rate_panel_overlay<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    row![
        exchange_rate_panel(app),
        spacer(Orientations::Horizontal, Spacing::Fill),
    ]
    .spacing(0)
    .into()
}

/// Holds the `ExchangeRate`s from the `Bank`'s `CurrencyExchange` and allows them to be edited.
#[must_use]
fn exchange_rate_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::MediumCard, height: Heights::MediumCard },
        PaddingSizes::Small, {
            column![
                ui_string(app, "Exchange Rates", TextSizes::Interactable, MaterialColors::StrongText),
                
                spacer(Orientations::Vertical, Spacing::Medium),
                panel(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::CardHollow,
                        depth: Depths::Recessed,
                    },
                    PanelSize { width: Widths::Fill, height: Heights::Fill },
                    PaddingSizes::None, {
                        row![
                            spacer(Orientations::Horizontal, Spacing::Medium),
                            
                            scrollable({
                                let mut exchange_rate_slips: Vec<_> = app.bank.currency_exchange.get_rates().into_iter().map(|r| { exchange_rate_slip(app, r) }).collect();
                                exchange_rate_slips.insert(0, spacer(Orientations::Vertical, Spacing::Small));
                                exchange_rate_slips.push(spacer(Orientations::Vertical, Spacing::Small));
                                
                                column(exchange_rate_slips)
                                    .width(Fill)
                                    .spacing(Spacing::Micro.size())
                            })
                            .direction(Direction::Vertical(Scrollbar::hidden())),
                            //.spacing(Spacing::Medium.size()),
                            
                            spacer(Orientations::Horizontal, Spacing::Medium),
                        ]
                        .into()
                    }
                )
            ]
            .spacing(0)
            .into()
        }
    )
}

/// Holds an individual `ExchangeRate` and allows it to be edited.
#[must_use]
fn exchange_rate_slip<'a>(
    app: &'a App,
    rate: &'a ExchangeRate,
) -> Element<'a, Signal> {
    row![
        exchange_rate_status_panel(app, rate),
        spacer(Orientations::Horizontal, Spacing::Large),
        ui_string(app, &format!("1 {} → {} {}", rate.get_from(), rate.get_rate(), rate.get_to()), TextSizes::Interactable, MaterialColors::StrongText),
        spacer(Orientations::Horizontal, Spacing::Fill),
        new_rate_field(app, rate),
    ]
    .align_y(Center)
    .spacing(0)
    .into()
}

/// Displays the status of an `ExchangeRate`.
#[must_use]
fn exchange_rate_status_panel<'a>(
    app: &'a App,
    rate: &'a ExchangeRate,
) -> Element<'a, Signal> {
    match rate.get_status() {
        ExchangeRateStatus::Invalid => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Acrylic,
                    color: MaterialColors::danger(),
                    depth: Depths::Flat,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Small,
                icon("ban").into(),
            )
        }
        ExchangeRateStatus::Warning => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Acrylic,
                    color: MaterialColors::warning(),
                    depth: Depths::Flat,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Small,
                row![icon("triangle-exclamation"), ui_string(app, &format!("{} days old", rate.get_age()), TextSizes::Interactable, MaterialColors::StrongText)].align_y(Center).spacing(0).into(),
            )
        }
        ExchangeRateStatus::Valid => {
            panel(
                app,
                MaterialStyle {
                    material: Materials::Acrylic,
                    color: MaterialColors::success(),
                    depth: Depths::Flat,
                },
                PanelSize { width: Widths::Shrink, height: Heights::Shrink },
                PaddingSizes::Small,
                icon("circle-check").into(),
            )
        }
    }
}

/// Allows an `ExchangeRate` to be edited.
#[must_use]
fn new_rate_field<'a>(
    app: &'a App,
    rate: &'a ExchangeRate,
) -> Element<'a, Signal> {
    let on_change = |new_rate_string: String| Signal::UpdateNewExchangeRateString(rate.get_from().to_string(), rate.get_to().to_string(), new_rate_string);
    let on_submit_option = Some(Signal::TrySetNewExchangeRate(
        rate.get_from().to_string(),
        rate.get_to().to_string(),
        rate.new_rate_string.clone(),
    ));
    let error = rate.new_rate_string != "".to_string() && !rate.is_new_rate_string_valid();
    panel_text_input(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: if error { MaterialColors::danger() } else { MaterialColors::CardHollowContent },
            depth: Depths::Proud,
        },
        Widths::MicroField,
        "New Rate",
        &rate.new_rate_string,
        on_change,
        on_submit_option,
        true,
    )
}