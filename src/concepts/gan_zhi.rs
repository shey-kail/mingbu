use serde::Serialize;
use crate::concepts::yinyang::YinYang;
use crate::concepts::wu_xing::WuXing;
use crate::concepts::traits::{YinYangTrait, WuXingTrait, ChineseName, Index, Iter};

/// 天干枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HeavenlyStem {
    /// 甲
    Jia,
    /// 乙
    Yi,
    /// 丙
    Bing,
    /// 丁
    Ding,
    /// 戊
    Wu,
    /// 己
    Ji,
    /// 庚
    Geng,
    /// 辛
    Xin,
    /// 壬
    Ren,
    /// 癸
    Gui,
}

impl YinYangTrait for HeavenlyStem {
    fn yinyang(&self) -> YinYang {
        match self {
            HeavenlyStem::Jia | HeavenlyStem::Bing | HeavenlyStem::Wu | 
            HeavenlyStem::Geng | HeavenlyStem::Ren => YinYang::Yang,
            HeavenlyStem::Yi | HeavenlyStem::Ding | HeavenlyStem::Ji | 
            HeavenlyStem::Xin | HeavenlyStem::Gui => YinYang::Yin,
        }
    }
}

impl WuXingTrait for HeavenlyStem {
    fn wuxing(&self) -> WuXing {
        match self {
            HeavenlyStem::Jia | HeavenlyStem::Yi => WuXing::Wood,
            HeavenlyStem::Bing | HeavenlyStem::Ding => WuXing::Fire,
            HeavenlyStem::Wu | HeavenlyStem::Ji => WuXing::Earth,
            HeavenlyStem::Geng | HeavenlyStem::Xin => WuXing::Metal,
            HeavenlyStem::Ren | HeavenlyStem::Gui => WuXing::Water,
        }
    } 
}

impl ChineseName for HeavenlyStem {
    fn chinese_name(&self) -> &'static str {
        match self {
            HeavenlyStem::Jia => "甲",
            HeavenlyStem::Yi => "乙",
            HeavenlyStem::Bing => "丙",
            HeavenlyStem::Ding => "丁",
            HeavenlyStem::Wu => "戊",
            HeavenlyStem::Ji => "己",
            HeavenlyStem::Geng => "庚",
            HeavenlyStem::Xin => "辛",
            HeavenlyStem::Ren => "壬",
            HeavenlyStem::Gui => "癸",
        }
    }
}

impl Index for HeavenlyStem {
    fn from_index(index: usize) -> Self {
        match (index - 1) % 10 {
            0 => HeavenlyStem::Jia,
            1 => HeavenlyStem::Yi,
            2 => HeavenlyStem::Bing,
            3 => HeavenlyStem::Ding,
            4 => HeavenlyStem::Wu,
            5 => HeavenlyStem::Ji,
            6 => HeavenlyStem::Geng,
            7 => HeavenlyStem::Xin,
            8 => HeavenlyStem::Ren,
            9 => HeavenlyStem::Gui,
            _ => unreachable!(),
        }
    }

    fn index(&self) -> usize {
        match self {
            HeavenlyStem::Jia => 1,
            HeavenlyStem::Yi => 2,
            HeavenlyStem::Bing => 3,
            HeavenlyStem::Ding => 4,
            HeavenlyStem::Wu => 5,
            HeavenlyStem::Ji => 6,
            HeavenlyStem::Geng => 7,
            HeavenlyStem::Xin => 8,
            HeavenlyStem::Ren => 9,
            HeavenlyStem::Gui => 10,
        }
    }
}

impl Iter for HeavenlyStem {
    type Item = Self;

    fn next(&self) -> Self::Item {
        Self::from_index(self.index() + 1)
    }

    fn prev(&self) -> Self::Item {
        Self::from_index(self.index() + 9)
    }
}

/// 地支枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EarthlyBranch {
    /// 子
    Zi,
    /// 丑
    Chou,
    /// 寅
    Yin,
    /// 卯
    Mao,
    /// 辰
    Chen,
    /// 巳
    Si,
    /// 午
    Wu,
    /// 未
    Wei,
    /// 申
    Shen,
    /// 酉
    You,
    /// 戌
    Xu,
    /// 亥
    Hai,
}

impl YinYangTrait for EarthlyBranch {
    fn yinyang(&self) -> YinYang {
        match self {
            EarthlyBranch::Zi | EarthlyBranch::Wu | EarthlyBranch::Chen | 
            EarthlyBranch::You | EarthlyBranch::Shen | EarthlyBranch::Yin => YinYang::Yang,
            EarthlyBranch::Chou | EarthlyBranch::Wei | EarthlyBranch::Si | 
            EarthlyBranch::Hai | EarthlyBranch::Mao | EarthlyBranch::Xu => YinYang::Yin,
        }
    }
}

