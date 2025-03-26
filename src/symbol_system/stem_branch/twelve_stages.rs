//! 十二长生相关类型的模块
use crate::traits::{Iter, ChineseName};
use crate::symbol_system::{HeavenlyStem, EarthlyBranch};

/// 十二长生数组
const TWELVE_STAGES: [&str; 12] = [
    "长生",
    "沐浴",
    "冠带",
    "临官",
    "帝旺",
    "衰",
    "病",
    "死",
    "墓",
    "绝",
    "胎",
    "养",
];

// 十二长生结构体
pub struct TwelveStages {
    /// 十二长生数组的索引
    index: usize,
}

impl TwelveStages  {
    // 获得帝旺天干和地支
    fn get_diwang_branch(stem: HeavenlyStem) -> EarthlyBranch {
       match stem {
           HeavenlyStem::Jia => EarthlyBranch::Mao,
           HeavenlyStem::Yi => EarthlyBranch::Yin,
           HeavenlyStem::Bing => EarthlyBranch::Wu,
           HeavenlyStem::Ding => EarthlyBranch::Si,
           HeavenlyStem::Wu => EarthlyBranch::Wu,
           HeavenlyStem::Ji => EarthlyBranch::Si,
           HeavenlyStem::Geng => EarthlyBranch::You,
           HeavenlyStem::Xin => EarthlyBranch::Shen,
           HeavenlyStem::Ren => EarthlyBranch::Zi,
           HeavenlyStem::Gui => EarthlyBranch::Hai,
       } 
    }
    fn get_diwang_stem(branch: EarthlyBranch) -> Option<Vec<HeavenlyStem>> {
        match branch {
            EarthlyBranch::Mao =>  Some(vec![HeavenlyStem::Jia]),
            EarthlyBranch::Yin =>  Some(vec![HeavenlyStem::Yi]),
            EarthlyBranch::Wu =>   Some(vec![HeavenlyStem::Bing, HeavenlyStem::Wu]),
            EarthlyBranch::Si =>   Some(vec![HeavenlyStem::Ding, HeavenlyStem::Ji]),
            EarthlyBranch::You =>  Some(vec![HeavenlyStem::Geng]),
            EarthlyBranch::Shen => Some(vec![HeavenlyStem::Xin]),
            EarthlyBranch::Zi =>   Some(vec![HeavenlyStem::Ren]),
            EarthlyBranch::Hai =>  Some(vec![HeavenlyStem::Gui]),
            _ => None,
        }
    }
    // 获得天干在地支关系的十二长生
    pub fn get_twelve_stages(stem: HeavenlyStem, branch: EarthlyBranch) -> Self {
        let diwang_branch = Self::get_diwang_branch(stem);
        let mut index = 0;
        let mut current_branch = diwang_branch;
        while current_branch != branch {
            current_branch = current_branch.next();
            index += 1;
            if index >= 12 {
                break;
            }
        }
        Self { index }
    }
    // 根据天干和十二长生获得地支
    pub fn get_earthly_branch(&self, stem: HeavenlyStem) -> EarthlyBranch {
        let diwang_branch = Self::get_diwang_branch(stem);
        let mut current_branch = diwang_branch;
        for _ in 0..self.index {
            current_branch = current_branch.next();
        }
        current_branch
    }
    // 根据地支和十二长生获得天干
    pub fn get_heavenly_stem(&self, branch: EarthlyBranch) -> HeavenlyStem {
        let stems = Self::get_diwang_stem(branch).unwrap_or_default();
        if stems.is_empty() {
            return HeavenlyStem::Jia;
        }
        stems[0]
    }
    pub fn from_chinese_name(chinese_name: &str) -> Self {
        let index = TWELVE_STAGES.iter()
            .position(|&name| name == chinese_name)
            .unwrap_or(0);
        Self { index }
    }
}


impl ChineseName for TwelveStages {
    fn chinese_name(&self) -> &'static str {
        TWELVE_STAGES[self.index]
    }
}

impl Iter for TwelveStages {
    type Item = Self;

    fn next(&self) -> Self::Item {
        // 如果是最后一个，则返回第一个
        if self.index == TWELVE_STAGES.len() - 1 {
            Self { index: 0 }
        } else {
            // 否则返回下一个
            Self {
                index: self.index + 1,
            }
        }
    }
    fn prev(&self) -> Self::Item {
        // 如果是第一个，则返回最后一个
        if self.index == 0 {
            Self {
                index: TWELVE_STAGES.len() - 1,
            }  
        } else {
            // 否则返回上一个
            Self {
                index: self.index - 1,
            }
        }
    }

}