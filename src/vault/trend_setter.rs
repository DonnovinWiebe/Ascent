use std::cell::RefCell;

use crate::{container::app::App, ui::{components::{Heights, PaddingSizes, Widths}, material::{Depths, MaterialColors, Materials}}, vault::{bank::Bank, parse::CashFlow, result_stack::ResultStack, transaction::{Date, Months, Tag, Transaction}}};
use crate::vault::result_stack::ResultStack::{Pass, Fail};
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea, element::PathElement, series::LineSeries, style::{IntoFont, ShapeStyle}};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use iced::widget::image::Handle;
use iced::widget::image;
use plotters_bitmap::BitMapBackend;
use plotters_bitmap::bitmap_pixel::RGBPixel;

/// Defines how groups of `Transaction`s can be split by time intervals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intervals {
    /// Groups by week.
    Weekly,
    /// Groups by two-week couples.
    BiWeekly,
    /// Groups by month.
    Monthly,
    /// Groups by quarter.
    Quarterly,
    /// Groups by year.
    Yearly,
}



/// Holds data for a graphical representation of `CashFlow`s by `Tag` over time.
pub struct TrendParse {
    /// A list of individual `CashFlow`s over time grouped by `Tag`.
    time_lines: Vec<TimeLine>,
    /// The interval between `CashFlow`s.
    interval: Intervals
}
impl TrendParse {
    // constants
    /// The maximum size of the `TrendParse`, based on the width of the `trends_panel`.
    #[must_use]
    pub fn max_size() -> (u32, u32) {
        let home_panel_width = Widths::LargeCard.size();
        let home_panel_height = Heights::LargeCard.size();
        let home_panel_internal_padding = PaddingSizes::Small.size();
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // this will always turn out to be a positive value
        let width = (home_panel_width - (2.0 * home_panel_internal_padding)) as u32;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // this will always turn out to be a positive value
        let height = (home_panel_height - (2.0 * home_panel_internal_padding)) as u32;
        (width, height)
    }



    // data retrieval



    // assembling
    /// Creates a new `TrendParse`.
    #[must_use]
    pub fn new(bank: &Bank, transactions: &Vec<Transaction>, show_overall_cash_flow: bool, tags: Vec<Tag>, interval: Intervals, last_date: Date, length: usize) -> ResultStack<TrendParse> {
        // the list of time lines
        let mut time_line_results = Vec::new();

        // adding a time line for the overall cash flow
        if show_overall_cash_flow { time_line_results.push(TimeLine::new(bank, transactions, None, interval, last_date, length)) }

        // adding time lines for each tag
        for tag in tags { time_line_results.push(TimeLine::new(bank, transactions, Some(tag), interval, last_date, length)) }

        // filters out any failures
        let mut failures = Vec::new();
        let mut time_lines = Vec::new();
        for result in time_line_results {
            match result {
                Pass(time_line) => time_lines.push(time_line),
                Fail(_) => failures.push(result),
            }
        }

        // returns a fail if there were any failures
        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to create TrendParse.") }

        // returns the trend parse
        Pass(TrendParse { time_lines, interval })
    }

    /// Gets the highest and lowest `CashFlow` values (currency unified).
    #[must_use]
    fn get_flow_range(&self, bank: &Bank) -> ResultStack<(Decimal, Decimal)> {
        let mut lowest_flow: Option<Decimal> = None;
        let mut highest_flow: Option<Decimal> = None;
        let mut failures = Vec::new();
        
        for time_line in &self.time_lines {
            for time_stamp in &time_line.time_stamps {
                let unified_flow_result = time_stamp.cash_flow.unified(bank);
                match unified_flow_result {
                    Pass(value) => {
                        match lowest_flow {
                            Some(lowest) => if value < lowest { lowest_flow = Some(value); },
                            None => lowest_flow = Some(value),
                        }
                        match highest_flow {
                            Some(highest) => if value > highest { highest_flow = Some(value); },
                            None => highest_flow = Some(value),
                        }
                    }
                    
                    Fail(_) => failures.push(unified_flow_result),
                }
            }
        }

        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to get flow range!"); }

        if let Some(lowest) = lowest_flow && let Some(highest) = highest_flow {
            Pass((lowest, highest))
        }

        else { ResultStack::new_fail("Unknown failure.").fail("Failed to get flow range!") }
    }
    
