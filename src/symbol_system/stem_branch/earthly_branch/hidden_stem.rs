//! 地支藏干相关类型的模块

use super::{EarthlyBranch, super::HeavenlyStem};
use super::data::HIDDEN_STEM_PAIRS;

impl EarthlyBranch {
    /// 获取地支所藏天干
    pub fn hidden_stems(&self) -> Vec<HeavenlyStem> {
        HIDDEN_STEM_PAIRS
            .iter()
            .find(|(branch, _)| branch == self)
            .map(|(_, (first, second, third))| {
                let mut stems = vec![*first];
                if let Some(second_stem) = second {
                    stems.push(*second_stem);
                }
                if let Some(third_stem) = third {
                    stems.push(*third_stem);
                }
                stems
            })
            .unwrap_or_default()
    }
    // 获得主气
    pub fn main_stem(&self) -> HeavenlyStem {
        self.hidden_stems()[0].clone() 
    }
    // 获得中气
    pub fn middle_stem(&self) -> Option<HeavenlyStem> {
        self.hidden_stems().get(1).cloned()
    }
    // 获得余气
    pub fn residual_stem(&self) -> Option<HeavenlyStem> {
        self.hidden_stems().get(2).cloned()
    }
}
