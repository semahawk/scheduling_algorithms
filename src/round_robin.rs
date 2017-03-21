//
// round_robin.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 19 Mar 2017 15:57:22 +0100 (CET)
//

use process::*;
use scheduler::*;
use tui::*;

#[derive(Debug)]
pub struct RR {
  process_list: Vec<Process>,
  current: Option<usize>,
  context_switch_num: usize,
}

pub fn new() -> RR {
  RR {
    process_list: Vec::new(),
    current: None,
    context_switch_num: 0usize,
  }
}

impl Scheduler for RR {
  fn name(&self) -> String {
    format!("RR")
  }

  fn has_processes(&self) -> bool {
    self.process_list.is_empty() == false
  }

  fn add_process(&mut self, process: Process) {
    self.process_list.push(process);

    if let None = self.current {
      self.current = Some(0);
    };
  }

  fn schedule(&mut self) {
    match self.current {
      Some(idx) => {
        self.current = Some((idx + 1) % self.process_list.len());
      },
      None => self.current = Some(0),
    }

    self.context_switch_num += 1;
  }

  fn current_proc(&self) -> Option<&Process> {
    match self.current {
      Some(idx) => self.process_list.get(idx),
      None => None,
    }
  }

  fn current_proc_mut(&mut self) -> Option<&mut Process> {
    match self.current {
      Some(idx) => self.process_list.get_mut(idx),
      None => None,
    }
  }

  fn kill_current_proc(&mut self) {
    match self.current {
      Some(idx) => {
        self.process_list.remove(idx);
        if idx >= self.process_list.len() {
          self.current = Some(0);
          self.context_switch_num += 1;
        }
      },
      None => (),
    };
  }

  fn list_processes(&self, mut tui: &mut Tui) {
    tui.draw_process_list(self.process_list.as_slice());
  }

  fn increase_waiting_times(&mut self) {
    for process in self.process_list.iter_mut() {
      process.increase_waiting_time();
    }
  }

  fn context_switch_num(&self) -> usize {
    self.context_switch_num
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

