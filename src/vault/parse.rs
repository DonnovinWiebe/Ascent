use crate::container::app::App;
use crate::container::signal::Signal;
use crate::container::signal::Signal::*;
use crate::ui::components::{PaddingSizes, TextSizes, Widths, ui_string};
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
use iced::widget::canvas::{self, Frame, Path};
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
use tiny_skia::Pixmap;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::cmp::Ordering;
use iced::Point;
use crate::ui::charting::*;

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