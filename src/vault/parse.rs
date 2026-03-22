use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::{BorderThickness, PaddingSizes, TextSizes, Widths, ui_string};
use crate::ui::material::{AppThemes, MaterialColors, Materials};
use crate::vault::bank::Bank;
use crate::vault::bank::*;
use crate::vault::filter::*;
use crate::vault::result_stack::{FailureStack, ResultStack};
use crate::vault::result_stack::ResultStack::*;
use crate::vault::transaction::*;
use crate::vault::transaction::{Id, Transaction, Value};
use iced::{Radians};
use iced::Rectangle;
use iced::Size;
use iced::alignment::Vertical::Top;
use iced::mouse::Cursor;
use iced::widget::button;
use iced::widget::canvas::{self, Frame};
use iced::widget::text_editor::{Action, Content};
use iced::widget::*;
use iced::widget::{column, row};
use iced::widget::image::Handle;
use iced::{Center, Length};
use iced::{Color, Element};
use iced_font_awesome::fa_icon_solid as icon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;
use rusty_money::{Money, iso::Currency};
use tiny_skia::{Paint, Pixmap};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::cmp::Ordering;
use iced::Point;
use tiny_skia::*;
use iced::wgpu::wgc::device::MissingDownlevelFlags;
use iced::widget::*;
use crate::container::signal::Signal::*;
use crate::vault::transaction::*;
use crate::vault::bank::*;
use crate::vault::filter::*;
use crate::vault::result_stack::ResultStack::*;
use tiny_skia::*;

/// Provides enumerated options for the directions money can flow (in/out of your account).
#[derive(Debug, Clone, Copy)]
pub enum FlowDirections {
    Earning,
    Spending,
}
impl FlowDirections {
    /// Returns true if the given transaction value matches the flow direction.
    pub fn matches(&self, transaction: &Transaction) -> bool {
        match self {
            FlowDirections::Earning => transaction.value.is_positive(),
            FlowDirections::Spending => transaction.value.is_negative(),
        }
    }
}



/// Holds cash flow values for multiple currencies from a single list of transactions.
pub struct CashFlow {
    /// The list of transaction id's used.
    transaction_ids: Vec<Id>,
    /// The list of values grouped by currency.
    pub value_flows: Vec<Value>,
    /// The overall cash flow represented as a time price.
    pub time_flow: Vec<f64>,
}
impl CashFlow {
    /// Creates a new cash flows object from a list of transaction id's.
    pub fn new(transaction_ids: Vec<Id>, bank: &Bank) -> CashFlow {
        let value_flows = Self::get_value_flows(transaction_ids.clone(), bank);
        let time_flow = vec![Self::get_time_flow(&value_flows)];

        CashFlow {
            transaction_ids,
            value_flows,
            time_flow,
        }
    }

