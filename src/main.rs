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

  let scenarios = {
    (0..32).map(|_| (0..3).map(|_| SYSTEM_HZ).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()
  };

  tui.add_result(format!("Testing the FCFS scheduler"));
  for (run, scenario) in scenarios.iter().enumerate() {
    let scheduler = fcfs::new();
    let avg = run_simulation(&mut tui, scheduler, scenario.clone());
    tui.add_result(format!("#{:02}: Average waiting time: {:02.2}", run, avg));
  }

  tui.update();

  loop {}
}

fn run_simulation<S>(mut tui: &mut Tui, mut scheduler: S, mut process_list: Vec<usize>) -> f64
where S: Scheduler {
  let mut clock_tick = 0;
  let mut process_spawner = process::new_spawner();
  let mut average_waiting_time = 0f64;
  let mut num_of_spawned_procs = 1;

  tui.set_header(format!("Using algorithm: {}", scheduler.name()));
  tui.debug(format!("{:05}: Starting simulation using {}", clock_tick, scheduler.name()));

  // vectors only support the pop (from the back) operation so
  // reverse it (so we pop from the front)
  process_list.reverse();

  // Add the equivalent of the 'init' process
  scheduler.add_process(process_spawner.spawn(process_list.pop().unwrap()));

  loop {
    // update the process view
    scheduler.list_processes(&mut tui);
    // update all the views
    tui.update();

    if !scheduler.has_processes() {
      // if there's no more processes - end the simulation
      // if a process can only be created by another process - there's
      // nothing else to do
      break;
    }

    // simulate executing the current process
    tui.debug(format!("{:05}: Executing {}", clock_tick, scheduler.current_proc().unwrap().name));
    scheduler.current_proc_mut().unwrap().record_execution();

    // Simulate other processes creating new processes
    // Assume that you can't create new processes, if there's none already
    if clock_tick % SYSTEM_HZ == 0 {
      if let Some(burst_time) = process_list.pop() {
        let new_proc = process_spawner.spawn(burst_time);
        tui.debug(format!("{:05}: Spawning {}", clock_tick, new_proc.name));
        scheduler.add_process(new_proc);
        num_of_spawned_procs += 1;
      }
    }

    if scheduler.current_proc().unwrap().done_executing() {
      tui.debug(format!("{:05}: Killing process {}", clock_tick, scheduler.current_proc().unwrap().name));
      scheduler.kill_current_proc();
    }

    if scheduler.has_processes() {
      if clock_tick % SYSTEM_HZ == SYSTEM_HZ - 1 {
        tui.debug(format!("{:05}: Triggering a scheduling round", clock_tick));
        scheduler.schedule();
        tui.debug(format!("{:05}: {} was waiting {} clock ticks", clock_tick, scheduler.current_proc().unwrap().name, scheduler.current_proc().unwrap().waiting_time));
        average_waiting_time -= average_waiting_time / num_of_spawned_procs as f64;
        average_waiting_time += scheduler.current_proc().unwrap().waiting_time as f64 / num_of_spawned_procs as f64;
      }

      // increase the waiting time for every process
      scheduler.increase_waiting_times();
    }

    thread::sleep(time::Duration::from_millis(1000 / CLOCK_HZ));

    clock_tick += 1;
  }

  average_waiting_time
}

/*
 * vi: ts=2 sw=2 expandtab
 */

