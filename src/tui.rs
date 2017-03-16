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
use cursive::traits::*;

use process::*;

fn label_maker(value: usize, (min, max): (usize, usize)) -> String {
  format!("")
}

pub fn draw_process_list(mut tui: &mut Cursive, process_list: &[Process]) {
  let mut process_list_view = tui.find_id::<ListView>("process_list").unwrap();

  process_list_view.clear();

  for p in process_list.iter() {
    let progress_bar_value = ((p.execution_time as f64 / p.burst_time as f64) as f64 * 100f64) as usize;

    process_list_view.add_child(p.name.clone().as_str(),
      ProgressBar::new().with_value(Counter::new(progress_bar_value)));
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

