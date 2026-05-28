use crate::container::app::App;
use crate::ui::components::{BorderThickness, PaddingSizes, Widths};
use crate::ui::material::{AppThemes, Depths, MaterialColors, Materials};
use crate::vault::bank::{Bank, Filters};
use crate::vault::schrod::Schrod;
use crate::vault::schrod::Schrod::{Pass, Fail};
use crate::vault::transaction::Tag;
use crate::vault::transaction::{Id, Transaction, Value};
use iced::Size;
use iced::widget::image::Handle;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::iso::Currency;
use serde::{Deserialize, Serialize};
use tiny_skia::{FillRule, Paint, Path, PathBuilder, Pixmap, Transform};
use std::collections::HashMap;
use std::f32::consts::PI;
use std::cmp::Ordering;
use std::ops::Add;
use iced::Point;

/// Provides enumerated options for the directions money can flow (in/out of your account).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowDirections {
    Earning,
    Spending,
}
impl FlowDirections {
    /// Returns `true` if the given `Transaction` value matches the `FlowDirection`.
    #[must_use]
    pub fn matches(&self, transaction: &Transaction) -> bool {
        match self {
            FlowDirections::Earning => transaction.value.is_positive(),
            FlowDirections::Spending => transaction.value.is_negative(),
        }
    }
}

/// The different ways a `CashFlow` can be displayed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlowTypes {
    /// Collected in separate `Currency` groups.
    Collected,
    /// Unified into a single `Currency` value.
    Unified,
    /// As a time price.
    Time,
}



