use iced::{Center, Fill};
use iced::Element;
use iced::widget::{Stack, container, scrollable, stack};
use iced::widget::column;
use iced::widget::row;
use iced::widget::scrollable::{Direction, Scrollbar};
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::pages::transactions_page::tag_panel;
use crate::ui::components::*;
use crate::ui::material::{MaterialColors, MaterialStyle, Materials};
use crate::vault::transaction::Tag;

// tag registry page
/// The page used for managing the coloring of tags.
pub fn tag_registry_page<'a>(
    app: &'a App,
) -> Stack<'a, Signal> {
    stack![
        container(
            row![
                spacer(Orientations::Horizontal, Spacing::Small),
                navigation_panel(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                tag_registry_panel(app),
                spacer(Orientations::Horizontal, Spacing::Fill),
                spacer(Orientations::Horizontal, Spacing::Small),
            ]
            .spacing(Spacing::None.size())
        )
        .center_x(Fill),
        
        header(
            app,
            Vec::new(),
            Vec::new(),
        )
    ]
        .width(Fill)
        .height(Fill)
}

/// A panel used to edit the tag registry
pub fn tag_registry_panel<'a>(
    app: &'a App,
) -> Element<'a, Signal> {
    container(
        panel(
            app,
            MaterialStyle {
                material: Materials::Plastic,
                color: MaterialColors::Background,
                strength: 2,
                cast_shadow: true,
            },
            PanelSize { width: Widths::LargeCard, height: Heights::LargeCard },
           PaddingSizes::Medium, {
                let tag_resgistration_slip_states: &Vec<TagRegistrationSlipState> = app.tag_registry_slip_state_manager.get_states();
                
                column![
                    // title
                    row![
                        ui_string(app, 1, "Tag Registry".to_string(), TextSizes::LargeHeading),
                        spacer(Orientations::Horizontal, Spacing::Fill),
                    ]
                    .align_y(Center),
                    
                    // tag registrations
                    spacer(Orientations::Vertical, Spacing::Large),
                    panel(
                        app,
                        MaterialStyle {
                            material: Materials::Plastic,
                            color: MaterialColors::Background,
                            strength: 1,
                            cast_shadow: false,
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
    )
    .center_x(Fill)
    .center_y(Fill)
    .into()
}

/// Edits the color of an individual tag
pub fn tag_registration_slip<'a>(
    app: &'a App,
    state: &'a TagRegistrationSlipState,
) -> Element<'a, Signal> {
    
    row![
        tag_panel(app, state.get_tag()),
        
        spacer(Orientations::Horizontal, Spacing::Medium),
        {
            if state.is_expanded {
                panel(
                    app,
                    MaterialStyle {
                        material: Materials::Plastic,
                        color: MaterialColors::Background,
                        strength: 2,
                        cast_shadow: true,
                    },
                    PanelSize { width: Widths::Fill, height: Heights::Shrink },
                    PaddingSizes::None, {
                        let mut color_selection_buttons = MaterialColors::standard_colors().into_iter().map(|color| {
                            let button_color = if app.bank.tag_registry.get(state.get_tag()) == color { color } else { MaterialColors::Unavailable };
                                
                            panel_button(
                                app,
                                MaterialStyle {
                                    material: Materials::RimmedPlastic,
                                    color: button_color,
                                    strength: 1,
                                    cast_shadow: true,
                                },
                                ButtonShapes::LowProfile,
                                ui_string(app, 1, color.name(), TextSizes::Interactable),
                                Signal::SetTagColor(state.get_tag().clone(), color),
                                true,
                            )
                        }).collect::<Vec<_>>();
                        color_selection_buttons.insert(0, spacer(Orientations::Horizontal, Spacing::Small));
                        color_selection_buttons.push(spacer(Orientations::Horizontal, Spacing::Small));
                        
                    column![
                        spacer(Orientations::Vertical, Spacing::Small),
                        
                        scrollable(
                            row(color_selection_buttons)
                                .spacing(Spacing::None.size())
                        )
                        .direction(Direction::Horizontal(Scrollbar::hidden()))
                        .spacing(Spacing::None.size()),
                        
                        spacer(Orientations::Vertical, Spacing::Small),
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
                        material: Materials::RimmedPlastic,
                        color: MaterialColors::Background,
                        strength: 2,
                        cast_shadow: true,
                    },
                    ButtonShapes::Minimal,
                    ui_string(app, 1, "Edit Color".to_string(), TextSizes::Interactable),
                    ExpandTag(state.get_tag().clone()),
                    true,
                )
            }
        }
    ]
    .spacing(Spacing::None.size())
    .align_y(Center)
    .into()
}



/// Manages the states of every tag registration slip in the app.
pub struct TagRegistrationSlipStateManager {
    /// The states of all the slips.
    /// The slips correspond to the tags in the bank.
    slips_states: Vec<TagRegistrationSlipState>,
}
impl TagRegistrationSlipStateManager {
    /// Creates a new slip state manager with the given tags.
    pub fn new(tags: Vec<Tag>) -> Self {
        TagRegistrationSlipStateManager {
            slips_states: tags.into_iter().map(|tag| TagRegistrationSlipState::new(tag)).collect(),
        }
    }
    
    /// Returns a reference to the states of all the slips.
    pub fn get_states(&self) -> &Vec<TagRegistrationSlipState> {
        &self.slips_states
    }
    
    /// Expands the slip for the given tag and collapses all others.
    pub fn expand(&mut self, tag: &Tag) {
        for state in &mut self.slips_states {
            state.is_expanded = state.tag == *tag;
        }
    }
    
    /// Collapses the slip for the given tag.
    pub fn collapse(&mut self, tag: &Tag) {
        for state in &mut self.slips_states {
            if state.tag == *tag {
                state.is_expanded = false;
                return;
            }
        }
    }
}

/// Holds the state of a single tag registration slip.
pub struct TagRegistrationSlipState {
    /// the tag associated with the slip.
    tag: Tag,
    /// whether the slip is expanded or not.
    pub is_expanded: bool,
}
impl TagRegistrationSlipState {
    /// Creates a new slip state with for given tag.
    pub fn new(tag: Tag) -> TagRegistrationSlipState {
        TagRegistrationSlipState { tag, is_expanded: false }
    }
    
    /// Returns a reference to the tag associated with the slip.
    pub fn get_tag(&self) -> &Tag {
        &self.tag
    }
}