//! Commands Supported and Effects log.

use crate::protocol::nvme::{PAGE_SIZE, command};

/// Commands supported and effects error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid command submission and execution value.
    Submission(u8),
    /// Invalid command opcode value.
    Opcode(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Submission(x) => write!(f, "invalid command submission {x:#x}"),
            Self::Opcode(x) => write!(f, "invalid command opcode {x:#x}"),
        }
    }
}

/// Command submission and execution requirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Submission {
    /// May be submitted while other commands are executing.
    Concurrent = 0x0,
    /// Submitted only when no other command is executing on the namespace.
    NamespaceExclusive = 0x1,
    /// Submitted only when no other command is executing on any namespace.
    SubsystemExclusive = 0x2,
}

impl TryFrom<u8> for Submission {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Submission] = &[
            Submission::Concurrent,
            Submission::NamespaceExclusive,
            Submission::SubsystemExclusive,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::Submission(value))
    }
}

impl std::fmt::Display for Submission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Concurrent => write!(f, "concurrent"),
            Self::NamespaceExclusive => write!(f, "namespace exclusive"),
            Self::SubsystemExclusive => write!(f, "subsystem exclusive"),
        }
    }
}

/// Effects of executing a command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Effects {
    /// Command may change the content of logical blocks.
    pub logical_block_change: bool,
    /// Command may change the capabilities of a namespace.
    pub namespace_capability_change: bool,
    /// Command may change the namespaces of the NVM subsystem.
    pub namespace_inventory_change: bool,
    /// Command may change the capabilities of the controller.
    pub controller_capability_change: bool,
    /// Command submission and execution requirement.
    pub submission: Submission,
    /// Command supports selecting a UUID.
    pub uuid_selection: bool,
}

impl Effects {
    /// Parse effects field, absent when the command is unsupported.
    fn parse(value: u32) -> Result<Option<Self>, Error> {
        const SUPPORTED: u32 = 1 << 0;

        if (value & SUPPORTED) == 0 {
            return Ok(None);
        }

        Ok(Some(Self {
            logical_block_change: (value & (1 << 1)) != 0,
            namespace_capability_change: (value & (1 << 2)) != 0,
            namespace_inventory_change: (value & (1 << 3)) != 0,
            controller_capability_change: (value & (1 << 4)) != 0,
            submission: u8::try_from((value >> 16) & 0b111).unwrap().try_into()?,
            uuid_selection: (value & (1 << 19)) != 0,
        }))
    }
}

impl std::fmt::Display for Effects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(submission: {}, logical block: {}, namespace capability: {}, namespace inventory: \
             {}, controller capability: {}, UUID: {})",
            self.submission,
            self.logical_block_change,
            self.namespace_capability_change,
            self.namespace_inventory_change,
            self.controller_capability_change,
            self.uuid_selection
        )
    }
}

/// Supported admin command and its effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminCommand {
    /// Admin command opcode.
    pub opcode: command::AdminOpcode,
    /// Effects of executing the command.
    pub effects: Effects,
}

impl std::fmt::Display for AdminCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.opcode, self.effects)
    }
}

/// Supported I/O command and its effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IoCommand {
    /// I/O command opcode.
    pub opcode: command::IoOpcode,
    /// Effects of executing the command.
    pub effects: Effects,
}

impl std::fmt::Display for IoCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.opcode, self.effects)
    }
}

/// Commands Supported and Effects log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandEffects {
    /// Supported admin commands.
    pub admin: Box<[AdminCommand]>,
    /// Supported I/O commands.
    pub io: Box<[IoCommand]>,
}

impl CommandEffects {
    /// Size in bytes.
    pub(crate) const SIZE: usize = PAGE_SIZE;
}

impl TryFrom<&[u8; CommandEffects::SIZE]> for CommandEffects {
    type Error = super::Error;

    fn try_from(data: &[u8; CommandEffects::SIZE]) -> Result<Self, Self::Error> {
        const DWORD_SIZE: usize = size_of::<u32>();
        const ADMIN_RANGE: std::ops::Range<usize> = 0..1024;
        const IO_RANGE: std::ops::Range<usize> = 1024..2048;

        let mut admin = Vec::new();
        for (opcode, &entry) in data[ADMIN_RANGE]
            .as_chunks::<DWORD_SIZE>()
            .0
            .iter()
            .enumerate()
        {
            if let Some(effects) = Effects::parse(u32::from_le_bytes(entry))? {
                admin.push(AdminCommand {
                    opcode: u8::try_from(opcode)
                        .unwrap()
                        .try_into()
                        .map_err(Error::Opcode)?,
                    effects,
                });
            }
        }

        let mut io = Vec::new();
        for (opcode, &entry) in data[IO_RANGE]
            .as_chunks::<DWORD_SIZE>()
            .0
            .iter()
            .enumerate()
        {
            if let Some(effects) = Effects::parse(u32::from_le_bytes(entry))? {
                io.push(IoCommand {
                    opcode: u8::try_from(opcode)
                        .unwrap()
                        .try_into()
                        .map_err(Error::Opcode)?,
                    effects,
                });
            }
        }

        Ok(Self {
            admin: admin.into(),
            io: io.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn command_effects_parse() {
        const DATA_VALID: &[&[u8; CommandEffects::SIZE]] = &[test_data::patriot_p300::LOG_5H];
        const DATA_INVALID: &[&[u8; CommandEffects::SIZE]] = &[&[0xFF; _]];

        for &data in DATA_VALID {
            assert!(CommandEffects::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(CommandEffects::try_from(data).is_err());
        }
    }
}
