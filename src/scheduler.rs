//
// scheduler.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 13 Mar 2017 21:03:41 +0100 (CET)
//

use cursive::Cursive;

use process::*;

pub trait Scheduler {
  fn schedule(&mut self);
  fn has_processes(&self) -> bool;
  fn add_process(&mut self, Process);
  fn current_proc(&self) -> Option<&Process>;
  fn current_proc_mut(&mut self) -> Option<&mut Process>;
  fn kill_current_proc(&mut self);
  fn list_processes(&self, &mut Cursive);
}

/*
 * vi: ts=2 sw=2 expandtab
 */

