use serde::{Deserialize, Serialize, Serializer};

use crate::{error::ProtocolError, types::Serialize as Ser};

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Chat {
    #[serde(flatten)]
    pub component: Component,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub font: Option<Font>,
    pub color: Option<Color>,
    pub insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    pub click_event: Option<ClickEvent>,
    #[serde(rename = "hoverEvent")]
    pub hover_event: Option<HoverEvent>,
    pub extra: Option<Vec<Chat>>,
}

impl Ser for Chat {
    fn serialize(&self, buf: &mut bytes::BytesMut) {
        let s = serde_json::to_string(&self).unwrap();
        Ser::serialize(&s, buf);
    }

    fn deserialize(buf: &mut bytes::Bytes) -> crate::error::Res<Self>
    where
        Self: Sized,
    {
        let s: String = Ser::deserialize(buf)?;

        serde_json::from_str(&s).map_err(ProtocolError::JsonError)
    }
}

impl Chat {
    pub fn with_component(mut self, component: Component) -> Self {
        self.component = component;
        self
    }

    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    pub fn with_underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn with_obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }

    pub fn with_font(mut self, font: Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_insertion(mut self, insertion: String) -> Self {
        self.insertion = Some(insertion);
        self
    }

    pub fn with_click_event(mut self, click_event: ClickEvent) -> Self {
        self.click_event = Some(click_event);
        self
    }

    pub fn with_hover_event(mut self, hover_event: HoverEvent) -> Self {
        self.hover_event = Some(hover_event);
        self
    }

    pub fn with_extra(mut self, extra: Chat) -> Self {
        if self.extra.is_none() {
            self.extra = Some(vec![]);
        }

        if let Some(v) = self.extra.as_mut() {
            v.push(extra);
        }

        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Component {
    String(TextComponent),
    Translation {
        translate: String,
        with: Option<Vec<Component>>,
    },
    Keybind(KeybindComponent),
    // TODO: find out what type value has
    // Score {
    //     name: String,
    //     objective: String,
    //     value: !,
    // },
    // TODO: Selector
}

impl Default for Component {
    fn default() -> Self {
        Self::String(TextComponent {
            text: String::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextComponent {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KeybindComponent {
    pub keybind: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Font {
    #[serde(rename = "minecraft:uniform")]
    Uniform,
    #[serde(rename = "minecraft:alt")]
    Alt,
    #[serde(rename = "minecraft:default")]
    Default,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(untagged)]
pub enum Color {
    NormalColor(NormalColor),
    FormatCode(ColorFormatCode),
    #[serde(serialize_with = "web_color_serialize")]
    #[serde(deserialize_with = "web_color_deserialize")]
    WebColor(u8, u8, u8),
}

fn web_color_serialize<S>(r: &u8, g: &u8, b: &u8, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("#{r:02x}{g:02x}{b:02x}"))
}

struct WebColorVisitor;
impl<'de> serde::de::Visitor<'de> for WebColorVisitor {
    type Value = (u8, u8, u8);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing a color formatted as a hex triplet")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.starts_with('#') && v[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            let r;
            let g;
            let b;

            if v.len() == 7 {
                r = u8::from_str_radix(&v[1..3], 16).unwrap();
                g = u8::from_str_radix(&v[3..5], 16).map_err(E::custom)?;
                b = u8::from_str_radix(&v[5..7], 16).map_err(E::custom)?;
            } else if v.len() == 4 {
                r = u8::from_str_radix(&v[1..2], 16).unwrap();
                g = u8::from_str_radix(&v[2..3], 16).map_err(E::custom)?;
                b = u8::from_str_radix(&v[3..4], 16).map_err(E::custom)?;
            } else {
                return Err(E::custom(
                    "a web color string may only be 4 or 7 chars long",
                ));
            }

            Ok((r, g, b))
        } else {
            Err(E::custom(
                "web color needs to start with '#' and may only be ascii hex numbers",
            ))
        }
    }
}

fn web_color_deserialize<'de, D>(d: D) -> Result<(u8, u8, u8), D::Error>
where
    D: serde::Deserializer<'de>,
{
    d.deserialize_str(WebColorVisitor)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NormalColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    BrightGreen,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ColorFormatCode {
    #[serde(rename = "§0")]
    Black,
    #[serde(rename = "§1")]
    DarkBlue,
    #[serde(rename = "§2")]
    DarkGreen,
    #[serde(rename = "§3")]
    DarkCyan,
    #[serde(rename = "§4")]
    DarkRed,
    #[serde(rename = "§5")]
    Purple,
    #[serde(rename = "§6")]
    Gold,
    #[serde(rename = "§7")]
    Gray,
    #[serde(rename = "§8")]
    DarkGray,
    #[serde(rename = "§9")]
    Blue,
    #[serde(rename = "§a")]
    BrightGreen,
    #[serde(rename = "§b")]
    Cyan,
    #[serde(rename = "§c")]
    Red,
    #[serde(rename = "§d")]
    Pink,
    #[serde(rename = "§e")]
    Yellow,
    #[serde(rename = "§f")]
    White,
    #[serde(rename = "§k")]
    Obfuscated,
    #[serde(rename = "§l")]
    Bold,
    #[serde(rename = "§m")]
    Strikethrough,
    #[serde(rename = "§n")]
    Underline,
    #[serde(rename = "§o")]
    Italic,
    #[serde(rename = "§r")]
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClickEvent {
    pub action: ClickEventAction,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventAction {
    OpenUrl,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoverEvent {
    pub action: HoverEventAction,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HoverEventAction {
    ShowText,
    ShowItem,
    ShowEntity,
}
