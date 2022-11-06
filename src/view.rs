use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

///#画面
pub struct GUI(pub Table);
#[derive(Debug)]
pub struct Grade<Rx: ToString> {
    //列表
    pub explain: Vec<Rx>,
    //内容
    pub output: Vec<Vec<Rx>>,
}
///#风格
pub enum Colour {
    ///#[Frames::]
    Error,
}
///#画面数据
pub struct Frames {
    //文本
    text: Attribute,
    //单元格前景色
    frames: Color,
    //背景颜色
    background: Color,
}

impl Colour {
    ///#构建
    pub fn build(&self) -> Frames {
        return match self {
            Colour::Error => Frames {
                text: Attribute::SlowBlink,
                frames: Color::DarkRed,
                background: Color::Black,
            },
        };
    }
}
impl From<(Colour, Grade<&str>)> for GUI {
    ///#fn from(value: (Colour, Grade<&str>)) -> Self
    ///#[view::Colour][view::Grade]
    fn from(value: (Colour, Grade<&str>)) -> Self {
        let e = value.0.build();
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(
                value
                    .1
                    .explain
                    .into_iter()
                    .map(|x| -> Cell {
                        Cell::new(x)
                            .add_attribute(e.text)
                            .fg(e.frames)
                            .bg(e.background)
                    })
                    .collect::<Vec<_>>(),
            );
        value.1.output.into_iter().for_each(|r| {
            table.add_row(
                r.into_iter()
                    .map(|x| -> Cell {
                        Cell::new(x)
                            .add_attribute(e.text)
                            .fg(e.frames)
                            .bg(e.background)
                    })
                    .collect::<Vec<_>>(),
            );
        });
        return GUI(table);
    }
}
impl Deref for GUI {
    type Target = Table;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for GUI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl<Rx: Sized> AsRef<Rx> for GUI
where
    <GUI as Deref>::Target: AsRef<Rx>,
{
    fn as_ref(&self) -> &Rx {
        return self.deref().as_ref();
    }
}
impl<Rx: Sized> AsMut<Rx> for GUI
where
    <GUI as Deref>::Target: AsMut<Rx>,
{
    fn as_mut(&mut self) -> &mut Rx {
        return self.deref_mut().as_mut();
    }
}
