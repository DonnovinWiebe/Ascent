use iced::wgpu::wgc::device::MissingDownlevelFlags;
use iced::{Center, Length};
use iced::{Color, Element};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::button;
use iced::widget::text_editor::{Content, Action};
use iced_font_awesome::fa_icon_solid as icon;
use crate::container::app::App;
use crate::container::signal::Signal;
use crate::ui::components::PaddingSizes;
use crate::ui::material::{MaterialColors, Materials};
use crate::container::signal::Signal::*;
use iced::widget::canvas::{self, Frame, Path};
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

// ring chart
/// A ring chart visualization of tag-grouped earning or spending.
#[derive(Debug, Clone, PartialEq)]
pub struct RingChart {
    segments: Vec<Segment>,
}
impl canvas::Program<Signal> for RingChart {
    type State = ();
    
    fn draw(&self, _state: &(), renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        
        for segment in &self.segments {
            let center = iced::Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let max_size = bounds.width.min(bounds.height) / 2.0;
            let level_offset = segment.get_level() as f32 * (RingChart::THICKNESS + RingChart::LEVEL_SPACING.size());
            let percentage_angle = Radians(segment.get_visual_percentage() * (2.0 * PI));
            let radius = max_size - level_offset - (RingChart::THICKNESS / 2.0);
            let start_angle = Radians(segment.get_offset() * (2.0 * PI));
            let end_angle = start_angle + percentage_angle;
            
            let path = Path::new(|p| {
                
                p.arc(canvas::path::Arc {
                    center,
                    radius,
                    start_angle: start_angle,
                    end_angle: end_angle,
                });
            });
            frame.stroke(&path,
                canvas::Stroke::default()
                    .with_width(RingChart::THICKNESS)
                    .with_color(segment.get_color())
            );
        }
        
        vec![frame.into_geometry()]
    }
}
impl<'a> From<RingChart> for Element<'a, Signal> {
    fn from(chart: RingChart) -> Self {
        canvas(chart)
            .width(Length::Fill)
            .into()
    }
}
impl RingChart {
    // constants
    /// The thickness of the ring.
    const THICKNESS: f32 = 16.0;
    /// The spacing between levels of rings.
    const LEVEL_SPACING: PaddingSizes = PaddingSizes::Small;
    
    
    
    /// Creates a new RingChart.
    pub fn new(app: &App, bank: &Bank, filter: Filters, flow_direction: FlowDirections) -> ResultStack<RingChart> {
        let ring_parse_result = RingParse::new(app, bank, filter, flow_direction);
        match ring_parse_result {
            Pass(ring_parse) => Pass(RingChart { segments: ring_parse.get_ring_data() }),
            Fail(_) => ResultStack::new_fail_from_stack(ring_parse_result.get_stack()).fail("Failed to create RingChart."),
        }
    }
}

/// An individual segment of a RingChart representing one tag with all earning or spending transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    tag: Tag,
    color: Color,
    percentage: f32,
    visual_percentage: f32,
    offset_percentage: f32,
    level: usize,
}
impl Segment {
    // constants
    /// Defines the minimum visual percentage for a segment.
    pub const MINIMUM_VISUAL_PERCENTAGE: f32 = 0.05;
    /// Defines the spacing between segments in Radians.
    pub const SPACING: f32 = 0.025;
    
    
    
    // basic getters
    /// Gets the tag.
    pub fn get_tag(&self) -> Tag {
        self.tag.clone()
    }
    
    /// Gets the color.
    pub fn get_color(&self) -> Color {
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
    pub fn get_offset(&self) -> f32 {
        self.offset_percentage
    }
    
    /// Gets the level.
    pub fn get_level(&self) -> usize {
        self.level
    }
    
    
    
    /// Returns a new Segment.
    pub fn new(app: &App, tag: Tag, color: MaterialColors, percentage: f32, offset_percentage: f32, level: usize) -> ResultStack<Segment> {
        let visual_percentage = percentage.max(Self::MINIMUM_VISUAL_PERCENTAGE);
        if percentage <= 0.0 || percentage > 1.0 {
            return ResultStack::new_fail("Segment percentage must be between 0.0 and 1.0!").fail("Failed to create Segment.");
        }
        if offset_percentage < 0.0 || offset_percentage > 1.0 {
            return ResultStack::new_fail("Segment offset must be between 0.0 and 1.0!").fail("Failed to create Segment.");
        }
        
        Pass(Segment { tag, color: color.themed(&app.theme_selection, 1), percentage, visual_percentage, offset_percentage, level })
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
            sum_visual_percentage += ring[i].visual_percentage;
            if i != 0 { sum_visual_percentage += Segment::SPACING; }
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
}