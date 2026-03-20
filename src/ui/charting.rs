use iced::wgpu::wgc::device::MissingDownlevelFlags;
use iced::widget::image::Handle;
use iced::{Center, Length};
use iced::{Color, Element};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button;
use iced::widget::text_editor::{Content, Action};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::{BorderThickness, Heights, PaddingSizes, TextSizes, Widths, ui_string};
use crate::ui::material::{AppThemes, MaterialColors, Materials};
use crate::container::signal::Signal::*;
use iced::widget::canvas::{self, Frame};
use iced::mouse::Cursor;
use iced::Rectangle;
use crate::vault::transaction::*;
use crate::vault::bank::*;
use crate::vault::filter::*;
use std::f32::consts::PI;
use iced::Radians;
use crate::vault::result_stack::ResultStack;
use crate::vault::result_stack::ResultStack::*;
use crate::vault::parse::*;
use std::cmp::Ordering;
use tiny_skia::*;

// Ring Chart
/// An individual segment of a RingChart representing one tag with all earning or spending transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    tag: Tag,
    color: MaterialColors,
    percentage: f32,
    visual_percentage: f32,
    offset_percentage: f32,
    level: usize,
}
impl Segment {
    // constants
    /// Defines the minimum visual percentage for a segment.
    const MINIMUM_VISUAL_PERCENTAGE: f32 = 0.05;
    /// The thickness of the ring.
    const THICKNESS: f32 = 24.0;
    /// Defines the spacing between segments in percentage.
    const SPACING: f32 = 0.015;
    /// The spacing between levels of rings.
    const LEVEL_SPACING: PaddingSizes = PaddingSizes::Micro;
    /// The maximum size of the ring chart, based on the width of the Transaction Management Panel.
    fn max_size() -> u32 {
        let home_panel_width = Widths::SmallCard.size();
        let home_panel_internal_padding = PaddingSizes::Small.size();
        (home_panel_width - (2.0 * home_panel_internal_padding)) as u32
    }
    /// The border thickness of the segment.
    fn border_thickness() -> f32 {
        BorderThickness::Thin.size()
    }
    
    
    
    // basic getters
    /// Gets the tag.
    pub fn get_tag(&self) -> Tag {
        self.tag.clone()
    }

    /// Gets the color.
    pub fn get_color(&self) -> MaterialColors {
        self.color.clone()
    }

    /// Gets the percentage.
    pub fn get_percentage(&self) -> f32 {
        self.percentage
    }

    /// Gets the visual percentage
    pub fn get_visual_percentage(&self) -> f32 {
        self.visual_percentage
    }

    /// Gets the offset.
    pub fn get_offset_percentage(&self) -> f32 {
        self.offset_percentage
    }

    /// Gets the level.
    pub fn get_level(&self) -> usize {
        self.level
    }



    // segment work
    /// Returns a new Segment.
    pub fn new(tag: Tag, color: MaterialColors, percentage: f32, offset_percentage: f32, level: usize) -> ResultStack<Segment> {
        let visual_percentage = percentage.max(Self::MINIMUM_VISUAL_PERCENTAGE);
        if percentage <= 0.0 || percentage > 1.0 {
            return ResultStack::new_fail("Segment percentage must be between 0.0 and 1.0!").fail("Failed to create Segment.");
        }
        if offset_percentage < 0.0 || offset_percentage > 1.0 {
            return ResultStack::new_fail("Segment offset must be between 0.0 and 1.0!").fail("Failed to create Segment.");
        }

        Pass(Segment { tag, color, percentage, visual_percentage, offset_percentage, level })
    }

    /// Updates the offsets in a list of segments.
    pub fn update_offsets_for(ring: &mut Vec<Segment>) -> ResultStack<()> {
        for i in 0..ring.len() {
            let used_space_result = Segment::get_visual_percentage_before_position(ring, i);
            match used_space_result {
                Pass(used_space) => {
                    ring[i].offset_percentage = used_space;
                }
                Fail(_) => { return used_space_result.empty_type().fail("Failed to update offsets in ring.") }
            }
        }
        Pass(())
    }

    /// Gets the visual percentage (with offsets) of all the segments before the segment at the given position (index) in a ring.
    fn get_visual_percentage_before_position(ring: &Vec<Segment>, position: usize) -> ResultStack<f32> {
        if position >= ring.len() {
            return ResultStack::new_fail("Position/index out of bounds!").fail("Failed to get visual percentage up to position in a ring.");
        }

        if ring.is_empty() { return Pass(0.0) }

        let mut sum_visual_percentage = 0.0;
        for i in 0..position {
            sum_visual_percentage += ring[i].visual_percentage + Segment::SPACING;
        }

        Pass(sum_visual_percentage)
    }

    pub fn update_levels_for(rings: &mut Vec<Vec<Segment>>) {
        for (level, ring) in rings.iter_mut().enumerate() {
            for segment in ring {
                segment.level = level;
            }
        }
    }

    /// Returns the sum percent of all the segments in a given list.
    pub fn sum_visual_percent(segments: &Vec<Segment>) -> f32 {
        let mut sum_visual_percentage = segments.iter().map(|s| s.visual_percentage).sum();
        sum_visual_percentage += Segment::SPACING * (segments.len() as f32 - 1.0);
        sum_visual_percentage
    }

    /// Checks if a given segment fits into a collection of Segments.
    pub fn fits_into(&self, segments: &Vec<Segment>) -> bool {
        let used_space = Segment::sum_visual_percent(segments);
        1.0 - used_space >= self.visual_percentage + Segment::SPACING
    }

