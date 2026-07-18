// 窗口与标题栏配置 —— 修改窗口外观只改这里

/// 窗口与标题栏配置集合
#[allow(dead_code)]
pub struct WindowConfig {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
    /// 明亮模式标题栏背景色（COLORREF: 0x00BBGGRR）
    pub light_caption_color: u32,
    /// 明亮模式标题栏文字色
    pub light_text_color: u32,
    /// 黑暗模式标题栏背景色
    pub dark_caption_color: u32,
    /// 黑暗模式标题栏文字色
    pub dark_text_color: u32,
    /// 窗口图标路径（相对 src/）
    pub icon_path: &'static str,
}

pub const WINDOW_CONFIG: WindowConfig = WindowConfig {
    title: "TinyTransfer",
    width: 1100,
    height: 780,
    min_width: 800,
    min_height: 600,
    light_caption_color: 0x00FAFBFB, // #FBFBFA 明亮模式背景
    light_text_color: 0x00000000,    // 黑色文字
    dark_caption_color: 0x000F0F0F,  // #0F0F0F 黑暗模式背景
    dark_text_color: 0x00FFFFFF,     // 白色文字
    icon_path: "../icons/128x128@2x.png",
};
