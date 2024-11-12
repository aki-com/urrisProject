use iced::{
    Application,Element, executor, Length, Color, Background
};
use iced::widget::{Column, Row, Container};

struct TetrisApp {
    field: Vec<Vec<u8>>,  // 0: 空, 1: ブロック
}

impl TetrisApp {
    fn new() -> Self {
        Self {
            field: vec![vec![0; 10]; 20], // 20x10のフィールド
        }
    }
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for TetrisApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Tetris App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut col = Column::new().spacing(2);
        for row in &self.field {
            let mut r = Row::new().spacing(2);
            for &cell in row {
                let color = if cell == 0 {
                    Color::from_rgb(0.8, 0.8, 0.8) // 灰色
                } else {
                    Color::from_rgb(0.0, 0.0, 1.0) // 青色
                };

                // Containerのスタイルにcolorを適用
                r = r.push(
                    Container::new(Text::new(""))  // 空のテキストを表示
                        .width(Length::Units(20))
                        .height(Length::Units(20))
                        .style(iced::container::Style {
                            background: Background::Color(color),
                            ..Default::default()
                        }),
                );
            }
            col = col.push(r);
        }
        Container::new(col).into()
    }
}

fn main() {
    TetrisApp::run(iced::Settings::default());
}