impl WuXingTrait for EarthlyBranch {
    fn wuxing(&self) -> WuXing {
        match self {
            EarthlyBranch::Yin | EarthlyBranch::Mao => WuXing::Wood,
            EarthlyBranch::Si | EarthlyBranch::Wu => WuXing::Fire,
            EarthlyBranch::Chen | EarthlyBranch::Xu | 
            EarthlyBranch::Chou | EarthlyBranch::Wei => WuXing::Earth,
            EarthlyBranch::Shen | EarthlyBranch::You => WuXing::Metal,
            EarthlyBranch::Hai | EarthlyBranch::Zi => WuXing::Water,
        }
    }
}

impl ChineseName for EarthlyBranch {
    fn chinese_name(&self) -> &'static str {
        match self {
            EarthlyBranch::Zi => "子",
            EarthlyBranch::Chou => "丑",
            EarthlyBranch::Yin => "寅",
            EarthlyBranch::Mao => "卯",
            EarthlyBranch::Chen => "辰",
            EarthlyBranch::Si => "巳",
            EarthlyBranch::Wu => "午",
            EarthlyBranch::Wei => "未",
            EarthlyBranch::Shen => "申",
            EarthlyBranch::You => "酉",
            EarthlyBranch::Xu => "戌",
            EarthlyBranch::Hai => "亥",
        }
    }
}

impl Index for EarthlyBranch {
    fn from_index(index: usize) -> Self {
        match (index - 1) % 12 {
            0 => EarthlyBranch::Zi,
            1 => EarthlyBranch::Chou,
            2 => EarthlyBranch::Yin,
            3 => EarthlyBranch::Mao,
            4 => EarthlyBranch::Chen,
            5 => EarthlyBranch::Si,
            6 => EarthlyBranch::Wu,
            7 => EarthlyBranch::Wei,
            8 => EarthlyBranch::Shen,
            9 => EarthlyBranch::You,
            10 => EarthlyBranch::Xu,
            11 => EarthlyBranch::Hai,
            _ => unreachable!(),
        }
    }
    fn index(&self) -> usize {
        match self {
            EarthlyBranch::Zi => 1,
            EarthlyBranch::Chou => 2,
            EarthlyBranch::Yin => 3,
            EarthlyBranch::Mao => 4,
            EarthlyBranch::Chen => 5,
            EarthlyBranch::Si => 6,
            EarthlyBranch::Wu => 7,
            EarthlyBranch::Wei => 8,
            EarthlyBranch::Shen => 9,
            EarthlyBranch::You => 10,
            EarthlyBranch::Xu => 11,
            EarthlyBranch::Hai => 12,
        }
    }
}

impl Iter for EarthlyBranch {
    type Item = EarthlyBranch;
    fn next(&self) -> Self::Item {
        Self::from_index(self.index() + 1)
    }
    fn prev(&self) -> Self::Item {
        Self::from_index(self.index() + 11)
    }
}

/// 六十甲子结构体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct GanZhi {
    /// 天干
    pub stem: HeavenlyStem,
    /// 地支
    pub branch: EarthlyBranch,
}