/// Holds cash flow values for multiple `Currency`s from a single list of `Transaction`s.
#[derive(Debug, Clone, PartialEq)]
pub struct CashFlow {
    /// The list of `Value`s grouped by `Currency`.
    value_flows: Vec<Value>,
    /// The unified value of all `Currency`s in the `value_flows`.
    unified_value_flow: Value,
    /// The overall cash flow represented as a time price.
    time_flow: f64,
}
impl CashFlow {
    /// Creates a new `CashFlow` from a list of `Transaction` `Id`s.
    #[must_use]
    pub fn new(bank: &Bank, transaction_ids: &[Id], time_price: f64) -> Schrod<CashFlow> {
        // value flows
        let value_flows_result = CashFlow::get_value_flows(bank, transaction_ids.to_owned());
        if value_flows_result.is_fail() {
            return value_flows_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let value_flows = value_flows_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        // unified value
        let unified_value_flow_result = CashFlow::get_unified_value_flow(bank, &value_flows);
        if unified_value_flow_result.is_fail() {
            return unified_value_flow_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let unified_value_flow = unified_value_flow_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        // time flow
        let time_flow_result = CashFlow::get_time_flow(&value_flows, time_price);
        if time_flow_result.is_fail() {
            return time_flow_result
                .convert("CashFlow::new()")
                .fail("Failed to create Cash Flow.", "CashFlow::new()");
        }
        let time_flow = time_flow_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::new()");

        Pass(CashFlow {
            value_flows,
            unified_value_flow,
            time_flow,
        })
    }

    /// Gets the `String` representation of the `CashFlow` based on the given flow type.
    #[must_use]
    pub fn display(&self, flow_type: FlowTypes) -> Vec<String> {
        match flow_type {
            FlowTypes::Collected => {
                self.value_flows.iter().map(|vf| format!("{} {}", vf, vf.currency())).collect()
            }
            FlowTypes::Unified => {
                vec![self.unified_value_flow.to_string()]
            }
            FlowTypes::Time => {
                vec![format!("{:.2} hrs", self.time_flow)]
            }
        }
    }

    /// Returns the collected value flows of the `CashFlow` as a `Vec<Value>`.
    #[must_use]
    pub fn collected(&self) -> Vec<Value> {
        self.value_flows.clone()
    }

    /// Returns the unified value flow of the `CashFlow` as a `Value`.
    #[must_use]
    pub fn unified(&self) -> Value {
        self.unified_value_flow.clone()
    }

    /// Returns the time flow of the `CashFlow` as an `f64`.
    #[must_use]
    pub fn time(&self) -> f64 {
        self.time_flow
    }

    /// Turns a list of `Transaction`s into a collection of `Value`s, grouped by `Currency`,
    /// that each represent the overall cash flow for the given `Currency`.
    #[must_use]
    fn get_value_flows(bank: &Bank, transaction_ids: Vec<Id>) -> Schrod<Vec<Value>> {
        // the list of all the transactions (by id) grouped by their currencies
        let mut coupled_value_groups: Vec<(Currency, Vec<Id>)> = Vec::new();

        // collects each transaction value into separate value groups
        for id in transaction_ids {
            // the current transaction
            let transaction_result = bank.get(id);
            if transaction_result.is_fail() {
                return transaction_result
                    .convert("CashFlow::get_value_flows()")
                    .fail("Failed to get value flows.", "CashFlow::get_value_flows()")
            }
            let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
            // checks if the currency has been used already
            let mut is_currency_used = false;

            // adds the transaction to the current group if their currencies are the same
            for group in &mut coupled_value_groups {
                if *transaction.value.currency() == group.0 {
                    is_currency_used = true;
                    group.1.push(id);
                    break;
                }
            }

            // creates a new group if the currency has not been used yet
            if !is_currency_used { coupled_value_groups.push((*transaction.value.currency(), vec![id])); }
        }

        // collects the coupled cash flow groups into individual values
        let value_flow_results: Vec<Schrod<Value>> = coupled_value_groups.into_iter().map(|couple| {
            // tracks the flow of each couple
            let mut flow: Decimal = Decimal::ZERO;
            // adds the transaction value to the flow
            for id in &couple.1 {
                let transaction_result = bank.get(*id);
                if transaction_result.is_fail() {
                    return transaction_result
                        .convert("CashFlow::get_value_flows()")
                        .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
                }
                let transaction = transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
                let value_amount = transaction.value.amount();
                flow = flow.add(value_amount);
            }
            
            // gets the currency from the first transaction in the couple
            let first_transaction_result = bank.get(couple.1[0]);
            if first_transaction_result.is_fail() {
                return first_transaction_result
                    .convert("CashFlow::get_value_flows()")
                    .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
            }
            let last_transaction = first_transaction_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_value_flows()");
            let currency = last_transaction.value.currency();
            Pass(Value::from_decimal(flow, currency))
        }).collect();

        // returns early if any value flow results are fails
        if Schrod::contains_fail(&value_flow_results) {
            return Schrod::collect_and_fail(&value_flow_results, "CashFlow::get_value_flows()")
                .convert("CashFlow::get_value_flows()")
                .fail("Failed to get value flows.", "CashFlow::get_value_flows()");
        }

        // takes the inernal values out of the results
        let value_flows: Vec<Value> = value_flow_results
            .into_iter()
            .map(|vf| vf.wont_fail("This is past a contains_fail() guard clause.", "CashFlow::get_value_flows()"))
            .collect();
        
        // returns the cash flow groups
        Pass(value_flows)
    }

    /// Returns all value flows combined into the same `Currency` based on the `main_currency` in the `CurrencyExchange`.
    #[must_use]
    fn get_unified_value_flow(bank: &Bank, value_flows: &[Value]) -> Schrod<Value> {
        let new_value_results: Vec<_> = value_flows
            .iter()
            .map(|flow| bank.currency_exchange.convert(flow.amount(), flow.currency(), &bank.currency_exchange.get_main_currency()))
            .collect();

        if Schrod::contains_fail(&new_value_results) {
            return Schrod::collect_and_fail(&new_value_results, "CashFlow::unified()")
                    .convert("CashFlow::unified()")
                    .fail("Failed to unify values!", "CashFlow::unified()")
        }

        let new_values: Vec<_> = new_value_results
            .into_iter()
            .map(|r| r.wont_fail("This is past a contains_fail() guard clause.", "CashFlow::unified()"))
            .collect();
        let mut unified_value = Decimal::from(0);
        for value in new_values { unified_value += value; }

        Pass(Value::from_decimal(unified_value, bank.currency_exchange.get_main_currency()))
    }

    /// Gets the overall time flow value from a list of `Value`s.
    #[must_use]
    fn get_time_flow(value_flows: &[Value], time_price: f64) -> Schrod<f64> {
        if time_price <= 0.0 { return Schrod::new_fail("Time price must be greater than 0!", "CashFlow::get_time_flow()").fail("Failed to get time flow.", "CashFlow::get_time_flow()"); }
        let mut time_flow = 0.0;
        for value_flow in value_flows {
            let f64_flow_result = Schrod::from_option(value_flow.amount().to_f64(), "Failed to convert Decimal to f64!", "CashFlow::get_time_flow()");
            if f64_flow_result.is_fail() { return f64_flow_result.fail("Failed to get time flow.", "CashFlow::get_time_flow()") }
            time_flow += f64_flow_result.wont_fail("This is past an is_fail() guard clause.", "CashFlow::get_time_flow()") / time_price; // todo: account for currency
        }
        Pass(time_flow)
    }
}



/// Holds the data that the `RingChart` displays.
#[derive(Debug, Clone, PartialEq)]
pub struct RingParse {
    ring_data: Vec<Segment>,
    hovered_segment_tag: Option<Tag>,
    cached_handles: HashMap<Option<Tag>, Handle>,
    current_handle: Handle,
}

impl RingParse {
    // constants
    /// The maximum size of the `RingChart`, based on the width of the `transaction_management_panel`.
    #[must_use]
    pub fn max_size() -> u32 {
        let home_panel_width = Widths::SmallCard.size();
        let home_panel_internal_padding = PaddingSizes::Small.size();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // this will always turn out to be a positive value
        let size = (home_panel_width - (2.0 * home_panel_internal_padding)) as u32;
        size
    }
    
    
    
    // data retrieval
    /// Gets the `hovered_tag`.
    #[must_use]
    pub fn get_hovered_tag(&self) -> Option<Tag> {
        self.hovered_segment_tag.clone()
    }
    
    /// Gets the `Segment` for the given `Tag`.
    #[must_use]
    pub fn get_segment(&self, tag: &Tag) -> Schrod<&Segment> {
        for segment in &self.ring_data {
            if segment.tag == *tag {
                return Schrod::Pass(segment);
            }
        }
        Schrod::new_fail(&format!("Could not get Segment for tag {} in Ring Parse.", tag.get_label()), "RingParse::get_segment()")
    }
    
    /// Returns a copy of the current handle.
    #[must_use]
    pub fn get_current_handle(&self) -> Handle {
        self.current_handle.clone()
    }
    
    
    
    // assembling
    /// Creates a new `RingParse`.
    #[must_use]
    pub fn new(app: &App, bank: &Bank, filter: Filters, flow_direction: FlowDirections) -> Schrod<RingParse> {
        let max_size = RingParse::max_size();
        let ring_data_result = RingParse::assemble(app, bank, filter, flow_direction);
        let empty_pixmap_result = Schrod::from_option(Pixmap::new(max_size, max_size), "Failed to create empty Pixmap for RingParse.", "RingParse::new()");
        if empty_pixmap_result.is_fail() {
            return empty_pixmap_result
                .convert("RingParse::new()")
                .fail("Failed to create RingParse.", "RingParse::new()");
        }
        let empty_pixmap = empty_pixmap_result.wont_fail("This is past an is_fail() guard clause.", "RingParse::new()");
        
        match ring_data_result {
            Pass(ring_data) => Pass(
                RingParse {
                    ring_data,
                    hovered_segment_tag: None,
                    cached_handles: HashMap::new(),
                    current_handle: Handle::from_rgba(max_size, max_size, empty_pixmap.take()),
                }
            ),
            Fail(_) => {
                ring_data_result
                    .convert("RingParse::new()")
                    .fail("Failed to create RingParse.", "RingParse::new()")
            }
        }
    }
    
    /// Gets the `ring_data`.
    #[must_use]
    pub fn get_ring_data(&self) -> Vec<Segment> {
        self.ring_data.clone()
    }
    
    /// Assmebles rings of `Segment`s for a `RingParse`.
    #[must_use]
    fn assemble(app: &App, bank: &Bank, filter: Filters, flow_direction: FlowDirections) -> Schrod<Vec<Segment>> {
        // gets the transactions by id and fails if any of them could not be retrieved
        let transaction_results = bank.get_filtered_ids(filter)
            .into_iter()
            .map(|id| bank.get(id))
            .collect::<Vec<Schrod<&Transaction>>>();
        if Schrod::contains_fail(&transaction_results) {
            return Schrod::collect_and_fail(&transaction_results, "RingParse::assemble()")
                .convert("RingParse::assemble()")
                .fail("Failed to assemble rings for RingParse.", "RingParse::assemble()");
        }
        let mut transactions = transaction_results.into_iter().map(|r| r.wont_fail("This is past a contains_fail() block.", "RingParse::assemble()")).collect::<Vec<&Transaction>>();
        
        // filters out transactions that do not match the flow direction
        transactions.retain(|t| { flow_direction.matches(t) });
        
        
        
        // assembles a list of segments from the tags
        let segment_results: Vec<_> = Tag::get_tags_from(&transactions).into_iter().map(|tag| {
            // gets the percentage for the tag
            let percentage_result: Schrod<f64> = Tag::get_tag_percentage(&tag, &transactions);
            if percentage_result.is_fail() { percentage_result.convert("RingParse::assemble()") }
            
            // creates a segment for the tag
            else {
                let percentage = percentage_result.wont_fail("This is past an is_fail() guard clause.", "RingParse::assemble()");
                #[allow(clippy::cast_possible_truncation)] // percentage will always be a small number
                let segment_result = Segment::new(tag.clone(), app.bank.tag_registry.get(&tag), percentage as f32, 0.0, 0);
                segment_result
            }
        }).collect();

        // returns early if there was a failure
        if Schrod::contains_fail(&segment_results) {
            return Schrod::collect_and_fail(&segment_results, "RingParse::assemble()")
                .convert("RingParse::assemble()")
                .fail("Failed to assemble rings for RingParse.", "RingParse::assemble()");
        }

        // converts the segment results into a sorted list of segments
        let mut segments: Vec<_> = segment_results.into_iter().map(|result| result.wont_fail("This is past a contains_fail() guard clause.", "RingParse::assemble()")).collect();
        segments = Segment::sorted(&segments);
        
        
        
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
                if !found_a_home && segment.fits_into(ring) {
                    ring.push(segment.clone());
                    found_a_home = true;
                    break;
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
                return update_offsets_result
                    .convert("RingParse::assemble()")
                    .fail("Failed to assemble rings for RingParse.", "RingParse::assemble()")
            }
        }
        
        // checks if the segments are safe to display
        for ring in &rings {
            if !Segment::is_safe(ring) {
                return Schrod::new_fail("Ring precent overflow!", "RingParse::assemble()")
                    .fail("Failed to assemble rings for RingParse.", "RingParse::assemble()");
            }
        }
        
        // combines the rings into a single Vec with level data for each Segment
        Segment::update_levels_for(&mut rings);
        let segments: Vec<Segment> = rings.into_iter().flat_map(IntoIterator::into_iter).collect();
        
        // returns the collected segments
        Pass(segments)
    }
    
    
    
    // rendering
    /// Generates all the possible `Handle`s for different `Segment`s being hovered over.
    /// Instead of re-rendering every time the hovered `Segment` changes, the `RingParse` can simply return the appropriate cached `Handle`.
    #[must_use]
    pub fn render(&mut self, theme: AppThemes) -> Schrod<()> {
        // collecting the base information
        let max_size = RingParse::max_size();
        let pixmap_result = Schrod::from_option(Pixmap::new(max_size, max_size), "Failed to create Pixmap while generating image handle for Segment.", "RingParse::render()");
        if pixmap_result.is_fail() {
            return pixmap_result
                .convert("RingParse::render()")
                .fail("Failed to render Ring Parse.", "RingParse::render()");
        }
        let mut base_pixmap = pixmap_result.wont_fail("This is past an is_fail() guard clause.", "RingParse::render()");
        let background = MaterialColors::Card.materialized(Materials::Plastic, Depths::Flat, false, theme);
        base_pixmap.fill(tiny_skia::Color::from_rgba(background.r, background.g, background.b, background.a).unwrap_or(tiny_skia::Color::TRANSPARENT));
        
        // collecting the individual hovered segment handles
        let cached_handle_results: Vec<(Option<Tag>, Handle, Vec<Schrod<()>>)> = self.ring_data.par_iter().map(|hovered_segment| {
            let mut case_pixmap = base_pixmap.clone();
            let mut draw_failures = Vec::new();
            
            for case_segment in &self.ring_data {
                let is_hovered = case_segment == hovered_segment;
                let case_draw_result = case_segment.draw_into(theme, &mut case_pixmap, is_hovered);
                if case_draw_result.is_fail() {
                    draw_failures.push(case_draw_result.convert("RingParse::render()").fail("Failed to render Ring Parse.", "RingParse::render()"));
                }
            }
            
            let case_handle = Handle::from_rgba(max_size, max_size, case_pixmap.take());
            (Some(hovered_segment.get_tag().clone()), case_handle, draw_failures)
        }).collect();
        
        // separating handles and failures
        let mut draw_failures: Vec<Schrod<()>> = Vec::new();
        let mut cached_handles: HashMap<Option<Tag>, Handle> = HashMap::new();
        for (tag, handle, segment_failures) in cached_handle_results {
            draw_failures.extend(segment_failures);
            cached_handles.insert(tag, handle);
        }
        
        // returning if there are any draw failures
        if Schrod::contains_fail(&draw_failures) {
            return Schrod::collect_and_fail(&draw_failures, "RingParse::render()")
                .fail("Failed to render Ring Parse.", "RingParse::render()")
        }
        
        // collecting the default handle for when no segment is hovered
        for base_segment in &self.ring_data {
            let case_draw_result = base_segment.draw_into(theme, &mut base_pixmap, false);
            if case_draw_result.is_fail() {
                return case_draw_result
                    .convert("RingParse::render()")
                    .fail("Failed to render Ring Parse.", "RingParse::render()")
            }
        }
        
        let case_handle = Handle::from_rgba(max_size, max_size, base_pixmap.take());
        cached_handles.insert(None, case_handle);
        
        // caching the handles
        self.cached_handles = cached_handles;
        Pass(())
    }
    
    /// Same as `render()`, but returns a new `RingParse` that has been rendered internally instead of rendering in place.
    #[must_use]
    pub async fn get_rendered(ring_parse: RingParse, theme: AppThemes) -> (Schrod<RingParse>, Schrod<()>) {
        let mut rendered_ring_parse = ring_parse;
        let render_result = rendered_ring_parse.render(theme).await;
        let stop_hovering_result = rendered_ring_parse.stop_hovering();
        
        if render_result.is_fail() { return (Pass(rendered_ring_parse), render_result); }
        if stop_hovering_result.is_fail() { return (Pass(rendered_ring_parse), stop_hovering_result); }
        
        (Pass(rendered_ring_parse), Pass(()))
    }
    
    /// Detects which `Segment` is hovered by the given position and updates the hovered segment `Tag`.
    #[must_use]
    pub fn update_hovering(&mut self, pos: Point, layout_size: Size) -> Schrod<()> {
        let mut new_hovered_segment_tag: Option<Tag> = None;
        
        for segment in &self.ring_data {
            if segment.contains(pos, layout_size) {
                new_hovered_segment_tag = Some(segment.get_tag().clone());
                break;
            }
        }
        
        if self.hovered_segment_tag != new_hovered_segment_tag {
            self.hovered_segment_tag = new_hovered_segment_tag;
            let new_current_handle_result = Schrod::from_option(self.cached_handles.get(&self.hovered_segment_tag), "Failed to fetch handle for hovered segment.", "RingParse::update_hovering()");
            if new_current_handle_result.is_fail() {
                return new_current_handle_result
                    .convert("RingParse::update_hovering()")
                    .fail("Failed to update hovering in RingParse.", "RingParse::update_hovering()")
            }
            self.current_handle = new_current_handle_result.wont_fail("This is past an is_fail() guard clause.", "RingParse::update_hovering()").clone();
        }
        
        Pass(())
    }
    
    /// Stops hovering any `Segment`.
    #[must_use]
    pub fn stop_hovering(&mut self) -> Schrod<()> {
        self.hovered_segment_tag = None;
        let new_current_handle_result = Schrod::from_option(self.cached_handles.get(&None), "Failed to fetch handle for no hovered segment.", "RingParse::stop_hovering()");
        if new_current_handle_result.is_fail() {
            return new_current_handle_result
                .convert("RingParse::stop_hovering()")
                .fail("Failed to update hovering in RingParse.", "RingParse::stop_hovering()")
        }
        self.current_handle = new_current_handle_result.wont_fail("This is past an is_fail() guard clause.", "RingParse::stop_hovering()").clone();
        
        Pass(())
    }
}

/// An individual segment of a `RingChart` representing one `Tag` with all earning or spending `Transaction`s.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    /// The `Tag` associated with this `Segment`.
    tag: Tag,
    /// The color of this `Segment`.
    color: MaterialColors,
    /// The percentage of the transactions represented by this `Segment`.
    percentage: f32,
    /// The visual percentage of this `Segment`, accounting for very small/invisible percentages.
    visual_percentage: f32,
    /// The offset percentage of this `Segment`, used for positioning.
    offset_percentage: f32,
    /// The level of this `Segment`, used for which ring it goes into.
    level: usize,
}
impl Segment {
    // constants
    /// Defines the minimum `visual_percentage` for a `Segment`.
    const MINIMUM_VISUAL_PERCENTAGE: f32 = 0.01;
    /// The thickness of the ring.
    const THICKNESS: f32 = 20.0;
    /// Defines the spacing between `Segment`s in percentage.
    const SPACING: f32 = 0.0075;
    /// The spacing between `level`s of rings.
    #[must_use]
    fn level_sapcing() -> f32 {
        PaddingSizes::Small.size()
    }
    /// The border thickness of the `Segment`.
    #[must_use]
    fn border_thickness() -> f32 {
        BorderThickness::Thin.size() / 2.0
    }
    
    
    
