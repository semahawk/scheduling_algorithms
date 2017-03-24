//
// fcfs.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 15 Mar 2017 18:30:07 +0100 (CET)
//

use std::collections::VecDeque;

use process::*;
use scheduler::*;
use tui::*;

#[derive(Debug)]
pub struct FCFS {
  process_list: VecDeque<Process>,
  context_switch_num: usize,
}

pub fn new() -> FCFS {
  FCFS {
    process_list: VecDeque::new(),
    context_switch_num: 0usize,
  }
}

impl Scheduler for FCFS {
  fn name(&self) -> String {
    format!("FCFS")
  }

  fn has_processes(&self) -> bool {
    self.process_list.is_empty() == false
  }

  fn add_process(&mut self, process: Process) {
    self.process_list.push_back(process);
  }

  fn schedule(&mut self) {
    // we're always taking the first one in the queue
  }

  fn current_proc(&self) -> Option<&Process> {
    self.process_list.front()
  }

  fn current_proc_mut(&mut self) -> Option<&mut Process> {
    self.process_list.front_mut()
  }

  fn kill_current_proc(&mut self) {
    self.context_switch_num += 1;
    self.process_list.pop_front();
  }

  fn list_processes(&self, mut tui: &mut Tui) {
    tui.draw_process_list(self.process_list.as_slices().0);
  }

  fn increase_waiting_times(&mut self) {
    for process in self.process_list.iter_mut().skip(1) {
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