// 创建纳音数组
// 甲子乙丑海中金 丙寅丁卯炉中火 戊辰己巳大林木
// 庚午辛未路旁土 壅申癸酉剑锋金 甲戌乙亥山头火
// 丙子丁丑涧下水 戊寅己卯城头土 庚辰辛巳白蜡金
// 壬午癸未杨柳木 甲申乙酉泉中水 丙戌丁亥屋上土
// 戊子己丑霹雳火 庚寅辛卯松柏木 壬辰癸巳长流水
// 甲午乙未沙中金 丙申丁酉山下火 戊戌己亥平地木
// 庚子辛丑壁上土 壬寅癸卯金箔金 甲辰乙巳覆灯火
// 丙午丁未天河水 戊申己酉大驿土 庚戌辛亥钗钏金
// 壬子癸丑桑柘木 甲寅乙卯大溪水 丙辰丁巳沙中土
// 戊午己未天上火 庚申辛酉石榴木 壬戌癸亥大海水
const SOUNDS: [(&str, WuXing); 30] = [
    ("海中金", WuXing::Metal), ("炉中火", WuXing::Fire), ("大林木", WuXing::Wood),
    ("路旁土", WuXing::Earth), ("剑锋金", WuXing::Metal), ("山头火", WuXing::Fire),
    ("涧下水", WuXing::Water), ("城头土", WuXing::Earth), ("白蜡金", WuXing::Metal),
    ("杨柳木", WuXing::Wood), ("泉中水", WuXing::Water), ("屋上土", WuXing::Earth),
    ("霹雳火", WuXing::Fire), ("松柏木", WuXing::Wood), ("长流水", WuXing::Water),
    ("沙中金", WuXing::Metal), ("山下火", WuXing::Fire), ("平地木", WuXing::Wood),
    ("壁上土", WuXing::Earth), ("金箔金", WuXing::Metal), ("覆灯火", WuXing::Fire),
    ("天河水", WuXing::Water), ("大驿土", WuXing::Earth), ("钗钏金", WuXing::Metal),
    ("桑柘木", WuXing::Wood), ("大溪水", WuXing::Water), ("沙中土", WuXing::Earth),
    ("天上火", WuXing::Fire), ("石榴木", WuXing::Wood), ("大海水", WuXing::Water),
];
const SEXAGESIMAL_CYCLE: [&str; 60] = [
    "甲子", "乙丑", "丙寅", "丁卯", "戊辰", "己巳", "庚午", "辛未", "壬申", "癸酉",
    "甲戌", "乙亥", "丙子", "丁丑", "戊寅", "己卯", "庚辰", "辛巳", "壬午", "癸未",
    "甲申", "乙酉", "丙戌", "丁亥", "戊子", "己丑", "庚寅", "辛卯", "壬辰", "癸巳",
    "甲午", "乙未", "丙申", "丁酉", "戊戌", "己亥", "庚子", "辛丑", "壬寅", "癸卯",
    "甲辰", "乙巳", "丙午", "丁未", "戊申", "己酉", "庚戌", "辛亥", "壬子", "癸丑",
    "甲寅", "乙卯", "丙辰", "丁巳", "戊午", "己未", "庚申", "辛酉", "壬戌", "癸亥",
];

impl Index for GanZhi {
    // 根据天干index和地支index，计算出六十甲子的index
    // 计算模型：
    // 定义天干的index为h，地支的index为e，六十甲子的index为s，h-e为x
    // 当x<0时，则让x+12
    // 当x>=0时，s=h + x * 5
    fn index(&self) -> usize {
        let h = self.stem.index();
        let e = self.branch.index();
        let x = if h < e { 12 + h - e } else { h - e };
        h + x * 5
    }
    // 根据六十甲子的index，计算出天干index和地支index
    // 计算模型：
    // 定义天干的index为h，地支的index为e，六十甲子的index为s
    // h = s % 10，
    // e = s % 12
    fn from_index(index: usize) -> Self {
        let h = index % 10;
        let e = index % 12;
        GanZhi::new(
            HeavenlyStem::from_index(h), 
            EarthlyBranch::from_index(e)
        ).unwrap()
    }
}

impl Iter for GanZhi {
    type Item = Self;

    fn next(&self) -> Self::Item {
        Self::from_index(self.index() + 1)
    }

    fn prev(&self) -> Self::Item {
        Self::from_index(self.index() + 59)
    }
}

impl ChineseName for GanZhi {
    fn chinese_name(&self) -> &'static str {
        // 因为SEXAGESIMAL_CYCLE数组早就规定好而且是常量，所以没有必要进行长度检查
        // 因为index是从1开始的，所以需要减1
        SEXAGESIMAL_CYCLE[self.index() - 1]
    }
}

impl GanZhi {
    /// 创建一个新的六十甲子实例
    pub fn new(stem: HeavenlyStem, branch: EarthlyBranch) -> Result<Self, &'static str> {
        if stem.yinyang() != branch.yinyang() {
            return Err("天干地支的阴阳属性不匹配");
        }
        Ok(Self { stem, branch })
    }

    /// 获取天干
    pub fn stem(&self) -> &HeavenlyStem {
        &(self.stem)
    }

    /// 获取地支
    pub fn branch(&self) -> &EarthlyBranch {
        &(self.branch)
    }

    /// 获取纳音名称
    pub fn sound(&self) -> (&str, WuXing) {
        let index = self.index();
        // sound_index是index向上整除2
        let sound_index = (index + 1) / 2;
        // 因为SOUNDS数组早就规定好而且是常量，所以没有必要进行长度检查
        SOUNDS[sound_index - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gan_zhi_serialization() {
        let gz = GanZhi::new(HeavenlyStem::Jia, EarthlyBranch::Zi).unwrap();
        let json = serde_json::to_string(&gz).unwrap();
        // 序列化结果可能不是简单数组，而是包含字段的对象
        assert!(json.contains("stem"));
        assert!(json.contains("branch"));
    }
}
