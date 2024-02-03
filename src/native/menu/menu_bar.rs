//! menu bar

use iced_widget::core::{
    alignment, event, layout::{self, Node, Limits}, mouse, overlay, renderer, touch, widget::{tree, Tree}, 
    Event, 
    Alignment, Clipboard, Color, Element, Layout, Length, Overlay, Padding, Rectangle, Shell, Size, Widget
};

use super::{
    flex, menu_bar_overlay::MenuBarOverlay, menu_tree::*
};

pub(super) struct MenuBarState{
    pub(super) active_root: usize,
    pub(super) open: bool,
    pub(super) viewport: Rectangle,
    pub(super) indices: Vec<usize>,
}
impl Default for MenuBarState{
    fn default() -> Self {
        Self {
            active_root: 0,
            open: false,
            viewport: Rectangle::default(),
            indices: Vec::new(),
        }
    }
}

/// menu bar
pub struct MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    roots: Vec<Item<'a, Message, Theme, Renderer>>,
    spacing: f32,
    padding: Padding,
    width: Length,
    height: Length,
}
#[allow(missing_docs)]
impl<'a, Message, Theme, Renderer> MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(roots: Vec<Item<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            roots,
            spacing: 0.0,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }
    
}
impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for MenuBar<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<MenuBarState>()
    }

    fn state(&self) -> tree::State {
        tree::State::Some(Box::new(MenuBarState::default()))
    }

    /// \[Tree{item_state, \[widget_state, menu_state]}...]
    fn children(&self) -> Vec<Tree> {
        println!("bar children");
        todo!()
    }

    /// tree: Tree{bar_state, \[item_tree...]}
    fn diff(&self, tree: &mut Tree) {
        println!("bar diff");
        todo!()
    }
    
    /// tree: Tree{bar_state, \[item_tree...]}
    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        println!("bar layout");
        todo!()
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        println!("bar event");
        use event::Status::*;

        todo!()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        println!("bar draw");
        todo!()
    }
    
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        println!("bar overlay");
        todo!()
    }
}
impl<'a, Message, Theme, Renderer> From<MenuBar<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + renderer::Renderer,
    // Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