    /// Returns rendering data: one entry per TimeLine — (series label, points).
    #[must_use]
    fn get_plot_data(&self, bank: &Bank) -> ResultStack<Vec<(String, Vec<(f64, f64)>)>> {
        let plot_data_results: Vec<_> = self.time_lines.iter().map(|tl| tl.get_plot_data(bank)).collect();
        
        let mut failures = Vec::new();
        for result in &plot_data_results { if result.is_fail() { failures.push(result) } }
        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to get plot data.") }

        let plot_data: Vec<_> = plot_data_results.into_iter().map(|pd| pd.wont_fail("This is past an is_fail() guard clause.")).collect();
        Pass(plot_data)
    }

    /// Generates a chart `Handle` for the given `TrendParse`.
    pub fn render(&self, app: &App) -> ResultStack<Handle> {
        // holds the image data
        let size = TrendParse::max_size();
        let mut buffer = vec![0u8; (size.0 * size.1 * 3) as usize];

        // colors
        let background_color = MaterialColors::color_as_rgb(MaterialColors::Card.materialized(
            Materials::Plastic,
            Depths::Flat,
            false,
            app.theme_selection,
        ));
        let grid_color = MaterialColors::color_as_rgb(MaterialColors::CardContent.materialized(
            Materials::Plastic,
            Depths::Flat,
            true,
            app.theme_selection,
        ));
        let text_color = MaterialColors::color_as_rgb(MaterialColors::StrongText.materialized(
            Materials::Plastic,
            Depths::Flat,
            false,
            app.theme_selection,
        ));

        // the base chart
        let base_result = ResultStack::from_result(BitMapBackend::<RGBPixel>::with_buffer_and_format(&mut buffer, (size.0, size.1)), "Failed to create BitMapBackend!");
        if base_result.is_fail() { return ResultStack::new_fail_from_stack(base_result.get_stack()).fail("Failed to render TrendParse.") }
        let base = base_result.wont_fail("This is past an is_fail() guard clause.");
        let base = base.into_drawing_area();

        // fills the background of the base chart
        let fill_result = ResultStack::from_result(base.fill(&background_color), "Failed to fill background of chart.");
        if fill_result.is_fail() { return ResultStack::new_fail_from_stack(fill_result.get_stack()).fail("Failed to render TrendParse.") }

        // gets the plot data
        let plot_data_result = self.get_plot_data(&app.bank);
        if plot_data_result.is_fail() { return ResultStack::new_fail_from_stack(plot_data_result.get_stack()).fail("Failed to render TrendParse.") }
        let plot_data = plot_data_result.wont_fail("This is past an is_fail() guard clause.");

        // presents the chart
        // without plot data
        if plot_data.is_empty() {
            let presented_base_result = ResultStack::from_result(base.present(), "Failed to present chart without data.");
            if presented_base_result.is_fail() { return ResultStack::new_fail_from_stack(presented_base_result.get_stack()).fail("Failed to render TrendParse.") }
        }
        // with plot data
        else {
            // get the data bounds
            let all_y: Vec<f64> = plot_data
                .iter()
                .flat_map(|(_, points)| points.iter().map(|&(_, y)| y))
                .collect();
            let smallest_y = all_y.iter().cloned().fold(f64::INFINITY, f64::min);
            let largest_y = all_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let y_padding = (largest_y - smallest_y).abs() * 0.1 + 1.0;
            let length = plot_data[0].1.len().saturating_sub(1) as f64;

            // starts building the chart
            let mut chart_result = ResultStack::from_result(
                ChartBuilder::on(&base)
                    .margin(PaddingSizes::Small.size())
                    .x_label_area_size(40)
                    .y_label_area_size(55)
                    .build_cartesian_2d(0f64..length, (smallest_y - y_padding)..(largest_y + y_padding)),
                "Failed to build chart with data."
            );
            if chart_result.is_fail() { return ResultStack::new_fail_from_stack(chart_result.get_stack()).fail("Failed to render TrendParse.") }
            let mut chart = chart_result.wont_fail("This is past an is_fail() guard clause.");

            // configures the appearance
            let mut failures: RefCell<Vec<ResultStack<()>>> = RefCell::new(Vec::new());
            let configure_result = ResultStack::from_result(
                chart.configure_mesh()
                .light_line_style(grid_color)
                .bold_line_style(grid_color)
                .axis_style(text_color)
                .label_style(("sans-serif", 11).into_font().color(&text_color))
                .x_label_formatter(&|x| {
                    // gets the first time line to collect date labels
                    let first_time_line_result = ResultStack::from_option(self.time_lines.first(), "No time lines to get labels from!");
                    // fails if there are no time lines
                    // this should never happen as data is guararanteed at this point
                    if first_time_line_result.is_fail() {
                        failures.borrow_mut().push(first_time_line_result.empty_type().fail("Failed to render TrendParse."));
                        "no label data".to_string()
                    }
                    // proceeds to get the corrent label
                    else {
                        // collects the labels
                        let first_time_line = first_time_line_result.wont_fail("This is past an is_fail() guard clause.");
                        let labels: Vec<_> = first_time_line.time_stamps.iter().map(|tl| tl.time_label.clone()).collect();
                        // picks the label at the right position
                        let label_result = ResultStack::from_option(labels.get(*x as usize).cloned(), "Could not get label for x position!");
                        // fails if that position did not exist
                        if label_result.is_fail() {
                            failures.borrow_mut().push(label_result.empty_type().fail("Failed to render TrendParse."));
                            "no label data".to_string()
                        }
                        // returns the correct label
                        else { label_result.wont_fail("This is past an is_fail() clause.") }
                    }
                }).draw(),
                "Failed to configure chart!"
            );

            // checks if the configuration was successful
            let failures = failures.into_inner();
            if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to render TrendParse.") }
            if configure_result.is_fail() { return ResultStack::new_fail_from_stack(configure_result.get_stack()).fail("Failed to render TrendParse.") }

            // draws the lines with their respective tag labels
            let mut failures = Vec::new();
            for (i, (tag_label, points)) in plot_data.iter().enumerate() {
                // gets a temporary tag to get its color from the tag registry
                let tag_getter_result = Tag::new(tag_label);
                let material_color = if tag_getter_result.is_fail() {
                    failures.push(tag_getter_result.empty_type());
                    MaterialColors::Unavailable
                }
                else {
                    let getter_tag = tag_getter_result.wont_fail("This is past an is_fail() guard clause.");
                    app.bank.tag_registry.get(&getter_tag)
                };

                // the color
                let color = MaterialColors::color_as_rgb(material_color.materialized(Materials::Plastic, Depths::Flat, false, app.theme_selection));

                // draws the line
                let series_result = ResultStack::from_result(chart.draw_series(LineSeries::new(points.iter().copied(), ShapeStyle { color: color, filled: false, stroke_width: 2 })), "Failed to draw line!");
                if series_result.is_fail() { failures.push(series_result.empty_type()) }
                let series = series_result.wont_fail("This is past an is_fail() guard clause.");
                series
                    .label(tag_label)
                    .legend(move |(x, y)| PathElement::new([(x, y), (x + 16, y)], ShapeStyle { color: color, filled: false, stroke_width: 2 }));
            }

            // checks for failures
            if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to render TrendParse.") }

            // draws a legend box
            if plot_data.len() > 1 {
                let draw_result = ResultStack::from_result(
                    chart.configure_series_labels()
                        .background_style(background_color)
                        .border_style(grid_color)
                        .label_font(("sans-serif", 11).into_font().color(&text_color))
                        .draw(),
                    "Failed to draw legend!"
                );
                if draw_result.is_fail() { return ResultStack::new_fail_from_stack(draw_result.get_stack()).fail("Failed to render TrendParse.") }
            }
        }

        // gets the rgba data
        drop(base);
        let rgba_data: Vec<u8> = buffer.chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect();

        // returns the handle
        Pass(Handle::from_rgba(size.0, size.1, rgba_data))
    }
}



