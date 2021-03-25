use std::process::{Child, Command};
use sysinfo::{ProcessExt, System, SystemExt};

pub struct MinerProcess {
    region: String,
    process: Child,
}

pub struct MinerInfo {
    region: String,
    memory: u64,
    cpu: f32,
}

pub enum MinerEvent {
    StartMiner(String),
    CloseAll,
    CloseMiner(String),
}

pub enum MinerErr {
    MinerAlreadyExist,
    MinerNotFound,
}

pub fn make_event(e: MinerEvent, miners_processes: &mut Vec<MinerProcess>) -> Result<(), MinerErr> {
    match e {
        MinerEvent::StartMiner(region) => {}
        MinerEvent::CloseMiner(region) => {}
        MinerEvent::CloseAll => {}
    }
    Ok(())
}

fn close_miner(region: String, miners_processes: &mut Vec<MinerProcess>) {}

fn close_all_miners(miners_processes: &mut Vec<MinerProcess>) {}

fn open_miner(region: String, miners_processes: &mut Vec<MinerProcess>) {
    for process in miners_processes {}
}

pub fn miners_resources_usage(miners_processes: &Vec<MinerProcess>) -> Vec<MinerInfo> {
    unimplemented!()
}
