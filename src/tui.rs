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
  let process_list = ListView::new().with_id("process_list");
  let info_bar = LinearLayout::vertical().with_id("info_bar");

  let mut layout = LinearLayout::horizontal();

  layout.add_child(Dialog::around(process_list).title("Process list"));
  layout.add_child(Dialog::around(info_bar).title("Info bar"));

  renderer.set_fps(60);
  renderer.add_layer(layout);
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
}

/*
 * vi: ts=2 sw=2 expandtab
 */

