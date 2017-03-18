//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 13 Mar 2017 21:03:41 +0100 (CET)
//

extern crate rand;
extern crate cursive;

use std::thread;
use std::time;

mod process;
mod scheduler;
mod fcfs;
mod tui;

use scheduler::*;
use tui::*;

/// Length of a single system clock tick (frequency of actual, real time)
const CLOCK_HZ: u64 = 50;
/// Switch context every <value> clock ticks
const SYSTEM_HZ: usize = 8;

fn main() {
  let mut tui = tui::new();

  run_simulation(&mut tui, fcfs::new(), vec![8, 8, 8, 8, 8, 8, 8, 64]);
  run_simulation(&mut tui, fcfs::new(), vec![64, 8, 8, 8, 8, 8, 8, 8]);
  run_simulation(&mut tui, fcfs::new(), vec![8, 8, 8, 32, 32, 6, 8, 8]);

  loop {}
}

fn run_simulation<S>(mut tui: &mut Tui, mut scheduler: S, mut process_list: Vec<usize>)
where S: Scheduler {
  let mut clock_tick = 0;
  let mut process_spawner = process::new_spawner();

  tui.set_header(format!("Using algorithm: {}", "TODO"));

  // vectors only support the pop (from the back) operation so
  // reverse it (so we pop from the front)
  process_list.reverse();

  // Add the equivalent of the 'init' process
  scheduler.add_process(process_spawner.spawn(process_list.pop().unwrap()));

  loop {
    // Simulate other processes creating new processes
    // Assume that you can't create new processes, if there's none already
    if scheduler.has_processes() {
      if clock_tick % SYSTEM_HZ == 0 {
        if let Some(burst_time) = process_list.pop() {
          scheduler.add_process(process_spawner.spawn(burst_time));
        }
      }
    }

    if scheduler.has_processes() {
      if scheduler.current_proc().unwrap().done_executing() {
        scheduler.kill_current_proc();
      }
    }

    // update all the views
    tui.update();

    if !scheduler.has_processes() {
      // if there's no more processes - end the simulation
      // if a process can only be created by another process - there's
      // nothing else to do
      break;
    }

    if clock_tick % SYSTEM_HZ == 0 {
      scheduler.schedule();
    }

    // simulate executing the current process
    scheduler.current_proc_mut().unwrap().record_execution();

    // update the process view
    scheduler.list_processes(&mut tui);

    thread::sleep(time::Duration::from_millis(1000 / CLOCK_HZ));

    clock_tick += 1;
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

