pub mod help;
pub mod join;
pub mod leave;
pub mod play;
pub mod queue;
pub mod shuffle;
pub mod skip;

use anyhow::Result;
use songbird::{input::AuxMetadata, typemap::TypeMapKey};
use strum::EnumIter;

use std::str::FromStr;

use crate::handler::MessageContext;

use self::{
    help::Help, join::Join, leave::Leave, play::Play, queue::Queue, shuffle::Shuffle, skip::Skip,
};

#[derive(Debug, PartialEq, EnumIter)]
pub enum AvailableCommands {
    Help,
    Join,
    Play,
    Queue,
    Shuffle,
    Skip,
    Leave,
}

impl FromStr for AvailableCommands {
    type Err = String;

    /// This converts a string of a discord command into the 'AvailableCommands' enum
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-help" => Ok(Self::Help),
            "-join" => Ok(Self::Join),
            "-play" => Ok(Self::Play),
            "-queue" => Ok(Self::Queue),
            "-shuffle" => Ok(Self::Shuffle),
            "-skip" => Ok(Self::Skip),
            "-leave" => Ok(Self::Leave),
            _ => Err(format!("parsed command not implemented: {}", s).to_owned()),
        }
    }
}

pub trait Command {
    async fn call(ctx: MessageContext) -> Result<()>;
    fn description() -> String;
}

impl AvailableCommands {
    pub fn get_description(&self) -> String {
        match self {
            AvailableCommands::Help => Help::description(),
            AvailableCommands::Join => Join::description(),
            AvailableCommands::Play => Play::description(),
            AvailableCommands::Queue => Queue::description(),
            AvailableCommands::Shuffle => Shuffle::description(),
            AvailableCommands::Skip => Skip::description(),
            AvailableCommands::Leave => Leave::description(),
        }
    }

    pub async fn call_func(&self, ctx: MessageContext) -> Result<()> {
        match self {
            AvailableCommands::Help => Help::call(ctx).await,
            AvailableCommands::Join => Join::call(ctx).await,
            AvailableCommands::Play => Play::call(ctx).await,
            AvailableCommands::Queue => Queue::call(ctx).await,
            AvailableCommands::Shuffle => Shuffle::call(ctx).await,
            AvailableCommands::Skip => Skip::call(ctx).await,
            AvailableCommands::Leave => Leave::call(ctx).await,
        }
    }
}

pub struct TrackMetadata;

impl TypeMapKey for TrackMetadata {
    type Value = AuxMetadata;
}
