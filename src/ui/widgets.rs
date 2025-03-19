//! Custom widgets for Ice

use iced::Alignment::Center;
use iced::alignment::Horizontal::Right;
use iced::widget::{
    center, column, container, horizontal_space, mouse_area, opaque, row, stack, text,
    vertical_space,
};
use iced::{Color, Element, Length};

#[derive(Clone)]
pub struct KeyValueView {
    /// Data to be displayed in the widget
    ///
    /// (Key, Value, Option<Units>)
    kv_store: Vec<(String, String, Option<String>)>,
}

impl KeyValueView {
    pub fn new() -> Self {
        Self {
            kv_store: Vec::new(),
        }
    }

    pub fn add_item<K, V>(&mut self, key: K, val: V)
    where
        K: ToString,
        V: ToString,
    {
        self.kv_store.push((key.to_string(), val.to_string(), None));
    }

    pub fn add_item_with_units<K, V, U>(&mut self, key: K, val: V, units: U)
    where
        K: ToString,
        V: ToString,
        U: ToString,
    {
        self.kv_store
            .push((key.to_string(), val.to_string(), Some(units.to_string())));
    }

    fn get_scaled_color(&self, is_dark: bool) -> Color {
        if is_dark { Color::WHITE } else { Color::BLACK }.scale_alpha(0.5)
    }

    pub fn view<'a, Message>(&self, is_dark: bool) -> iced::widget::Row<'a, Message>
    where
        Message: 'a + Clone,
    {
        let scaled = self.get_scaled_color(is_dark);

        let mut keys = column![].spacing(5).align_x(Right);
        let mut vals = column![].spacing(5);

        for kv in self.kv_store.clone().into_iter() {
            keys = keys.push(text(kv.0).style(move |_| text::Style {
                color: Some(scaled.clone()),
            }));

            if let Some(units) = kv.2 {
                vals = vals.push(text(format!("{}{}", kv.1, units)));
            } else {
                vals = vals.push(text(kv.1));
            }
        }
        let a = row![keys, vals].spacing(5).align_y(Center);
        a
    }
}

pub fn placeholder<'a, Message, M>(msg: M) -> Element<'a, Message>
where
    Message: 'a + Clone,
    M: ToString,
{
    container(column![
        vertical_space(),
        row![
            horizontal_space(),
            // text("Нажмите кнопку \"Обновить\" для получения\nсведений о погоде в заданном месте.")
            text(msg.to_string())
                .size(25)
                .color(Color::WHITE.scale_alpha(0.5)),
            horizontal_space(),
        ],
        vertical_space(),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

/// Creates modal window over above `base` Element
///
/// ## Styles:
/// - `iced::widget::container::rounded_box`
pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    stack![
        base.into(),
        opaque(
            mouse_area(
                center(opaque(
                    container(content).style(container::rounded_box).padding(10)
                ))
                .style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..Default::default()
                    }
                })
            )
            .on_press(on_blur)
        )
    ]
    .into()
}
