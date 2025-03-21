//! Use a color picker as an input element for picking colors.
//!
//! *This API requires the following crate features to be activated: `color_picker`*

use iced_widget::{
    button,
    core::{
        event,
        layout::{Limits, Node},
        mouse::{self, Cursor},
        overlay, renderer,
        widget::{
            self,
            tree::{self, Tag, Tree},
        },
        Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Widget,
    },
    renderer::Renderer,
};

pub use crate::style::color_picker::{Appearance, StyleSheet};

use super::overlay::color_picker::{
    self, ColorBarDragged, ColorPickerOverlay, ColorPickerOverlayButtons,
};

//TODO: Remove ignore when Null is updated. Temp fix for Test runs
/// An input element for picking colors.
///
/// # Example
/// ```ignore
/// # use iced_aw::ColorPicker;
/// # use iced::{Color, widget::{button, Button, Text}};
/// #
/// #[derive(Clone, Debug)]
/// enum Message {
///     Open,
///     Cancel,
///     Submit(Color),
/// }
///
/// let color_picker = ColorPicker::new(
///     true,
///     Color::default(),
///     Button::new(Text::new("Pick color"))
///         .on_press(Message::Open),
///     Message::Cancel,
///     Message::Submit,
/// );
/// ```
#[allow(missing_debug_implementations)]
pub struct ColorPicker<'a, Message, Theme = iced_widget::style::Theme>
where
    Message: Clone,
    Theme: StyleSheet + button::StyleSheet,
{
    /// Show the picker.
    show_picker: bool,
    /// The color to show.
    color: Color,
    /// The underlying element.
    underlay: Element<'a, Message, Renderer<Theme>>,
    /// The message that is send if the cancel button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_cancel: Message,
    /// The function that produces a message when the submit button of the [`ColorPickerOverlay`](ColorPickerOverlay) is pressed.
    on_submit: Box<dyn Fn(Color) -> Message>,
    /// The style of the [`ColorPickerOverlay`](ColorPickerOverlay).
    style: <Theme as StyleSheet>::Style,
    /// The buttons of the overlay.
    overlay_state: Element<'a, Message, Renderer<Theme>>,
}

impl<'a, Message, Theme> ColorPicker<'a, Message, Theme>
where
    Message: 'a + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + widget::text::StyleSheet,
{
    /// Creates a new [`ColorPicker`](ColorPicker) wrapping around the given underlay.
    ///
    /// It expects:
    ///     * if the overlay of the color picker is visible.
    ///     * the initial color to show.
    ///     * the underlay [`Element`] on which this [`ColorPicker`](ColorPicker)
    ///         will be wrapped around.
    ///     * a message that will be send when the cancel button of the [`ColorPicker`](ColorPicker)
    ///         is pressed.
    ///     * a function that will be called when the submit button of the [`ColorPicker`](ColorPicker)
    ///         is pressed, which takes the picked [`Color`] value.
    pub fn new<U, F>(
        show_picker: bool,
        color: Color,
        underlay: U,
        on_cancel: Message,
        on_submit: F,
    ) -> Self
    where
        U: Into<Element<'a, Message, Renderer<Theme>>>,
        F: 'static + Fn(Color) -> Message,
    {
        Self {
            show_picker,
            color,
            underlay: underlay.into(),
            on_cancel,
            on_submit: Box::new(on_submit),
            style: <Theme as StyleSheet>::Style::default(),
            overlay_state: ColorPickerOverlayButtons::default().into(),
        }
    }

    /// Sets the style of the [`ColorPicker`](ColorPicker).
    #[must_use]
    pub fn style(mut self, style: <Theme as StyleSheet>::Style) -> Self {
        self.style = style;
        self
    }
}

/// The state of the [`ColorPicker`](ColorPicker).
#[derive(Debug, Default)]
pub struct State {
    /// The state of the overlay.
    pub(crate) overlay_state: color_picker::State,
}

impl State {
    /// Creates a new [`State`](State).
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self {
            overlay_state: color_picker::State::new(color),
        }
    }

    /// Resets the color of the state.
    pub fn reset(&mut self) {
        self.overlay_state.color = Color::from_rgb(0.5, 0.25, 0.25);
        self.overlay_state.color_bar_dragged = ColorBarDragged::None;
    }
}

impl<'a, Message, Theme> Widget<Message, Renderer<Theme>> for ColorPicker<'a, Message, Theme>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + widget::text::StyleSheet,
{
    fn tag(&self) -> Tag {
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.color))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.underlay), Tree::new(&self.overlay_state)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.underlay, &self.overlay_state]);
    }

    fn width(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn height(&self) -> Length {
        self.underlay.as_widget().width()
    }

    fn layout(&self, renderer: &Renderer<Theme>, limits: &Limits) -> Node {
        self.underlay.as_widget().layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer<Theme>,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.underlay.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer<Theme>,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &state.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer<Theme>,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer<Theme>,
    ) -> Option<overlay::Element<'b, Message, Renderer<Theme>>> {
        let picker_state: &mut State = state.state.downcast_mut();

        if !self.show_picker {
            return self
                .underlay
                .as_widget_mut()
                .overlay(&mut state.children[0], layout, renderer);
        }

        let bounds = layout.bounds();
        let position = Point::new(bounds.center_x(), bounds.center_y());

        Some(
            ColorPickerOverlay::new(
                picker_state,
                self.on_cancel.clone(),
                &self.on_submit,
                position,
                self.style,
                &mut state.children[1],
            )
            .overlay(),
        )
    }
}

impl<'a, Message, Theme> From<ColorPicker<'a, Message, Theme>>
    for Element<'a, Message, Renderer<Theme>>
where
    Message: 'static + Clone,
    Theme: 'a + StyleSheet + button::StyleSheet + widget::text::StyleSheet,
{
    fn from(color_picker: ColorPicker<'a, Message, Theme>) -> Self {
        Element::new(color_picker)
    }
}
