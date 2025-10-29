// src/calendar/swisseph_sys.rs
// 这个模块将包含由 bindgen 生成的 Swiss Ephemeris 库绑定
// 由 build.rs 中的 bindgen 生成

// 由于 build.rs 会生成绑定到 OUT_DIR/swisseph_bindings.rs
// 我们从那里引入生成的绑定
include!(concat!(env!("OUT_DIR"), "/swisseph_bindings.rs"));