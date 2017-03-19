//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 13 Mar 2017 21:03:41 +0100 (CET)
//

extern crate rand;
extern crate cursive;

#[cfg(feature = "go-slow")]
use std::thread;
#[cfg(feature = "go-slow")]
use std::time;

mod process;
mod scheduler;
mod fcfs;
mod round_robin;
mod tui;

use scheduler::*;
use tui::*;

/// Length of a single system clock tick (frequency of actual, real time)
#[cfg(feature = "go-slow")]
const CLOCK_HZ: u64 = 1000;
/// Switch context every <value> clock ticks
const SYSTEM_HZ: usize = 8;

fn main() {
  let mut tui = tui::new();

  let scenarios = {
    (0..20).map(|_| (0..64).map(|_| {
      let upper_limit = rand::random::<usize>() % (SYSTEM_HZ * 4) + 1;
      let burst_time = rand::random::<usize>() % upper_limit;

      burst_time + 1 // + 1 so there's no chance of having a process with 0 burst time
    }).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()
  };

  macro_rules! run_simulation_suite {
    ($scheduler:ident) => ({
      let mut average_waiting_time = 0f64;

      for (run, scenario) in scenarios.iter().enumerate() {
        let scheduler = $scheduler::new();
        let avg = run_simulation(&mut tui, scheduler, scenario.clone());
        tui.add_result(format!("{}: #{:02}: Average waiting time: {:02.2}", stringify!($scheduler), run, avg));

        average_waiting_time -= average_waiting_time / (run + 1) as f64;
        average_waiting_time += avg / (run + 1) as f64;
      }

      tui.add_result(format!("{}: Overall average: {:02.2}", stringify!($scheduler), average_waiting_time));
    })
  }

  run_simulation_suite!(fcfs);
  run_simulation_suite!(round_robin);

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

    if cfg!(feature = "go-slow") || clock_tick % (SYSTEM_HZ * 2) == 0 {
      // update all the views
      tui.update();
    }

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
        let prev_proc_name = scheduler.current_proc().unwrap().name.clone();
        scheduler.schedule();
        tui.debug(format!("{:05}: Switching context: {} -> {}", clock_tick, prev_proc_name, scheduler.current_proc().unwrap().name));
        tui.debug(format!("{:05}: {} was waiting {} clock ticks", clock_tick, scheduler.current_proc().unwrap().name, scheduler.current_proc().unwrap().waiting_time));

        average_waiting_time -= average_waiting_time / num_of_spawned_procs as f64;
        average_waiting_time += scheduler.current_proc().unwrap().waiting_time as f64 / num_of_spawned_procs as f64;
      }

      // increase the waiting time for every process
      scheduler.increase_waiting_times();
    }

    #[cfg(feature = "go-slow")]
    thread::sleep(time::Duration::from_millis(1000 / CLOCK_HZ));

    clock_tick += 1;
  }

  average_waiting_time
}

/*
 * vi: ts=2 sw=2 expandtab
 */

