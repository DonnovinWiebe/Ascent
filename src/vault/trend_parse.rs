use std::cell::RefCell;
use crate::{ui::{components::{Heights, PaddingSizes, TextSizes, Widths}, material::{AppThemes, Depths, MaterialColors, Materials}}, vault::{bank::{Bank, TagRegistry}, parse::CashFlow, schrod::Schrod, transaction::{Date, Months, Tag, Transaction, Value}}};
use crate::vault::schrod::Schrod::Pass;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea, element::PathElement, series::LineSeries, style::{IntoFont, ShapeStyle}};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use iced::widget::image::Handle;
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



/// Holds a group of `Transaction`s that share the same time interval.
#[derive(Debug, Clone, PartialEq)]
struct TimeGroup<'a> {
    transactions: Vec<&'a Transaction>,
    date: Date,
    interval: Intervals,
}
impl<'a> TimeGroup<'a> {
    /// Creates a new `TimeGroup`.
    #[must_use]
    fn new(transactions: Vec<&'a Transaction>, date: Date, interval: Intervals) -> TimeGroup<'a> {
        TimeGroup { transactions, date, interval }
    }
    
    /// Checks if the given `TimeGroup` contains the given `Date`.
    #[must_use]
    fn contains_date(&self, date: Date) -> bool {
        if self.transactions.is_empty() { return false; }
        
        match self.interval {
            Intervals::Weekly => TimeGroup::is_in_same_week(self.transactions[0].date, date),
            Intervals::BiWeekly => TimeGroup::is_in_same_week(self.transactions[0].date, date),
            Intervals::Monthly => TimeGroup::is_in_same_week(self.transactions[0].date, date),
            Intervals::Quarterly => TimeGroup::is_in_same_week(self.transactions[0].date, date),
            Intervals::Yearly => TimeGroup::is_in_same_week(self.transactions[0].date, date),
        }
    }

    /// Filters out all `Transaction`s that do not have the given `Tag`.
    fn filter_for(&mut self, tag: &Tag) {
        self.transactions.retain(|t| t.has_tag(tag));
    }

    /// Sorts a given list of `TimeGroup`s by `Date`.
    fn sort_time_groups(groups: &mut Vec<TimeGroup>) {
        groups.sort_by(|a, b| b.date.as_value().cmp(&a.date.as_value()));
    }
    
    /// Places a `Transaction` into the correct group of `Transaction`s.
    fn place_into_time_group<'b>(transaction: &'a Transaction, groups: &'b mut Vec<TimeGroup<'a>>, interval: Intervals) where 'a: 'b {
        for group in groups.iter_mut() {
            match interval {
                Intervals::Weekly => {
                    if TimeGroup::is_in_same_week(group.date, transaction.date) {
                        group.transactions.push(transaction);
                        return;
                    }
                }
                Intervals::BiWeekly => {
                    if TimeGroup::is_in_same_biweek(group.date, transaction.date) {
                        group.transactions.push(transaction);
                        return;
                    }
                }
                Intervals::Monthly => {
                    if TimeGroup::is_in_same_month(group.date, transaction.date) {
                        group.transactions.push(transaction);
                        return;
                    }
                }
                Intervals::Quarterly => {
                    if TimeGroup::is_in_same_quarter(group.date, transaction.date) {
                        group.transactions.push(transaction);
                        return;
                    }
                }
                Intervals::Yearly => {
                    if TimeGroup::is_in_same_year(group.date, transaction.date) {
                        group.transactions.push(transaction);
                        return;
                    }
                }
            }
        }
        
        groups.push(TimeGroup::new(vec![transaction], transaction.date, interval));
    }

    /// Gets the label for the `TimeGroup`.
    #[must_use]
    fn date_label(&self) -> String {
        match self.interval {
            Intervals::Weekly => { format!("Week\n{}", TimeGroup::get_week_id_for(self.date)) }
            Intervals::BiWeekly => { format!("Bi-Week\n{}", TimeGroup::get_biweek_id_for(self.date)) }
            Intervals::Monthly => { format!("{},\n{}", self.date.get_month().display(), self.date.get_year()) }
            Intervals::Quarterly => {
                let quarter = match self.date.get_month() {
                    Months::January | Months::February | Months::March => 1,
                    Months::April | Months::May | Months::June => 2,
                    Months::July | Months::August | Months::September => 3,
                    Months::October | Months::November | Months::December => 4,
                };
                format!("Q{}\n{}", quarter, self.date.get_year())
            }
            Intervals::Yearly => { format!("{}", TimeGroup::get_year_id_for(self.date)) }
        }
    }