    /// Returns a sorted copy of the segments by percentage.
    pub fn sorted(segments: Vec<Segment>) -> Vec<Segment> {
        let mut segments = segments.to_vec();
        segments.sort_by(|a, b| a.percentage.partial_cmp(&b.percentage).unwrap_or(Ordering::Equal));
        segments
    }

    pub fn is_safe(segments: &Vec<Segment>) -> bool {
        Segment::sum_visual_percent(segments) <= 1.0
    }



    // drawing
    pub fn render<'a>(&self, app: &'a App) -> Element<'a, Signal> {
        let handle_result = self.generate_handle(app);
        
        if handle_result.is_fail() {
            ui_string(app, 1, "Failed to generate image handle for Segment.".to_string(), TextSizes::Interactable)
        }
        else {
            let handle = handle_result.wont_fail("This is past an is_fail() guard clause.");
            image(handle).into()
        }
    }
    
    fn generate_handle(&self, app: &App) -> ResultStack<Handle> {
        // pixmap
        let max_size: u32 = Segment::max_size();
        let pixmap_result = ResultStack::from_option(Pixmap::new(max_size, max_size), "Failed to create Pixmap while generating Segmane image handle.");
        if pixmap_result.is_fail() { return ResultStack::new_fail_from_stack(pixmap_result.get_stack()).fail("Failed to generate Segment image handle."); }
        let mut pixmap = pixmap_result.wont_fail("This is past an is_fail() guard clause.");
        
        // coloring
        let mut fill_paint = Paint::default();
        let iced_fill_color = self.color.themed(&app.theme_selection, 1);
        let r = (iced_fill_color.r * 255.0) as u8;
        let g = (iced_fill_color.g * 255.0) as u8;
        let b = (iced_fill_color.b * 255.0) as u8;
        let a = (iced_fill_color.a * 255.0) as u8;
        fill_paint.set_color_rgba8(r, g, b, a);
        fill_paint.anti_alias = true;
        
        let mut stroke_paint = Paint::default();
        let iced_stroke_color = self.color.themed(&app.theme_selection, 2);
        let r = (iced_stroke_color.r * 255.0) as u8;
        let g = (iced_stroke_color.g * 255.0) as u8;
        let b = (iced_stroke_color.b * 255.0) as u8;
        let a = (iced_stroke_color.a * 255.0) as u8;
        stroke_paint.set_color_rgba8(r, g, b, a);
        stroke_paint.anti_alias = true;
        
        

        // coloring
        let fill_path_result = self.generate_segment_path(false);
        let stroke_path_result = self.generate_segment_path(true);
        if fill_path_result.is_fail() { return ResultStack::new_fail_from_stack(fill_path_result.get_stack()).fail("Failed to generate Segment image handle."); }
        if stroke_path_result.is_fail() { return ResultStack::new_fail_from_stack(stroke_path_result.get_stack()).fail("Failed to generate Segment image handle."); }
        pixmap.fill_path(&fill_path_result.wont_fail("This is past an is_fail() guard clause."), &fill_paint, FillRule::Winding, Transform::identity(), None);
        pixmap.stroke_path(&stroke_path_result.wont_fail("This is past an is_fail() guard clause."), &stroke_paint, &Stroke { width: BorderThickness::Standard.size(), ..Default::default() }, Transform::identity(), None);

        // returning
        Pass(Handle::from_rgba(max_size, max_size, pixmap.take()))
    }
    
    fn generate_segment_path(&self, is_stroke: bool) -> ResultStack<Path> {
        // bounds
        let max_size: u32 = Segment::max_size();
        let center_x = max_size as f32 / 2.0;
        let center_y = center_x;
        
        // sizing
        let percentage_angle = self.visual_percentage * (2.0 * PI);
        let level_offset = self.level as f32 * (Segment::THICKNESS + Segment::LEVEL_SPACING.size());
        let outer_radius_stroke_modifier = if is_stroke { Segment::border_thickness() / 2.0 } else { 0.0 };
        let outer_radius: f32 = (max_size as f32) / 2.0 - level_offset - outer_radius_stroke_modifier;
        let inner_radius = outer_radius - Segment::THICKNESS + outer_radius_stroke_modifier;
        let start_angle = self.offset_percentage * (2.0 * PI);
        let steps = (outer_radius * 0.5).max(32.0) as usize;
        
        // building the path of the segment
        let mut path = PathBuilder::new();
        
        // start point
        path.move_to(
            center_x + (outer_radius * start_angle.cos()),
            center_y + (outer_radius * start_angle.sin()),
        );
        // drawing the outer arc
        for i in 1..=steps {
            let progress = i as f32 / steps as f32;
            let angle = start_angle + (percentage_angle * progress);
            path.line_to(
                center_x + (outer_radius * angle.cos()),
                center_y + (outer_radius * angle.sin()),
            );
        }
        // moving to the inner arc
        path.line_to(
            center_x + (inner_radius * (start_angle + percentage_angle).cos()),
            center_y + (inner_radius * (start_angle + percentage_angle).sin()),
        );
        // drawing the inner arc
        for i in 1..=steps {
            let progress = 1.0 - (i as f32 / steps as f32);
            let angle = start_angle + (percentage_angle * progress);
            path.line_to(
                center_x + (inner_radius * angle.cos()),
                center_y + (inner_radius * angle.sin()),
            );
        }
        // completing
        path.line_to(
            center_x + (outer_radius * start_angle.cos()),
            center_y + (outer_radius * start_angle.sin()),
        );

        ResultStack::from_option(path.finish(), "Failed to draw segment geometry.")
    }
}