    // basic getters
    /// Gets the `Tag`.
    #[must_use]
    pub fn get_tag(&self) -> Tag {
        self.tag.clone()
    }

    /// Gets the `color`.
    #[must_use]
    pub fn get_color(&self) -> MaterialColors {
        self.color
    }

    /// Gets the `percentage`.
    #[must_use]
    pub fn get_percentage(&self) -> f32 {
        self.percentage
    }

    /// Gets the `visual_percentage`.
    #[must_use]
    pub fn get_visual_percentage(&self) -> f32 {
        self.visual_percentage
    }

    /// Gets the `offset_percentage`.
    #[must_use]
    pub fn get_offset_percentage(&self) -> f32 {
        self.offset_percentage
    }

    /// Gets the `level`.
    #[must_use]
    pub fn get_level(&self) -> usize {
        self.level
    }



    // segment work
    /// Returns a new `Segment`.
    #[must_use]
    pub fn new(tag: Tag, color: MaterialColors, percentage: f32, offset_percentage: f32, level: usize) -> Schrod<Segment> {
        let visual_percentage = percentage.max(Self::MINIMUM_VISUAL_PERCENTAGE);
        if percentage <= 0.0 || percentage > 1.0 {
            return Schrod::new_fail(&format!("Segment percentage must be greater than 0.0 and less than or equal to 1.0! Percentage was {percentage:.3}."), "Segment::new()")
                .fail("Failed to create Segment.", "Segment::new()")
        }
        if !(0.0..1.0).contains(&offset_percentage) {
            return Schrod::new_fail(&format!("Segment offset must be between 0.0 and 1.0! Offset was {offset_percentage:.3}."), "Segment::new()")
                .fail("Failed to create Segment.", "Segment::new()")
        }

        Pass(Segment { tag, color, percentage, visual_percentage, offset_percentage, level })
    }

