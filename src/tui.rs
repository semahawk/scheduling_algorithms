//
// tui.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 16 Mar 2017 22:14:17 +0100 (CET)
//

use cursive::Cursive;
use cursive::view::*;
use cursive::views::*;

use process::*;

pub struct Tui {
  renderer: Cursive,
}

pub fn new() -> Tui {
  let mut renderer = Cursive::new();

  let process_list = ListView::new().with_id("process_list").full_width();
  let results = TextView::new("").scroll_strategy(ScrollStrategy::StickToBottom)
    .with_id("results").full_height().min_width(48);
  let header = TextView::new("").with_id("header");
  let debug = TextView::new("").scroll_strategy(ScrollStrategy::StickToBottom).with_id("debug").full_height();
  let scenarios = ListView::new().with_id("scenarios").max_height(4);

  renderer.set_fps(60);

  if cfg!(feature = "debug-info") {
    renderer.add_layer(debug);
  }

  renderer.add_layer(
    LinearLayout::vertical().child(
        LinearLayout::horizontal()
          .child(
            LinearLayout::vertical()
              .child(Dialog::around(header).title("Header"))
              .child(Dialog::around(results).title("Results"))
          )
          .child(Dialog::around(process_list).title("Process list"))
      ).child(Dialog::around(scenarios).title("Scenarios")));

  renderer.add_global_callback('q', |tui| tui.quit());

  Tui {
    renderer: renderer,
  }
}

impl Tui {
  pub fn update(&mut self) {
    self.renderer.step();
  }

  pub fn draw_process_list(&mut self, process_list: &[Process]) {
    let mut process_list_view = self.renderer.find_id::<ListView>("process_list").unwrap();

    process_list_view.clear();

    for p in process_list.iter() {
      let progress_bar_value = ((p.execution_time as f64 / p.burst_time as f64) as f64 * 100f64) as usize;

      process_list_view.add_child(p.name.clone().as_str(),
        ProgressBar::new().with_value(Counter::new(progress_bar_value)));
    }
  }

  pub fn set_header(&mut self, text: String) {
    let mut header = self.renderer.find_id::<TextView>("header").unwrap();

    header.set_content(text.clone());
  }

  pub fn debug(&mut self, text: String) {
    if cfg!(feature = "debug-info") {
      let mut debug = self.renderer.find_id::<TextView>("debug").unwrap();

      debug.append_content(&text);
      debug.append_content("\n");
    }
  }

  pub fn add_result(&mut self, text: String) {
    let mut debug = self.renderer.find_id::<TextView>("results").unwrap();

    debug.append_content(&text);
    debug.append_content("\n");
  }

  pub fn add_scenario(&mut self, prefix: String, scenario: &Vec<usize>) {
    let mut scenarios = self.renderer.find_id::<ListView>("scenarios").unwrap();

    scenarios.add_child(&prefix, TextView::new(format!("{:?}", scenario)));
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