/// Holds data for displaying the relative spending or earning of a `Tag` over time (as `CashFlow`s at points in time).
pub struct TimeLine {
    /// Each `TimeLine` has a `Tag` attached. No `Tag` represents the overall `CashFlow`.
    tag: Option<Tag>,
    /// The list of `TimeStamp`s
    time_stamps: Vec<TimeStamp>
}
impl TimeLine {
    /// Creates a new `TimeLine`.
    #[must_use]
    fn new(bank: &Bank, transactions: &Vec<Transaction>, trending_tag: Option<Tag>, interval: Intervals, last_date: Date, length: usize) -> ResultStack<TimeLine>{
        // splits the transactions by time group.
        let mut all_time_groups: Vec<Vec<&Transaction>> = Vec::new();
        for transaction in transactions { TimeLine::place_into_time_group(transaction, &mut all_time_groups, interval); }

        // finds the first group to add.
        // it is the "first" because it comes first in the list, not because it is the first chronolgically
        let mut starting_index = 0;
        let mut start_found = false;
        for (i, group) in all_time_groups.iter().enumerate() {
            if TimeLine::contains_date(&group, last_date, interval) {
                start_found = true;
                starting_index = i;
                break;
            }
        }

        if !start_found { return ResultStack::new_fail("Failed to find the starting date in the given Transactions!").fail("Failed to create TrendParse.") }
        
        // takes only the time groups within the given length
        let mut collected_time_groups = Vec::new();
        let mut time_groups_added = 0;
        for i in starting_index..all_time_groups.len() {
            if time_groups_added < length {
                collected_time_groups.push(all_time_groups[i].clone());
                time_groups_added += 1;
            }
        }

        // filters out all the transactions that do not have the tag
        // None results in getting the trend of the overall cash flow
        if let Some(tag) = &trending_tag {
            for time_group in &mut collected_time_groups {
                time_group.retain(|t| t.has_tag(tag));
            }
        }

        // collects the cash flows for the collected time groups
        let cash_flow_results: Vec<ResultStack<CashFlow>> = collected_time_groups.iter().map(|group| CashFlow::new(bank, &Bank::get_ids_from(group), 1.0)).collect();
        let mut cash_flows = Vec::new();
        let mut failures = Vec::new();
        for cash_flow_result in cash_flow_results {
            match cash_flow_result {
                Pass(cash_flow) => cash_flows.push(cash_flow),
                Fail(_) => failures.push(cash_flow_result),
            }
        }

        // returns a failure if any of the cash flows failed to generate
        if !failures.is_empty() { return ResultStack::new_fail_from_stack(failures[0].get_stack()).fail("Failed to create TrendParse."); }

        // returns a failure is the length of cash flows and time groups are different
        if collected_time_groups.len() != cash_flows.len() { return ResultStack::new_fail("Generated mismatched TimeGroups and CashFlows while creating a TrendParse!").fail("Failed to create TrendParse.") }

        // creates the timeline from the collected time groups and cash flows
        let mut time_stamps = Vec::new();
        for (i, cash_flow) in cash_flows.into_iter().enumerate() {
            time_stamps.push(TimeStamp { cash_flow: cash_flow, time_label: TimeStamp::get_time_label(&collected_time_groups[i], interval) })
        }

        // returns a new TimeLine
        Pass(TimeLine { tag: trending_tag, time_stamps })
    }