    /// Updates the offsets in a list of `Segment`s.
    #[must_use]
    pub fn update_offsets_for(ring: &mut [Segment]) -> Schrod<()> {
        for i in 0..ring.len() {
            let used_space_result = Segment::get_visual_percentage_before_position(ring, i);
            match used_space_result {
                Pass(used_space) => {
                    ring[i].offset_percentage = used_space;
                }
                Fail(_) => {
                    return used_space_result
                        .convert("Segment::update_offsets_for()")
                        .fail("Failed to update offsets in ring.", "Segment::update_offsets_for()")
                }
            }
        }
        Pass(())
    }

    /// Gets the visual percentage (with offsets) of all the `Segment`s before the segment at the given position (index) in a ring.
    #[must_use]
    fn get_visual_percentage_before_position(ring: &[Segment], position: usize) -> Schrod<f32> {
        if position >= ring.len() {
            return Schrod::new_fail(&format!("Position/index out of bounds! Position was {}. out of {} max position", position, ring.len() - 1), "Segment::get_visual_percentage_before_position()")
                .fail("Failed to get visual percentage up to position in a ring.", "Segment::get_visual_percentage_before_position()");
        }

        if ring.is_empty() { return Pass(0.0) }

        let mut sum_visual_percentage = 0.0;
        for segment in ring.iter().take(position) {
            sum_visual_percentage += segment.visual_percentage + Segment::SPACING;
        }

        Pass(sum_visual_percentage)
    }

