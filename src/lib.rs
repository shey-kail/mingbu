//! 命理学库的主要入口

mod basic;
mod symbol_system;
mod traits;

pub use basic::{WuXing, YinYang};
pub use symbol_system::{EarthlyBranch, HeavenlyStem, Trigram};
pub use traits::{ChineseName, Index, Iter, Relationship, TripleRelationship};
