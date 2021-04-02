use crate::miner::MinerEvent::{self, CloseAll, CloseMiner, StartMiner};
use regex::Regex;

pub enum MessageError {
    CommandError(String),
    RegionError(String),
    RegionNotSpecified,
    EmptyMsg,
}

pub enum Event {
    MinerEvent(MinerEvent),
    ConnectionClose,
}

pub fn classify_msg(msg: String) -> Result<Event, MessageError> {
    let mut split_message = msg.as_str().split(":");
    let regions_regex = Regex::new("^(br1|eun1|euw1|jp1|kr|la1|la2|na1|oc1|tr1|ru)$").unwrap();

    match split_message.next() {
        Some(slice) => match slice {
            "start" => match split_message.next() {
                Some(slice) => {
                    if regions_regex.is_match(slice) {
                        Ok(Event::MinerEvent(StartMiner(slice.to_owned())))
                    } else {
                        Err(MessageError::RegionError(slice.to_owned()))
                    }
                }
                None => Err(MessageError::RegionNotSpecified),
            },
            "close" => match split_message.next() {
                Some(slice) => match slice {
                    "connection" => Ok(Event::ConnectionClose),
                    "all" => Ok(Event::MinerEvent(CloseAll)),
                    _ => {
                        if regions_regex.is_match(slice) {
                            Ok(Event::MinerEvent(CloseMiner(slice.to_owned())))
                        } else {
                            Err(MessageError::RegionError(slice.to_owned()))
                        }
                    }
                },
                None => Err(MessageError::RegionNotSpecified),
            },
            _ => Err(MessageError::CommandError(slice.to_owned())),
        },
        None => Err(MessageError::EmptyMsg),
    }
}