    /// Gets the data used to plot the `TimeLine` on a chart.
    #[must_use]
    fn get_plot_data(&self, bank: &Bank) -> ResultStack<(String, Vec<(f64, f64)>)> {
        let label = match &self.tag {
            Some(tag) => tag.get_label(),
            None => "Overall".to_string(),
        };

        let mut failures = Vec::new();
        
        let points = self.time_stamps.iter().enumerate().map(|(i, ts)| {
            let flow_result = ts.cash_flow.unified(bank);
            
            if flow_result.is_fail() {
                failures.push(flow_result.empty_type());
                (i as f64, 0.0)
            }
            
            else {
                let flow_decimal = flow_result.wont_fail("This is past an is_fail() guard clause.");
                let flow_f64_result = ResultStack::from_option(flow_decimal.to_f64(), "Failed to convert decimal to f64!");
                
                if flow_f64_result.is_fail() {
                    failures.push(flow_f64_result.empty_type());
                    (i as f64, 0.0)
                }
                
                else {
                    let flow_f64 = flow_f64_result.wont_fail("This is past an is_fail() guard clause.");
                    (i as f64, flow_f64)
                }
            }
            
        }).collect();
        
        Pass((label, points))
    }

    /// Checks if a given `Date` fits into a given group of `Transaction`s (based on its `Interval`).
    #[must_use]
    fn contains_date(group: &Vec<&Transaction>, date: Date, interval: Intervals) -> bool {
        if group.is_empty() { return false; }
        
        match interval {
            Intervals::Weekly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::BiWeekly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Monthly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Quarterly => TimeLine::is_in_same_week(group[0].date, date),
            Intervals::Yearly => TimeLine::is_in_same_week(group[0].date, date),
        }
    }
    