    /// Asigns levels to `Segment`s in a collection of rings.
    pub fn update_levels_for(rings: &mut [Vec<Segment>]) {
        for (level, ring) in rings.iter_mut().enumerate() {
            for segment in ring {
                segment.level = level;
            }
        }
    }

    /// Returns the sum percent of all the `Segment`s in a given list.
    #[must_use]
    pub fn sum_visual_percent(segments: &[Segment]) -> f32 {
        let mut sum_visual_percentage = segments.iter().map(|s| s.visual_percentage).sum();
        #[allow(clippy::cast_precision_loss)] // the length of segments will always be small
        let percentage = if segments.len() == 1 { Segment::SPACING * (segments.len() as f32 - 1.0) } else { Segment::SPACING * (segments.len() as f32) };
        sum_visual_percentage += percentage;
        sum_visual_percentage
    }

    /// Checks if a given `Segment` fits into a collection of `Segment`s.
    #[must_use]
    pub fn fits_into(&self, segments: &[Segment]) -> bool {
        let used_space = Segment::sum_visual_percent(segments);
        1.0 - used_space >= self.visual_percentage + Segment::SPACING
    }

    /// Returns a sorted copy of the `Segment`s by percentage.
    #[must_use]
    pub fn sorted(segments: &[Segment]) -> Vec<Segment> {
        let mut sorted_segments = segments.to_vec();
        sorted_segments.sort_by(|a, b| a.percentage.partial_cmp(&b.percentage).unwrap_or(Ordering::Equal));
        sorted_segments
    }

