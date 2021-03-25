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
    MinerNotStarted,
}

pub fn make_event(e: MinerEvent, miners_processes: &mut Vec<MinerProcess>) -> Result<(), MinerErr> {
    match e {
        MinerEvent::StartMiner(region) => open_miner(region, miners_processes),

        MinerEvent::CloseMiner(region) => close_miner(region, miners_processes),

        MinerEvent::CloseAll => close_all_miners(miners_processes),
    }
}

fn close_miner(region: String, miners_processes: &mut Vec<MinerProcess>) -> Result<(), MinerErr> {
    for index in 0..miners_processes.len() {
        if miners_processes[index].region == region {
            miners_processes.remove(index);
            return Ok(());
        }
    }

    Err(MinerErr::MinerNotStarted)
}

fn close_all_miners(miners_processes: &mut Vec<MinerProcess>) -> Result<(), MinerErr> {
    miners_processes.clear();
    Ok(())
}

fn open_miner(region: String, miners_processes: &mut Vec<MinerProcess>) -> Result<(), MinerErr> {
    for process in miners_processes.iter() {
        if process.region == region {
            return Err(MinerErr::MinerAlreadyExist);
        }
    }

    let process = Command::new("lol-project")
        .arg(region.clone())
        .spawn()
        .unwrap();

    miners_processes.push(MinerProcess { region, process });

    Ok(())
}

pub fn miners_resources_usage(miners_processes: &Vec<MinerProcess>) -> Vec<MinerInfo> {
    unimplemented!()
}