    /// Places a `Transaction` into the correct group of `Transaction`s.
    fn place_into_time_group<'a>(transaction: &'a Transaction, groups: &mut Vec<Vec<&'a Transaction>>, interval: Intervals) {
        for group in groups.iter_mut() {
            if group.is_empty() { continue; }

            match interval {
                Intervals::Weekly => {
                    if TimeLine::is_in_same_week(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::BiWeekly => {
                    if TimeLine::is_in_same_biweek(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Monthly => {
                    if TimeLine::is_in_same_month(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Quarterly => {
                    if TimeLine::is_in_same_quarter(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
                Intervals::Yearly => {
                    if TimeLine::is_in_same_year(group[0].date, transaction.date) {
                        group.push(transaction);
                        return;
                    }
                }
            }
        }
        
        groups.push(vec![transaction]);
    }

    /// Checks if two `Date`s are in the same week.
    #[must_use]
    fn is_in_same_week(first: Date, second: Date) -> bool {
        TimeLine::get_week_id_for(first) == TimeLine::get_week_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same two-week couple.
    #[must_use]
    fn is_in_same_biweek(first: Date, second: Date) -> bool {
        TimeLine::get_biweek_id_for(first) == TimeLine::get_biweek_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same month.
    #[must_use]
    fn is_in_same_month(first: Date, second: Date) -> bool {
        TimeLine::get_month_id_for(first) == TimeLine::get_month_id_for(second)
    }

    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_quarter(first: Date, second: Date) -> bool {
        TimeLine::get_quarter_id_for(first) == TimeLine::get_quarter_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_year(first: Date, second: Date) -> bool {
        TimeLine::get_year_id_for(first) == TimeLine::get_year_id_for(second)
    }
        
    /// Gets the week id for a given `Date`.
    #[must_use]
    fn get_week_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let mut distance_into_year: u32 = date.get_day();
        for month in date.get_month().get_previous_months() {
            distance_into_year += month.days_in_month(date.get_year());
        }
        let week = (distance_into_year / 7) + 1;
        
        (year * 100) + week
    }

    /// Gets the two-week couple id for a given `Date`.
    #[must_use]
    fn get_biweek_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let mut distance_into_year: u32 = date.get_day();
        for month in date.get_month().get_previous_months() {
            distance_into_year += month.days_in_month(date.get_year());
        }
        let biweek = (distance_into_year / 14) + 1;
        
        (year * 100) + biweek
    }
    
    /// Gets the month id for a given `Date`.
    #[must_use]
    fn get_month_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let month = date.get_month().as_value();
        (year * 100) + month
    }

    /// Gets the quarter id for a given `Date`.
    #[must_use]
    fn get_quarter_id_for(date: Date) -> u32 {
        let year = date.get_year();
        let quarter = match date.get_month() {
            Months::January | Months::February | Months::March => 1,
            Months::April | Months::May | Months::June => 2,
            Months::July | Months::August | Months::September => 3,
            Months::October | Months::November | Months::December => 4,
        };
        
        (year * 10) + quarter
    }
    
    /// Gets the year id for a given `Date`.
    #[must_use]
    fn get_year_id_for(date: Date) -> u32 {
        date.get_year()
    }
}



/// Holds the `CashFlow` for a certain `Tag` at a given point in time.
pub struct TimeStamp {
    /// Shows if money was earned or spent during a time period.
    cash_flow: CashFlow,
    /// The time period of the `TimeStamp`. (January, Q1 2026, etc.)
    time_label: String,
}
impl TimeStamp {
    /// Gets the label for the `TimeStamp`.
    #[must_use]
    fn get_time_label(time_group: &Vec<&Transaction>, interval: Intervals) -> String {
        if time_group.is_empty() { "No data".to_string() }
        else {
            let date = time_group[0].date;
            match interval {
                Intervals::Weekly => { format!("Week {}", TimeLine::get_week_id_for(date)) }
                Intervals::BiWeekly => { format!("Bi-Week {}", TimeLine::get_biweek_id_for(date)) }
                Intervals::Monthly => { format!("{}, {}", date.get_month().display(), date.get_year()) }
                Intervals::Quarterly => {
                    let quarter = match date.get_month() {
                        Months::January | Months::February | Months::March => 1,
                        Months::April | Months::May | Months::June => 2,
                        Months::July | Months::August | Months::September => 3,
                        Months::October | Months::November | Months::December => 4,
                    };
                    format!("Q{}", quarter)
                }
                Intervals::Yearly => { format!("{}", TimeLine::get_year_id_for(date)) }
            }
        }
    }
}