    /// Returns `true` if the sum of visual percentages of all `Segment`s is less than or equal to 1.0.
    #[must_use]
    pub fn is_safe(segments: &[Segment]) -> bool {
        Segment::sum_visual_percent(segments) <= 1.0
    }



    // rendering
    /// Returns `true` if the point is within the `Segment`'s bounds.
    #[must_use]
    pub fn contains(&self, point: Point, layout_size: Size) -> bool {
        // point information
        #[allow(clippy::cast_precision_loss)] // max_size will always be small
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
        #[allow(clippy::cast_precision_loss)] // self.level will always be small
        let level_offset = self.level as f32 * (Segment::THICKNESS + Segment::level_sapcing());
        let outer_radius: f32 = (max_size) / 2.0 - level_offset;
        let inner_radius = outer_radius - Segment::THICKNESS;
        let start_angle = self.offset_percentage * (2.0 * PI);
        let end_angle = start_angle + percentage_angle;
        
        // getting the results
        (radius <= outer_radius && radius >= inner_radius) && (angle >= start_angle && angle <= end_angle)
    }
    
    /// Generates an image handle for the `Segment`.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)] // color values will always be small and positive
    #[must_use]
    pub fn draw_into(&self, theme: AppThemes, pixmap: &mut Pixmap, is_hovered: bool) -> Schrod<()> {
        let mut fill_paint = Paint::default();
        let iced_fill_color = if is_hovered { MaterialColors::accent(theme).materialized(Materials::Plastic, Depths::Proud, false, theme) } else { self.color.materialized(Materials::Plastic, Depths::Proud, false, theme) };
        let r = (iced_fill_color.r * 255.0) as u8;
        let g = (iced_fill_color.g * 255.0) as u8;
        let b = (iced_fill_color.b * 255.0) as u8;
        let a = (iced_fill_color.a * 255.0) as u8;
        fill_paint.set_color_rgba8(r, g, b, a);
        fill_paint.anti_alias = true;
        
        let fill_path_result = self.generate_segment_path(false);
        if fill_path_result.is_fail() {
            return fill_path_result
                .convert("Segment::draw_into()")
                .fail("Failed to generate Segment image handle.", "Segment::draw_into()")
        }
        pixmap.fill_path(&fill_path_result.wont_fail("This is past an is_fail() guard clause.", "Segment::draw_into()"), &fill_paint, FillRule::Winding, Transform::identity(), None);
        
        // returning
        Pass(())
    }
    