    /// Checks if two `Date`s are in the same week.
    #[must_use]
    fn is_in_same_week(first: Date, second: Date) -> bool {
        TimeGroup::get_week_id_for(first) == TimeGroup::get_week_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same two-week couple.
    #[must_use]
    fn is_in_same_biweek(first: Date, second: Date) -> bool {
        TimeGroup::get_biweek_id_for(first) == TimeGroup::get_biweek_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same month.
    #[must_use]
    fn is_in_same_month(first: Date, second: Date) -> bool {
        TimeGroup::get_month_id_for(first) == TimeGroup::get_month_id_for(second)
    }

    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_quarter(first: Date, second: Date) -> bool {
        TimeGroup::get_quarter_id_for(first) == TimeGroup::get_quarter_id_for(second)
    }
    
    /// Checks if two `Date`s are in the same quarter.
    #[must_use]
    fn is_in_same_year(first: Date, second: Date) -> bool {
        TimeGroup::get_year_id_for(first) == TimeGroup::get_year_id_for(second)
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



/// Holds data for a graphical representation of `CashFlow`s by `Tag` over time.
#[derive(Debug, Clone, PartialEq)]
pub struct TrendParse {
    /// A list of individual `CashFlow`s over time grouped by `Tag`.
    time_lines: Vec<TimeLine>,
    /// The interval between `CashFlow`s.
    interval: Intervals,
    /// A cached `Handle` of the chart.
    pub chart_handle: Schrod<Handle>,
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
        (width * 2, height * 2)
    }



    // data retrieval
    /// Returns if the given `Tag` is trending.
    #[must_use]
    pub fn is_tag_trending(&self, tag: &Tag) -> bool {
        self.get_trending_tags().contains(tag)
    }

    /// Returns a list of all trending `Tag`s.
    #[must_use]
    pub fn get_trending_tags(&self) -> Vec<Tag> {
        self.time_lines.iter().map(|tl| tl.tag.clone()).flatten().collect()
    }



    // assembling
    /// Creates a new `TrendParse`.
    #[must_use]
    pub fn new(bank: &Bank, transactions: &Vec<Transaction>, show_balance: bool, tags: Vec<Tag>, interval: Intervals, last_date: Date, length: usize) -> Schrod<TrendParse> {
        // the list of time lines
        let mut time_line_results = Vec::new();

        // adding a time line for the overall balance
        if show_balance { time_line_results.push(TimeLine::new(bank, transactions, None, interval, last_date, length)) }

        // adding time lines for each tag
        for tag in tags { time_line_results.push(TimeLine::new(bank, transactions, Some(tag), interval, last_date, length)) }

        // checking for failures
        if Schrod::contains_fail(&time_line_results) {
            return Schrod::collect_and_fail(&time_line_results, "TrendParse::new()")
                .convert("TrendParse::new()")
                .fail("Failed to create TrendParse", "TrendParse::new()")
        }
        let time_lines: Vec<_> = time_line_results.into_iter().map(|result| result.wont_fail("This is past a contains_fail() guard clause.", "TrendParse::new()")).collect();
        
        // returns the trend parse
        Pass(TrendParse { time_lines, interval, chart_handle: Schrod::new_fail("No Handle has been generated.", "TrendParse::new()") })
    }
    
    /*
    /// Gets the highest and lowest `CashFlow` values (currency unified).
    #[must_use]
    fn get_flow_range(&self, currency_exchange: &CurrencyExchange) -> Schrod<(Decimal, Decimal)> {
        let mut lowest_flow: Option<Decimal> = None;
        let mut highest_flow: Option<Decimal> = None;
        let mut failures = Vec::new();
        
        for time_line in &self.time_lines {
            for time_stamp in &time_line.time_stamps {
                let unified_flow_result = time_stamp.cash_flow.unified(currency_exchange);
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

        if !failures.is_empty() { return Schrod::new_fail_from_stack(failures[0].get_stack()).fail("Failed to get flow range!"); }

        if let Some(lowest) = lowest_flow && let Some(highest) = highest_flow {
            Pass((lowest, highest))
        }

        else { Schrod::new_fail("Unknown failure.").fail("Failed to get flow range!") }
    }
    */
    
    /// Returns rendering data: one entry per TimeLine — (series label, points).
    #[must_use]
    fn get_plot_data(&self) -> Schrod<Vec<(String, Vec<(f64, f64)>)>> {
        // collects the data results
        let plot_data_results: Vec<_> = self.time_lines.iter().map(|tl| tl.get_plot_data()).collect();

        // checks for failures
        if Schrod::contains_fail(&plot_data_results) {
            return Schrod::collect_and_fail(&plot_data_results, "TrendParse::get_plot_data()")
                .convert("TrendParse::get_plot_data()")
                .fail("Failed to get plot data from TrendParse.", "TrendParse::get_plot_data()")
        }
        let plot_data: Vec<_> = plot_data_results.into_iter().map(|result| result.wont_fail("This is past a contains_fail() guard clause.", "TrendParse::get_plot_data()")).collect();

        // ensures that all the time lines have the same number of points
        if !plot_data.is_empty() {
            let data_length = plot_data[0].1.len();
            for line in &plot_data {
                if line.1.len() != data_length {
                    return Schrod::new_fail("Generated TimeLines of differing lengths!", "TrendParse::get_plot_data()")
                            .fail("Failed to get plot data from TrendParse.", "TrendParse::get_plot_data()")
                }
            }
        }

        // returns the plot data
        Pass(plot_data)
    }

    /// Generates a chart `Handle` for the given `TrendParse` and returns the results.
    #[must_use]
    pub async fn render(&mut self, tag_registry_copy: TagRegistry, theme: AppThemes) -> Schrod<()> {
        // a basic failed handle to place into self.chart_handle if rendering fails
        let failed_handle = Schrod::new_fail("Failed to render TrendParse.", "TrendParse::render()");
    
        // holds the image data
        let size = TrendParse::max_size();
        let mut buffer = vec![0u8; (size.0 * size.1 * 3) as usize];

        // colors
        let background_color = MaterialColors::color_as_rgb(MaterialColors::Card.materialized(
            Materials::Plastic,
            Depths::Flat,
            false,
            theme,
        ));
        let grid_color = MaterialColors::color_as_rgb(MaterialColors::CardContent.materialized(
            Materials::Plastic,
            Depths::Flat,
            false,
            theme,
        ));
        let text_color = MaterialColors::color_as_rgb(MaterialColors::StrongText.materialized(
            Materials::Plastic,
            Depths::Flat,
            false,
            theme,
        ));

        // the base chart
        let base_result = Schrod::from_result(BitMapBackend::<RGBPixel>::with_buffer_and_format(&mut buffer, (size.0, size.1)), "Failed to create BitMapBackend!", "TrendParse::render()");
        if base_result.is_fail() {
            self.chart_handle = failed_handle;
            return base_result
                .convert("TrendParse::Render()")
                .fail("Failed to render TrendParse.", "TrendParse::render()")
        }
        let base = base_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()").into_drawing_area();

        // fills the background of the base chart
        let fill_result = Schrod::from_result(base.fill(&background_color), "Failed to fill background of chart.", "TrendParse::render()");
        if fill_result.is_fail() {
            self.chart_handle = failed_handle;
            return fill_result
                .fail("Failed to render TrendParse.", "TrendParse::render()")
        }

        // gets the plot data
        let plot_data_result = self.get_plot_data();
        if plot_data_result.is_fail() {
            self.chart_handle = failed_handle;
            return plot_data_result
                .convert("TrendParse::Render()")
                .fail("Failed to render TrendParse.", "TrendParse::render()")
        }
        let plot_data = plot_data_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()");

        // presents the chart
        // without plot data
        if plot_data.is_empty() {
            let presented_base_result = Schrod::from_result(base.present(), "Failed to present chart without data.", "TrendParse::render()");
            if presented_base_result.is_fail() {
                self.chart_handle = failed_handle;
                return presented_base_result
                    .fail("Failed to render TrendParse.", "TrendParse::render()")
            }
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
            let chart_result = Schrod::from_result(
                ChartBuilder::on(&base)
                    .margin(PaddingSizes::Small.size())
                    .x_label_area_size(60)
                    .y_label_area_size(150)
                    .margin_right(150)
                    .build_cartesian_2d(0f64..length, (smallest_y - y_padding)..(largest_y + y_padding)),
                "Failed to build chart with data.",
                "TrendParse::render()",
            );
            if chart_result.is_fail() {
                self.chart_handle = failed_handle;
                return chart_result
                    .convert("TrendParse::Render()")
                    .fail("Failed to render TrendParse.", "TrendParse::render()")
            }
            let mut chart = chart_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()");

            // configures the appearance
            let failures: RefCell<Vec<Schrod<()>>> = RefCell::new(Vec::new());
            let configure_result = Schrod::from_result(
                chart.configure_mesh()
                .x_labels(plot_data[0].1.len())
                .bold_line_style(ShapeStyle { color: grid_color, filled: false, stroke_width: 2 })
                .light_line_style(plotters::style::TRANSPARENT)
                .axis_style(text_color)
                .x_label_style(("sans-serif", TextSizes::Interactable.size() * 2.0).into_font().color(&text_color))
                .y_label_style(("sans-serif", TextSizes::Interactable.size() * 2.0).into_font().color(&text_color))
                //.x_label_offset(-75)
                .x_label_formatter(&|x| {
                    // gets the first time line to collect date labels
                    let first_time_line_result = Schrod::from_option(self.time_lines.first(), "No time lines to get labels from!", "TrendParse::render()");
                    // fails if there are no time lines
                    // this should never happen as data is guararanteed at this point
                    if first_time_line_result.is_fail() {
                        failures.borrow_mut().push(first_time_line_result.convert("TrendParse::render()").fail("Failed to render TrendParse.", "TrendParse::render()"));
                        "no label data".to_string()
                    }
                    // proceeds to get the corrent label
                    else {
                        // collects the labels
                        let first_time_line = first_time_line_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()");
                        let labels: Vec<_> = first_time_line.time_stamps.iter().map(|tl| tl.date_label.clone()).collect();
                        // picks the label at the right position
                        let label_result = Schrod::from_option(labels.get(*x as usize).cloned(), "Could not get label for x position!", "TrendParse::render()");
                        // fails if that position did not exist
                        if label_result.is_fail() {
                            failures.borrow_mut().push(label_result.convert("TrendParse::render()").fail("Failed to render TrendParse.", "TrendParse::render()"));
                            "no label data".to_string()
                        }
                        // returns the correct label
                        else { label_result.wont_fail("This is past an is_fail() clause.", "TrendParse::render()") }
                    }
                }).draw(),
                "Failed to configure chart!",
                "TrendParse::render()",
            );

            // checks if the configuration was successful 
            let mut failures = failures.into_inner();
            failures.push(configure_result);
            if Schrod::contains_fail(&failures) {
                self.chart_handle = failed_handle;
                return Schrod::collect_and_fail(&failures, "TrendParse::render()")
                    .fail("Failed to render TrendParse.", "TrendParse::render()")
            }

            // draws the lines with their respective tag labels
            let mut failures = Vec::new();
            for (tag_label, points) in plot_data.iter() {
                let material_color;
                // if the tag label is "Balance" (representing the overall balance, not a specific tag), gets the accent color
                if tag_label == "Balance" { material_color = MaterialColors::accent(theme); }

                // or gets the color from the tag itself
                else {
                    // gets a temporary tag to get its color from the tag registry
                    let tag_getter_result = Tag::new(tag_label);
                    material_color = if tag_getter_result.is_fail() {
                        failures.push(tag_getter_result);
                        MaterialColors::Unavailable
                    }
                    else {
                        let getter_tag = tag_getter_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()");
                        tag_registry_copy.get(&getter_tag)
                    };
                }
                
                // the color
                let color = MaterialColors::color_as_rgb(material_color.materialized(Materials::Plastic, Depths::Flat, false, theme));

                // draws the line
                let series_result = Schrod::from_result(chart.draw_series(LineSeries::new(points.iter().copied(), ShapeStyle { color: color, filled: false, stroke_width: 4 })), "Failed to draw line!", "TrendParse::render()");
                if series_result.is_fail() { failures.push(series_result.convert("TrendParse::render()")) }
                let series = series_result.wont_fail("This is past an is_fail() guard clause.", "TrendParse::render()");
                series
                    .label(tag_label)
                    .legend(move |(x, y)| PathElement::new([(x, y), (x + 16, y)], ShapeStyle { color: color, filled: false, stroke_width: 2 }));
            }

            // checks for failures
            if Schrod::contains_fail(&failures) {
                self.chart_handle = failed_handle;
                return Schrod::collect_and_fail(&failures, "TrendParse::render()")
                    .convert("TrendParse::Render()")
                    .fail("Failed to render TrendParse.", "TrendParse::render()")
            }

            // draws a legend box
            if plot_data.len() > 1 {
                let draw_result = Schrod::from_result(
                    chart.configure_series_labels()
                        .background_style(background_color)
                        .border_style(grid_color)
                        .label_font(("sans-serif", 11).into_font().color(&text_color))
                        .draw(),
                    "Failed to draw legend!",
                    "TrendParse::render()",
                );
                if draw_result.is_fail() {
                    self.chart_handle = failed_handle;
                    return draw_result
                        .fail("Failed to render TrendParse.", "TrendParse::render()")
                }
            }
        }

        // gets the rgba data
        drop(base);
        let rgba_data: Vec<u8> = buffer.chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect();

        // succeeds
        self.chart_handle = Pass(Handle::from_rgba(size.0, size.1, rgba_data));
        Pass(())
    }
}



/// Holds data for displaying the relative spending or earning of a `Tag` over time
/// as `CashFlow`s at points in time, each building on the value of the previous.
#[derive(Debug, Clone, PartialEq)]
pub struct TimeLine {
    /// Each `TimeLine` has a `Tag` attached to it.
    /// No `Tag` represents overall `CashFlow`.
    tag: Option<Tag>,
    /// The list of `TimeStamp`s.
    time_stamps: Vec<TimeStamp>
}
impl TimeLine {
    /// Creates a new `TimeLine`.
    #[must_use]
    fn new(bank: &Bank, transactions: &Vec<Transaction>, trending_tag: Option<Tag>, interval: Intervals, last_date: Date, length: usize) -> Schrod<TimeLine>{
        // splits the transactions by time group and sorts them by date
        let mut all_time_groups: Vec<TimeGroup> = Vec::new();
        for transaction in transactions { TimeGroup::place_into_time_group(transaction, &mut all_time_groups, interval); }
        TimeGroup::sort_time_groups(&mut all_time_groups);

        // finds the first group to add.
        // it is the "first" because it comes first in the list (it should be last chronologically)
        let mut starting_index = 0;
        let mut start_found = false;
        for (i, group) in all_time_groups.iter().enumerate() {
            if group.contains_date(last_date) {
                start_found = true;
                starting_index = i;
                break;
            }
        }
        if !start_found {
            return Schrod::new_fail("Failed to find the starting date in the given list of Transactions!", "TimeLine::new()")
                .fail("Failed to create TimeLine.", "TimeLine::new()")
        }
        
        // takes only the time groups within the given length
        let mut collected_time_groups = Vec::new();
        let mut time_groups_added = 0;
        for i in starting_index..all_time_groups.len() {
            if time_groups_added < length {
                collected_time_groups.push(all_time_groups[i].clone());
                time_groups_added += 1;
            }
        }

        // reveres the order of the time groups since by default they have the most recent
        // "first" in order to show up first in the bank's ledger
        collected_time_groups.reverse();

        // filters out all the transactions that do not have the tag
        // None results in getting the trend of the overall cash flow
        if let Some(tag) = &trending_tag {
            for time_group in &mut collected_time_groups {
                time_group.filter_for(tag);
            }
        }

        // collects the cash flows for the collected time groups
        let cash_flow_results: Vec<Schrod<CashFlow>> = collected_time_groups.iter().map(|group| CashFlow::new(bank, &Bank::get_ids_from(&group.transactions))).collect();
        if Schrod::contains_fail(&cash_flow_results) {
            return Schrod::collect_and_fail(&cash_flow_results, "TimeLine::new()")
                .convert("TimeLine::new()")
                .fail("Failed to create TimeLine.", "TimeLine::new()")
        }
        let cash_flows: Vec<_> = cash_flow_results.into_iter().map(|result| result.wont_fail("This is past a contains_fail() guard clause.", "TimeLine::new()")).collect();

        // converts the values to normal decimals for ease of use and chains them together to form
        // a net balance line if the tag is set to None
        let mut current_balance = Decimal::from(0);
        let mut cash_flow_values = Vec::new();
        for (i, cash_flow) in cash_flows.iter().enumerate() {
            // if this is a balance line, the first value is always 0 to represent a starting point
            let unified = if i == 0 { Decimal::from(0) } else { *cash_flow.unified().amount() };
            current_balance += unified;
            cash_flow_values.push(if trending_tag == None { current_balance } else { unified });
        }
        
        // returns a failure is the length of cash flow values and time groups are different
        if collected_time_groups.len() != cash_flow_values.len() {
            return Schrod::new_fail("Generated different amounts of TimeGroups and cash flow values while creating a TimeLine!", "TimeLine::new()")
                .fail("Failed to create TimeLine.", "TimeLine::new()")
        }

        // creates the timeline from the collected time groups and cash flows
        let currency = bank.currency_exchange.get_main_currency();
        let mut time_stamps = Vec::new();
        for (i, cash_flow_value) in cash_flow_values.into_iter().enumerate() {
            time_stamps.push(TimeStamp { cash_flow_value: Value::from_decimal(cash_flow_value, &currency), date_label: collected_time_groups[i].date_label() })
        }

        // returns a new TimeLine
        Pass(TimeLine { tag: trending_tag, time_stamps })
    }

    /// Gets the data used to plot the `TimeLine` on a chart.
    #[must_use]
    fn get_plot_data(&self) -> Schrod<(String, Vec<(f64, f64)>)> {
        let tag_label = match &self.tag {
            Some(tag) => tag.get_label(),
            None => "Balance".to_string(),
        };

        let point_results: Vec<_> = self.time_stamps.iter().enumerate().map(|(i, ts)| {
            let flow_f64_result = Schrod::from_option(ts.cash_flow_value.amount().to_f64(), "Failed to convert decimal to f64!", "TimeLine::get_plot_data()");
            
            if flow_f64_result.is_fail() {
                flow_f64_result
                    .convert("TimeLine::get_plot_data()")
                    .fail("Failed to get plot data from TimeLine.", "TimeLine::get_plot_data()")
            }
            else {
                let flow_f64 = flow_f64_result.wont_fail("This is past an is_fail() guard clause.", "TimeLine::get_plot_data()");
                Pass((i as f64, flow_f64))
            }
        }).collect();

        if Schrod::contains_fail(&point_results) {
            return Schrod::collect_and_fail(&point_results, "TimeLine::get_plot_data()")
                .convert("TimeLine::get_plot_data()")
                .fail("Failed to get plot data from TimeLine.", "TimeLine::get_plot_data()")
        }

        let points: Vec<_> = point_results.into_iter().map(|result| result.wont_fail("This is past a contains_fail() guard clause.", "TimeLine::get_plot_data()")).collect();
        
        Pass((tag_label, points))
    }
}



/// Holds the cash flow (as a `Value`) associated with a given `Tag` at a given point in time.
/// The `Tag` is not stored in the `TimeStamp`, but is used when being created by the `TimeLine` it lives in.
#[derive(Debug, Clone, PartialEq)]
struct TimeStamp {
    /// Shows if money was earned or spent during a time period.
    /// These do not build cumulatively on the previous as that is tracked by the `TimeLine`.
    cash_flow_value: Value,
    /// The time period/date of the `TimeStamp`. (January, Q1 2026, etc.)
    date_label: String,
}