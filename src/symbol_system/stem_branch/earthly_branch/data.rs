//! 地支关系常量数组模块

use super::EarthlyBranch;
use crate::basic::wuxing::WuXing;

// 相刑关系常量数组
pub const PUNISHING_PAIRS: [(EarthlyBranch, EarthlyBranch); 12] = [
    //寅刑巳
    (EarthlyBranch::Yin, EarthlyBranch::Si),
    //巳刑申
    (EarthlyBranch::Si, EarthlyBranch::Shen),
    //申刑寅
    (EarthlyBranch::Shen, EarthlyBranch::Yin),
    //丑刑戌
    (EarthlyBranch::Chou, EarthlyBranch::Xu),
    //戌刑未
    (EarthlyBranch::Xu, EarthlyBranch::Wei),
    //未刑丑
    (EarthlyBranch::Wei, EarthlyBranch::Chou),
    //子卯、辰午酉亥自刑
    (EarthlyBranch::Zi, EarthlyBranch::Mao),
    (EarthlyBranch::Mao, EarthlyBranch::Zi),
    (EarthlyBranch::Chen, EarthlyBranch::Chen),
    (EarthlyBranch::Wu, EarthlyBranch::Wu),
    (EarthlyBranch::You, EarthlyBranch::You),
    (EarthlyBranch::Hai, EarthlyBranch::Hai),
];

// 相冲关系常量数组
pub const OPPOSITION_PAIRS: [(EarthlyBranch, EarthlyBranch); 6] = [
    // 子午相冲
    (EarthlyBranch::Zi, EarthlyBranch::Wu),
    // 丑未相冲
    (EarthlyBranch::Chou, EarthlyBranch::Wei),
    // 寅申相冲
    (EarthlyBranch::Yin, EarthlyBranch::Shen),
    // 卯酉相冲
    (EarthlyBranch::Mao, EarthlyBranch::You),
    // 辰戌相冲
    (EarthlyBranch::Chen, EarthlyBranch::Xu),
    // 巳亥相冲
    (EarthlyBranch::Si, EarthlyBranch::Hai),
];

// 六合关系常量数组
pub const SIX_HARMONY_PAIRS: [(EarthlyBranch, EarthlyBranch, WuXing); 6] = [
    // 子丑合土
    (EarthlyBranch::Zi, EarthlyBranch::Chou, WuXing::Earth),
    // 寅亥合木
    (EarthlyBranch::Yin, EarthlyBranch::Hai, WuXing::Wood),
    // 卯戌合火
    (EarthlyBranch::Mao, EarthlyBranch::Xu, WuXing::Fire),
    // 辰酉合金
    (EarthlyBranch::Chen, EarthlyBranch::You, WuXing::Metal),
    // 巳申合水
    (EarthlyBranch::Si, EarthlyBranch::Shen, WuXing::Water),
    // 午未合火
    (EarthlyBranch::Wu, EarthlyBranch::Wei, WuXing::Fire),
];

// 三合关系常量数组
pub const THREE_HARMONY_GROUPS: [(EarthlyBranch, EarthlyBranch, EarthlyBranch, WuXing); 4] = [
    // 寅午戌合火
    (EarthlyBranch::Yin, EarthlyBranch::Wu, EarthlyBranch::Xu, WuXing::Fire),
    // 亥卯未合木(按照index排序)
    (EarthlyBranch::Mao, EarthlyBranch::Wei, EarthlyBranch::Hai, WuXing::Wood),
    // 巳酉丑合金(按照index排序)
    (EarthlyBranch::Chou, EarthlyBranch::Si, EarthlyBranch::You, WuXing::Metal),
    // 申子辰合水(按照index排序)
    (EarthlyBranch::Zi, EarthlyBranch::Chen, EarthlyBranch::Shen, WuXing::Water),
];

// 半合关系常量数组
pub const HALF_HARMONY_PAIRS: [(EarthlyBranch, EarthlyBranch, WuXing); 8] = [
    // 寅午半合火
    (EarthlyBranch::Yin, EarthlyBranch::Wu, WuXing::Fire),
    // 午戌半合火
    (EarthlyBranch::Wu, EarthlyBranch::Xu, WuXing::Fire),
    // 亥卯半合木
    (EarthlyBranch::Mao, EarthlyBranch::Hai, WuXing::Wood),
    // 卯未半合木
    (EarthlyBranch::Mao, EarthlyBranch::Wei, WuXing::Wood),
    // 巳酉半合金
    (EarthlyBranch::Si, EarthlyBranch::You, WuXing::Metal),
    // 酉丑半合金
    (EarthlyBranch::Chou, EarthlyBranch::You, WuXing::Metal),
    // 申子半合水
    (EarthlyBranch::Zi, EarthlyBranch::Shen, WuXing::Water),
    // 子辰半合水
    (EarthlyBranch::Zi, EarthlyBranch::Chen, WuXing::Water),
];

// 拱合关系常量数组
pub const ARCH_HARMONY_PAIRS: [(EarthlyBranch, EarthlyBranch, WuXing); 4] = [
    // 寅戌拱合火
    (EarthlyBranch::Yin, EarthlyBranch::Xu, WuXing::Fire),
    // 亥未拱合木
    (EarthlyBranch::Wei, EarthlyBranch::Hai, WuXing::Wood),
    // 巳丑拱和金
    (EarthlyBranch::Chou, EarthlyBranch::Si, WuXing::Metal),
    // 申辰拱和水
    (EarthlyBranch::Chen, EarthlyBranch::Shen, WuXing::Water),
];

// 三会关系常量数组
pub const THREE_MEETING_GROUPS: [(EarthlyBranch, EarthlyBranch, EarthlyBranch, WuXing); 4] = [
    // 寅卯辰会木
    (EarthlyBranch::Yin, EarthlyBranch::Mao, EarthlyBranch::Chen, WuXing::Wood),
    // 巳午未会火
    (EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei, WuXing::Fire),
    // 申酉戌会金
    (EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu, WuXing::Metal),
    // 亥子丑会水
    (EarthlyBranch::Hai, EarthlyBranch::Zi, EarthlyBranch::Chou, WuXing::Water),
];

// 六害关系常量数组
pub const SIX_HARM_PAIRS: [(EarthlyBranch, EarthlyBranch); 6] = [
    // 子未六害
    (EarthlyBranch::Zi, EarthlyBranch::Wei),
    // 丑午六害
    (EarthlyBranch::Chou, EarthlyBranch::Wu),
    // 寅巳六害
    (EarthlyBranch::Yin, EarthlyBranch::Si),
    // 卯辰六害
    (EarthlyBranch::Mao, EarthlyBranch::Chen),
    // 申亥六害
    (EarthlyBranch::Shen, EarthlyBranch::Hai),
    // 酉戌六害
    (EarthlyBranch::You, EarthlyBranch::Xu),
];

// 破关系常量数组
pub const BREAK_PAIRS: [(EarthlyBranch, EarthlyBranch); 6] = [
    (EarthlyBranch::Yin, EarthlyBranch::Wei),
    (EarthlyBranch::Chen, EarthlyBranch::Hai),
    (EarthlyBranch::Si, EarthlyBranch::Xu),
    (EarthlyBranch::Wu, EarthlyBranch::Chou),
    (EarthlyBranch::Shen, EarthlyBranch::Mao),
    (EarthlyBranch::You, EarthlyBranch::Zi),
];