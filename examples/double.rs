// Crate Dependencies ---------------------------------------------------------
extern crate cursive;
extern crate chrono;
extern crate cursive_calendar_view;


// External Dependencies ------------------------------------------------------
use chrono::prelude::*;
use cursive::Cursive;
use cursive::direction::Orientation;
use cursive::views::{BoxView, Dialog, DummyView, LinearLayout};


// Modules --------------------------------------------------------------------
use cursive_calendar_view::{CalendarView, EnglishLocale, ViewMode};


// Example --------------------------------------------------------------------
fn main() {

    let mut siv = Cursive::new();

    let mut calendar_a = CalendarView::<UTC, EnglishLocale>::new(UTC.ymd(2017, 7, 26));
    calendar_a.set_highest_view_mode(ViewMode::Year);
    calendar_a.set_earliest_date(Some(UTC.ymd(2017, 1, 1)));
    calendar_a.set_latest_date(Some(UTC.ymd(2017, 12, 31)));
    calendar_a.set_show_iso_weeks(true);

    let mut calendar_b = CalendarView::<UTC, EnglishLocale>::new(UTC.ymd(2017, 7, 26));
    calendar_b.set_highest_view_mode(ViewMode::Year);
    calendar_b.set_earliest_date(Some(UTC.ymd(2017, 1, 1)));
    calendar_b.set_latest_date(Some(UTC.ymd(2017, 12, 31)));

    let mut layout = LinearLayout::new(Orientation::Horizontal);
    layout.add_child(calendar_a);
    layout.add_child(BoxView::with_fixed_size((4, 0), DummyView));
    layout.add_child(calendar_b);

    siv.add_layer(
        Dialog::around(layout).title("Calendar View Demo")
    );

    siv.run();

}