    /// Turns a list of transactions into a collection of values, grouped by currency, that each represent the overall cash flow for the given currency.
    fn get_value_flows(transaction_ids: Vec<Id>, bank: &Bank) -> Vec<Value> {
        // the list of all the transactions (by id) grouped into their currencies
        let mut coupled_value_groups: Vec<(Currency, Vec<Id>)> = Vec::new();

        // collects each transaction value into separate value groups
        for id in transaction_ids {
            // the current transaction
            let transaction = bank.get(id).unwrap(); // todo: this is temporary - fix later
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_value_groups {
                if transaction.value.currency().clone() == group.0 {
                    is_currency_used = true;
                    group.1.push(id);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used {
                coupled_value_groups.push((transaction.value.currency().clone(), vec![id]));
            }
        }

        // collects the coupled cash flow groups into individual values
        let value_flows: Vec<Value> = coupled_value_groups
            .into_iter()
            .map(|couple| {
                let mut flow: f64 = 0.0;
                for id in &couple.1 {
                    flow += bank
                        .get(id.clone())
                        .unwrap()
                        .value
                        .amount()
                        .to_f64()
                        .expect("Invalid transaction value!"); // todo: this is temporary - fix later
                }
                Value::from_minor(
                    (flow * 100.0) as i64,
                    bank.get(couple.1[0]).unwrap().value.currency(),
                ) // each couple is guaranteed to have at least one transaction // todo: this is temporary - fix later
            })
            .collect();

        // returns the cash flow groups
        value_flows
    }

    /// Gets the overall time flow value from a list of values.
    fn get_time_flow(value_flows: &Vec<Value>) -> f64 {
        let mut time_flow = 0.0;
        for value_flow in value_flows {
            time_flow += value_flow
                .amount()
                .to_f64()
                .expect("Invalid transaction value!"); //todo convert for currency
        }
        time_flow
    }
}



/// Holds the data that the RingChart displays.
pub struct RingParse {
    ring_data: Vec<Segment>,
    hovered_segment_tag: Option<Tag>,
    cached_handles: HashMap<Option<Tag>, Handle>,
    current_handle: Handle,
}

impl RingParse {
    // constants
    /// The maximum size of the ring chart, based on the width of the Transaction Management Panel.
    pub fn max_size() -> u32 {
        let home_panel_width = Widths::SmallCard.size();
        let home_panel_internal_padding = PaddingSizes::Small.size();
        (home_panel_width - (2.0 * home_panel_internal_padding)) as u32
    }
    
    
    
    // assembling
    /// Creates a new RingParse.
    pub fn new(app: &App, bank: &Bank, filter: Filters, flow_direction: FlowDirections) -> ResultStack<RingParse> {
        let max_size = RingParse::max_size();
        let ring_data_result = RingParse::assemble(app, bank, filter, flow_direction);
        let empty_pixmap_result = ResultStack::from_option(Pixmap::new(max_size, max_size), "Failed to create empty Pixmap for RingParse.");
        if empty_pixmap_result.is_fail() {
            return ResultStack::new_fail_from_stack(empty_pixmap_result.get_stack()).fail("Failed to create RingParse.");
        }
        let empty_pixmap = empty_pixmap_result.wont_fail("This is past an is_fail() guard clause.");
        
        match ring_data_result {
            Pass(ring_data) => Pass(
                RingParse {
                    ring_data,
                    hovered_segment_tag: None,
                    cached_handles: HashMap::new(),
                    current_handle: Handle::from_rgba(max_size, max_size, empty_pixmap.take()),
                }
            ),
            Fail(_) => ResultStack::new_fail_from_stack(ring_data_result.get_stack()).fail("Failed to create RingParse."),
        }
    }
    
    /// Gets the ring data.
    pub fn get_ring_data(&self) -> Vec<Segment> {
        self.ring_data.clone()
    }
    
    /// Assmebles rings of segments for a RingParse.
    fn assemble(app: &App, bank: &Bank, filter: Filters, flow_direction: FlowDirections) -> ResultStack<Vec<Segment>> {
        // getting the transactions from the filter
        let mut transactions: Vec<&Transaction> = Vec::new();
        let mut transaction_retrieval_failures: Vec<ResultStack<&Transaction>> = Vec::new();
        
        for id in bank.get_filtered_ids(filter) {
            let transaction_result = bank.get(id.clone());
            if transaction_result.is_pass() {
                transactions.push(transaction_result.wont_fail("This is inside an is_pass() block."));
            }
            else {
                transaction_retrieval_failures.push(transaction_result);
            }
        }
        
        // checking if there were any retrieval failures
        if !transaction_retrieval_failures.is_empty() {
            return ResultStack::new_fail_from_stack(transaction_retrieval_failures[0].get_stack()).fail("Failed to assemble rings for RingParse.");
        }
        
        // filters out transactions that do not match the flow direction
        transactions.retain(|t| { flow_direction.matches(t) });
        
        
        
        // assembles a list of segments from the tags
        let mut tag_percent_calcualation_failures: Vec<ResultStack<Segment>> = Vec::new();
        let mut segment_creation_failures: Vec<ResultStack<Segment>> = Vec::new();
        let mut segments: Vec<Segment> = Vec::new();
        for tag in Tag::get_tags_from(&transactions) {
            // gets the percentage for the tag
            let percentage_result: ResultStack<f64> = Tag::get_tag_percentage(&tag, &transactions);
            let percentage = match percentage_result {
                ResultStack::Pass(p) => p,
                ResultStack::Fail(_) => {
                    tag_percent_calcualation_failures.push(ResultStack::new_fail_from_stack(percentage_result.get_stack()));
                    0.0
                }
            };
            
            // creates a segment for the tag
            let segment_result = Segment::new(tag.clone(), app.bank.tag_registry.get(&tag), percentage as f32, 0.0, 0);
            if segment_result.is_pass() {
                segments.push(segment_result.wont_fail("This is inside an is_pass() block."));
            }
            else {
                segment_creation_failures.push(segment_result);
            }
        }
        segments = Segment::sorted(segments);
        
        // checking if there were any percentage calculation failures
        if !tag_percent_calcualation_failures.is_empty() {
            return ResultStack::new_fail_from_stack(tag_percent_calcualation_failures[0].get_stack()).fail("Failed to assemble rings for RingParse.");
        }
        // checking if there were any Segment creation failures.
        if !segment_creation_failures.is_empty() {
            return ResultStack::new_fail_from_stack(segment_creation_failures[0].get_stack()).fail("Failed to assemble rings for RingParse.");
        }
        // makes sure that there are segments to work with
        if segments.is_empty() {
            return ResultStack::new_fail("No Segments were collected.").fail("Failed to assemble rings for RingParse.");
        }
        
        
        
        // creates a series of rings from the segments
        let mut rings: Vec<Vec<Segment>> = Vec::new();
        // goes through each segment to add it to a ring
        for segment in &segments {
            // checks each ring to see if it fits
            let mut found_a_home = false;
            // adds the segment if the ring is empty
            for ring in &mut rings {
                if ring.is_empty() {
                    ring.push(segment.clone());
                    found_a_home = true;
                    break;
                }
                
                // checks if the segment fits in the ring
                if !found_a_home {
                    if segment.fits_into(ring) {
                        ring.push(segment.clone());
                        found_a_home = true;
                        break;
                    }
                }
            }
            
            // adds the segment to a new ring if no existing ring fits
            if !found_a_home {
                let new_ring = vec![segment.clone()];
                rings.push(new_ring);
            }
        }
        
        // adds spacing to the segments
        for ring in &mut rings {
            let update_offsets_result = Segment::update_offsets_for(ring);
            if update_offsets_result.is_fail() {
                return ResultStack::new_fail_from_stack(update_offsets_result.get_stack()).fail("Failed to assemble rings for RingParse.");
            }
        }
        
        // checks if the segments are safe to display
        for ring in &rings {
            if !Segment::is_safe(ring) {
                return ResultStack::new_fail("Ring precent overflow!").fail("Failed to assemble rings for RingParse.");
            }
        }
        
        // combines the rings into a single Vec with level data for each Segment
        Segment::update_levels_for(&mut rings);
        let segments: Vec<Segment> = rings.into_iter().flat_map(|ring| ring.into_iter()).collect();
        
        // returns the collected segments
        Pass(segments)
    }
    
    
    
    // rendering
    /// Generates all the possible handles for different segments being hovered over.
    /// Instead of re-rendering every time the hovered segment changes, the Ring Parse can simply return the appropriate cached handle.
    pub fn render(&mut self, theme: AppThemes) -> ResultStack<()> {
        // collecting the base information
        let max_size = RingParse::max_size();
        let pixmap_result = ResultStack::from_option(Pixmap::new(max_size, max_size), "Failed to create Pixmap while generating image handle for Segment.");
        if pixmap_result.is_fail() {
            return pixmap_result.empty_type().fail("Failed to render Ring Parse.");
        }
        let mut base_pixmap = pixmap_result.wont_fail("This is past an is_fail() guard clause.");
        
        // collecting the individual hovered segment handles
        let cached_handle_results: Vec<(Option<Tag>, Handle, Vec<ResultStack<()>>)> = self.ring_data.par_iter().map(|hovered_segment| {
            let mut case_pixmap = base_pixmap.clone();
            let mut draw_failures = Vec::new();
            
            for case_segment in &self.ring_data {
                let is_hovered = case_segment == hovered_segment;
                let case_draw_result = case_segment.draw_into(theme, &mut case_pixmap, is_hovered);
                if case_draw_result.is_fail() {
                    draw_failures.push(case_draw_result.empty_type().fail("Failed to render Ring Parse."));
                }
            }
            
            let case_handle = Handle::from_rgba(max_size, max_size, case_pixmap.take());
            (Some(hovered_segment.get_tag().clone()), case_handle, draw_failures)
        }).collect();
        
        // separating handles and failures
        let mut draw_failures: Vec<ResultStack<()>> = Vec::new();
        let mut cached_handles: HashMap<Option<Tag>, Handle> = HashMap::new();
        for (tag, handle, segment_failures) in cached_handle_results {
            draw_failures.extend(segment_failures);
            cached_handles.insert(tag, handle);
        }
        
        // returning if there are any draw failures
        if !draw_failures.is_empty() {
            return draw_failures[0].clone();
        }
        
        // collecting the default handle for when no segment is hovered
        for base_segment in &self.ring_data {
            let case_draw_result = base_segment.draw_into(theme, &mut base_pixmap, false);
            if case_draw_result.is_fail() {
                return case_draw_result.empty_type().fail("Failed to render Ring Parse.");
            }
        }
        
        let case_handle = Handle::from_rgba(max_size, max_size, base_pixmap.take());
        cached_handles.insert(None, case_handle);
        
        // caching the handles
        self.cached_handles = cached_handles;
        Pass(())
    }
    
    /// Returns a copy of the current handle.
    pub fn get_current_handle(&self) -> Handle {
        self.current_handle.clone()
    }
    
    /// Detects which segment is hovered by the given position and updates the hovered segment tag.
    pub fn update_hovering(&mut self, pos: Point, layout_size: Size) -> ResultStack<()> {
        let mut new_hovered_segment_tag: Option<Tag> = None;
        
        for segment in &self.ring_data {
            if segment.contains(pos, layout_size) {
                new_hovered_segment_tag = Some(segment.get_tag().clone());
                break;
            }
        }
        
        if self.hovered_segment_tag != new_hovered_segment_tag {
            self.hovered_segment_tag = new_hovered_segment_tag;
            let new_current_handle_result = ResultStack::from_option(self.cached_handles.get(&self.hovered_segment_tag), "Failed to fetch handle for hovered segment.");
            if new_current_handle_result.is_fail() {
                return new_current_handle_result.empty_type().fail("Failed to update hovering in RingParse.");
            }
            self.current_handle = new_current_handle_result.wont_fail("This is past an is_fail() guard clause.").clone();
        }
        
        Pass(())
    }
    
    /// Stops hovering any segment.
    pub fn stop_hovering(&mut self) -> ResultStack<()> {
        self.hovered_segment_tag = None;
        let new_current_handle_result = ResultStack::from_option(self.cached_handles.get(&None), "Failed to fetch handle for no hovered segment.");
        if new_current_handle_result.is_fail() {
            return new_current_handle_result.empty_type().fail("Failed to update hovering in RingParse.");
        }
        self.current_handle = new_current_handle_result.wont_fail("This is past an is_fail() guard clause.").clone();
        
        Pass(())
    }
}

/// An individual segment of a RingChart representing one tag with all earning or spending transactions.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    /// The tag associated with this segment.
    tag: Tag,
    /// The color of this segment.
    color: MaterialColors,
    /// The percentage of the transactions represented by this segment.
    percentage: f32,
    /// The visual percentage of this segment, accounting for very small/invisible percentages.
    visual_percentage: f32,
    /// The offset percentage of this segment, used for positioning.
    offset_percentage: f32,
    /// The level of this segment, used for which ring it goes into.
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

    /// Asigns levels to segments in a collection of rings..
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

    /// Returns true if the sum of visual percentages of all segments is less than or equal to 1.0.
    pub fn is_safe(segments: &Vec<Segment>) -> bool {
        Segment::sum_visual_percent(segments) <= 1.0
    }



    // rendering
    pub fn contains(&self, point: Point, layout_size: Size) -> bool {
        // point information
        let max_size = RingParse::max_size() as f32;
        let center_x = max_size / 2.0;
        let center_y = center_x;
        let local_scaling = max_size / (layout_size.width.min(layout_size.height)).max(1.0);
        let local_x = point.x * local_scaling;
        let local_y = point.y * local_scaling;
        let center_local_x = local_x - center_x;
        let center_local_y = local_y - center_y;
        let radius = (center_local_x * center_local_x + center_local_y * center_local_y).sqrt();
        let mut angle = center_local_y.atan2(center_local_x);
        if angle < 0.0 { angle += 2.0 * PI; }
        
        // segment information
        let percentage_angle = self.visual_percentage * (2.0 * PI);
        let level_offset = self.level as f32 * (Segment::THICKNESS + Segment::LEVEL_SPACING.size());
        let outer_radius: f32 = (max_size as f32) / 2.0 - level_offset;
        let inner_radius = outer_radius - Segment::THICKNESS;
        let start_angle = self.offset_percentage * (2.0 * PI);
        let end_angle = start_angle + percentage_angle;
        
        // getting the results
        (radius <= outer_radius && radius >= inner_radius) && (angle >= start_angle && angle <= end_angle)
    }
    
    /// Generates an image handle for the segment.
    pub fn draw_into(&self, theme: AppThemes, pixmap: &mut Pixmap, is_hovered: bool) -> ResultStack<()> {
        let mut fill_paint = Paint::default();
        let iced_fill_color = if is_hovered { self.color.themed(&theme, 2) } else { self.color.themed(&theme, 1) };
        let r = (iced_fill_color.r * 255.0) as u8;
        let g = (iced_fill_color.g * 255.0) as u8;
        let b = (iced_fill_color.b * 255.0) as u8;
        let a = (iced_fill_color.a * 255.0) as u8;
        fill_paint.set_color_rgba8(r, g, b, a);
        fill_paint.anti_alias = true;
        
        let mut stroke_paint = Paint::default();
        let iced_stroke_color = self.color.themed(&theme, 2);
        let r = (iced_stroke_color.r * 255.0) as u8;
        let g = (iced_stroke_color.g * 255.0) as u8;
        let b = (iced_stroke_color.b * 255.0) as u8;
        let a = (iced_stroke_color.a * 255.0) as u8;
        stroke_paint.set_color_rgba8(r, g, b, a);
        stroke_paint.anti_alias = true;
        
        let fill_path_result = self.generate_segment_path(false);
        let stroke_path_result = self.generate_segment_path(true);
        if fill_path_result.is_fail() { return ResultStack::new_fail_from_stack(fill_path_result.get_stack()).fail("Failed to generate Segment image handle."); }
        if stroke_path_result.is_fail() { return ResultStack::new_fail_from_stack(stroke_path_result.get_stack()).fail("Failed to generate Segment image handle."); }
        pixmap.fill_path(&fill_path_result.wont_fail("This is past an is_fail() guard clause."), &fill_paint, FillRule::Winding, Transform::identity(), None);
        pixmap.stroke_path(&stroke_path_result.wont_fail("This is past an is_fail() guard clause."), &stroke_paint, &Stroke { width: BorderThickness::Standard.size(), ..Default::default() }, Transform::identity(), None);
        
        // returning
        Pass(())
    }
    
    /// Generates a path for the segment, used for both the shape fill and stroke outline.
    fn generate_segment_path(&self, is_stroke: bool) -> ResultStack<Path> {
        // bounds
        let max_size: u32 = RingParse::max_size();
        let center_x = max_size as f32 / 2.0;
        let center_y = center_x;
        
        // sizing
        let percentage_angle = self.visual_percentage * (2.0 * PI);
        let level_offset = self.level as f32 * (Segment::THICKNESS + Segment::LEVEL_SPACING.size());
        let radius_stroke_modifier = if is_stroke { Segment::border_thickness() / 2.0 } else { 0.0 };
        let outer_radius: f32 = (max_size as f32) / 2.0 - level_offset - radius_stroke_modifier;
        let inner_radius = outer_radius - Segment::THICKNESS + (radius_stroke_modifier * 2.0);
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