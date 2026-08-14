use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::{App, Pages};
use crate::container::state::TagRegistrationSlipState;
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::signal::{Signal, TagRegistrySignal};
use crate::pages::transactions_page::tag_panel;
use materialui::components::{ButtonShapes, Heights, Orientations, PaddingSizes, PanelSize, Spacing, TextSizes, Widths, header, navigation_panel, panel, panel_button, spacer, ui_string};
use materialui::materials::{Depths, MaterialColors, MaterialStyle, Materials};
use crate::vault::transaction::Tag;

/// The page used for managing the persistent coloring of `Tag`s.
#[must_use]
pub fn tag_registry_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        row![
            navigation_panel(app, Pages::page_pointers(app)),
            container(tag_registry_panel(app)).center(Fill),
        ],
        header(app, Vec::new()),
    ]
}

/// A panel used to edit the `TagRegistry`.
#[must_use]
fn tag_registry_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    panel(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::Card,
            depth: Depths::Proud,
        },
        PanelSize { width: Widths::LargeCard, height: Heights::LargeCard },
        PaddingSizes::Small, {
            let tag_resgistration_slip_states: &Vec<TagRegistrationSlipState> = app.get_tag_registry_slip_state_manager().states();
            
            column![
                // title
                row![
                    ui_string(app, "Tag Registry", TextSizes::LargeHeading, MaterialColors::StrongText),
                    spacer(Orientations::Horizontal, Spacing::Fill),
                ]
                .align_y(Center),
                
                // tag registrations
                spacer(Orientations::Vertical, Spacing::Large),
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
                                let mut tag_registration_slips = tag_resgistration_slip_states.iter().map(|state| { tag_registration_slip(app, state) }).collect::<Vec<_>>();
                                tag_registration_slips.insert(0, spacer(Orientations::Vertical, Spacing::Medium));
                                tag_registration_slips.push(spacer(Orientations::Vertical, Spacing::Medium));
                                
                                column(tag_registration_slips)
                                    .width(Fill)
                                    .spacing(Spacing::Medium.size())
                            })
                            .direction(Direction::Vertical(Scrollbar::hidden())),
                            //.spacing(Spacing::Medium.size()),
                            
                            spacer(Orientations::Horizontal, Spacing::Medium),
                        ]
                        .into()
                    }
                )
            ]
            .spacing(Spacing::None.size())
            .into()
        }
    )
}

/// Edits the color of an individual `Tag`.
#[must_use]
fn tag_registration_slip<'a>(
    app: &'a App,
    state: &'a TagRegistrationSlipState,
) -> Element<'a, Signal> {
    
    row![
        tag_panel(app, state.tag()),
        
        spacer(Orientations::Horizontal, Spacing::None),
        reset_registration_button(app, state),
        
        spacer(Orientations::Horizontal, Spacing::Medium),
        {
            if state.is_expanded() {
                panel(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::Card,
                        depth: Depths::Proud,
                    },
                    PanelSize { width: Widths::Fill, height: Heights::Shrink },
                    PaddingSizes::None, {
                        let mut color_selection_buttons = MaterialColors::standard_colors().into_iter().map(|color| {
                            panel_button(
                                app,
                                MaterialStyle {
                                    material: Materials::Plastic,
                                    color,
                                    depth: Depths::Proud,
                                },
                                ButtonShapes::LowProfile,
                                ui_string(app, color.name(), TextSizes::Interactable, MaterialColors::StrongText),
                                Signal::TagRegistrySignal(TagRegistrySignal::SetTagColor(state.tag().clone(), color)),
                                true,
                            )
                        }).collect::<Vec<_>>();
                        color_selection_buttons.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
                        color_selection_buttons.push(spacer(Orientations::Horizontal, Spacing::Small));

                        column![
                            spacer(Orientations::Vertical, Spacing::Nano),
                            
                            scrollable(
                                row(color_selection_buttons)
                                    .spacing(Spacing::None.size())
                            )
                            .direction(Direction::Horizontal(Scrollbar::hidden()))
                            .spacing(Spacing::None.size()),
                            
                            spacer(Orientations::Vertical, Spacing::Nano),
                        ]
                        .spacing(Spacing::None.size())
                        .into()
                    }
                )
            }
            else {
                panel_button(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::CardHollowContent,
                        depth: Depths::Proud,
                    },
                    ButtonShapes::LowProfile,
                    ui_string(app, "Edit Color", TextSizes::Interactable, MaterialColors::StrongText),
                    Signal::TagRegistrySignal(TagRegistrySignal::ExpandTag(state.tag().clone())),
                    true,
                )
            }
        }
    ]
    .spacing(Spacing::None.size())
    .align_y(Center)
    .into()
}

/// Resets the color of a `Tag` in the `TagRegistry`.
#[must_use]
fn reset_registration_button<'a>(
    app: &'a App,
    state: &'a TagRegistrationSlipState,
) -> Element<'a, Signal> {
    panel_button(
        app,
        MaterialStyle {
            material: Materials::Plastic,
            color: MaterialColors::CardHollowContent,
            depth: Depths::Proud
        },
        ButtonShapes::Minimal,
        icon("arrow-rotate-left"),
        Signal::TagRegistrySignal(TagRegistrySignal::ResetTag(state.tag().clone())),
        true
    )
}