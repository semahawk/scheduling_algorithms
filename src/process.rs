//
// process.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 15 Mar 2017 18:06:31 +0100 (CET)
//

#[derive(Debug)]
pub struct Process {
  pub id: usize,
  pub name: String,
  pub arrival_time: usize,
  pub burst_time: usize,
  pub execution_time: usize,
}

impl Process {
  pub fn done_executing(&self) -> bool {
    self.execution_time >= self.burst_time
  }

  pub fn record_execution(&mut self) {
    self.execution_time += 1;
  }
}

pub struct ProcessSpawner {
  current_id: usize,
}

pub fn new_spawner() -> ProcessSpawner {
  ProcessSpawner {
    current_id: 0,
  }
}

impl ProcessSpawner {
  pub fn spawn(&mut self, burst_time: usize) -> Process {
    self.current_id += 1;

    Process {
      id: self.current_id - 1,
      name: format!("proc_{:?}", self.current_id - 1),
      arrival_time: 0,
      execution_time: 0,
      burst_time: burst_time,
    }
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