    /// Generates a `Path` for the `Segment`, used for both the shape fill and stroke outline.
    #[must_use]
    fn generate_segment_path(&self, is_stroke: bool) -> Schrod<Path> {
        // bounds
        let max_size: u32 = RingParse::max_size();
        #[allow(clippy::cast_precision_loss)] // max_size will always be small
        let center_x = max_size as f32 / 2.0;
        let center_y = center_x;
        
        // sizing
        let percentage_angle = self.visual_percentage * (2.0 * PI);
        #[allow(clippy::cast_precision_loss)] // self.level will always be small
        let level_offset = self.level as f32 * (Segment::THICKNESS + Segment::level_sapcing());
        let radius_stroke_modifier = if is_stroke { Segment::border_thickness() / 2.0 } else { 0.0 };
        #[allow(clippy::cast_precision_loss)] // max_size will always be small
        let outer_radius: f32 = (max_size as f32) / 2.0 - level_offset - radius_stroke_modifier;
        let inner_radius = outer_radius - Segment::THICKNESS + (radius_stroke_modifier * 2.0);
        let start_angle = self.offset_percentage * (2.0 * PI);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // outer_radius will always be small and positive
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
            #[allow(clippy::cast_precision_loss)] // i and steps will always be small
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
            #[allow(clippy::cast_precision_loss)] // i and steps will always be small
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

        Schrod::from_option(path.finish(), "Failed to draw segment geometry.", "Segment::generate_segment_path()")
    }
}