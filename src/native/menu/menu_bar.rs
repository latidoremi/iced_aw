//! A widget that handles menu trees

use super::menu_inner::{
    CloseCondition, Direction, ItemHeight, ItemWidth, Menu, MenuState, PathHighlight,
};
use super::menu_tree::MenuTree;
use crate::style::menu_bar::StyleSheet;

use iced_widget::core::{
    event,
    layout::{Limits, Node},
    mouse::{self, Cursor},
    overlay, renderer, touch,
    widget::{tree, Tree},
    Alignment, Clipboard, Color, Element, Layout, Length, Padding, Rectangle, Shell, Widget,
};

pub(super) struct MenuBarState {
    pub(super) pressed: bool,
    pub(super) view_cursor: Cursor,
    pub(super) open: bool,
    pub(super) active_root: Option<usize>,
    pub(super) horizontal_direction: Direction,
    pub(super) vertical_direction: Direction,
    pub(super) menu_states: Vec<MenuState>,
}
impl MenuBarState {
    pub(super) fn get_trimmed_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.menu_states
            .iter()
            .take_while(|ms| ms.index.is_some())
            .map(|ms| ms.index.expect("No indices were found in the menu state."))
    }

    pub(super) fn reset(&mut self) {
        self.open = false;
        self.active_root = None;
        self.menu_states.clear();
    }
}
impl Default for MenuBarState {
    fn default() -> Self {
        Self {
            pressed: false,
            view_cursor: Cursor::Available([-0.5, -0.5].into()),
            open: false,
            active_root: None,
            horizontal_direction: Direction::Positive,
            vertical_direction: Direction::Positive,
            menu_states: Vec::new(),
        }
    }
}

/// A `MenuBar` collects `MenuTree`s and handles
/// all the layout, event processing and drawing
#[allow(missing_debug_implementations)]
pub struct MenuBar<'a, Message, Renderer = crate::Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    spacing: f32,
    padding: Padding,
    bounds_expand: u16,
    close_condition: CloseCondition,
    item_width: ItemWidth,
    item_height: ItemHeight,
    path_highlight: Option<PathHighlight>,
    menu_roots: Vec<MenuTree<'a, Message, Renderer>>,
    style: <Renderer::Theme as StyleSheet>::Style,
}
impl<'a, Message, Renderer> MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates a new [`MenuBar`] with the given menu roots
    #[must_use]
    pub fn new(menu_roots: Vec<MenuTree<'a, Message, Renderer>>) -> Self {
        let mut menu_roots = menu_roots;
        menu_roots.iter_mut().for_each(MenuTree::set_index);

        Self {
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: 0.0,
            padding: Padding::ZERO,
            bounds_expand: 15,
            close_condition: CloseCondition {
                leave: true,
                click_outside: true,
                click_inside: true,
            },
            item_width: ItemWidth::Uniform(150),
            item_height: ItemHeight::Uniform(30),
            path_highlight: Some(PathHighlight::MenuActive),
            menu_roots,
            style: <Renderer::Theme as StyleSheet>::Style::default(),
        }
    }

    /// Sets the width of the [`MenuBar`]
    #[must_use]
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`MenuBar`]
    #[must_use]
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the spacing between menu roots
    #[must_use]
    pub fn spacing(mut self, units: f32) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the expand value for each menu's check bounds
    ///
    /// When the cursor goes outside of a menu's check bounds,
    /// the menu will be closed automatically, this value expands
    /// the check bounds
    #[must_use]
    pub fn bounds_expand(mut self, value: u16) -> Self {
        self.bounds_expand = value;
        self
    }

    /// Sets the [`Padding`] of the [`MenuBar`]
    #[must_use]
    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    /// [`CloseCondition`]
    #[must_use]
    pub fn close_condition(mut self, close_condition: CloseCondition) -> Self {
        self.close_condition = close_condition;
        self
    }

    /// [`ItemWidth`]
    #[must_use]
    pub fn item_width(mut self, item_width: ItemWidth) -> Self {
        self.item_width = item_width;
        self
    }

    /// [`ItemHeight`]
    #[must_use]
    pub fn item_height(mut self, item_height: ItemHeight) -> Self {
        self.item_height = item_height;
        self
    }

    /// Sets the method for drawing path highlight
    #[must_use]
    pub fn path_highlight(mut self, path_highlight: Option<PathHighlight>) -> Self {
        self.path_highlight = path_highlight;
        self
    }

    /// Sets the style of the menu bar and its menus
    #[must_use]
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }
}
impl<'a, Message, Renderer> Widget<Message, Renderer> for MenuBar<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn diff(&self, tree: &mut Tree) {
        if tree.children.len() > self.menu_roots.len() {
            tree.children.truncate(self.menu_roots.len());
        }

        tree.children
            .iter_mut()
            .zip(self.menu_roots.iter())
            .for_each(|(t, root)| {
                let flat = root
                    .flattern()
                    .iter()
                    .map(|mt| mt.item.as_widget())
                    .collect::<Vec<_>>();

                t.diff_children(&flat);
            });

        if tree.children.len() < self.menu_roots.len() {
            let extended = self.menu_roots[tree.children.len()..].iter().map(|root| {
                let mut tree = Tree::empty();
                let flat = root
                    .flattern()
                    .iter()
                    .map(|mt| Tree::new(mt.item.as_widget()))
                    .collect();
                tree.children = flat;
                tree
            });
            tree.children.extend(extended);
        }
    }

    fn state(&self) -> tree::State {
        tree::State::new(MenuBarState::default())
    }

    fn children(&self) -> Vec<Tree> {
        /*
        menu bar
            menu root 1 (stateless)
                flat tree
            menu root 2 (stateless)
                flat tree
            ...
        */

        self.menu_roots
            .iter()
            .map(|root| {
                let mut tree = Tree::empty();
                let flat = root
                    .flattern()
                    .iter()
                    .map(|mt| Tree::new(mt.item.as_widget()))
                    .collect();
                tree.children = flat;
                tree
            })
            .collect()
    }

    fn layout(&self, renderer: &Renderer, limits: &Limits) -> Node {
        use super::flex;

        let limits = limits.width(self.width).height(self.height);
        let children = self
            .menu_roots
            .iter()
            .map(|root| &root.item)
            .collect::<Vec<_>>();
        flex::resolve(
            &flex::Axis::Horizontal,
            renderer,
            &limits,
            self.padding,
            self.spacing,
            Alignment::Center,
            &children,
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: event::Event,
        layout: Layout<'_>,
        view_cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        use event::Event::{Mouse, Touch};
        use mouse::{Button::Left, Event::ButtonReleased};
        use touch::Event::{FingerLifted, FingerLost};

        let root_status = process_root_events(
            &mut self.menu_roots,
            view_cursor,
            tree,
            &event,
            layout,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let state = tree.state.downcast_mut::<MenuBarState>();

        match event {
            Mouse(ButtonReleased(Left)) | Touch(FingerLifted { .. } | FingerLost { .. }) => {
                if state.menu_states.is_empty() && view_cursor.is_over(layout.bounds()) {
                    state.view_cursor = view_cursor;
                    state.open = true;
                }
            }
            _ => (),
        }
        root_status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as renderer::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        view_cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<MenuBarState>();
        let cursor_pos = view_cursor.position().unwrap_or_default();
        let position = if state.open && (cursor_pos.x < 0.0 || cursor_pos.y < 0.0) {
            state.view_cursor
        } else {
            view_cursor
        };

        // draw path highlight
        if self.path_highlight.is_some() {
            let styling = theme.appearance(&self.style);
            if let Some(active) = state.active_root {
                let active_bounds = layout
                    .children()
                    .nth(active)
                    .expect("Active child not found in menu?")
                    .bounds();
                let path_quad = renderer::Quad {
                    bounds: active_bounds,
                    border_radius: styling.border_radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                };
                let path_color = styling.path;
                renderer.fill_quad(path_quad, path_color);
            }
        }

        self.menu_roots
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .for_each(|((root, t), lo)| {
                root.item.as_widget().draw(
                    &t.children[root.index],
                    renderer,
                    theme,
                    style,
                    lo,
                    position,
                    viewport,
                );
            });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let state = tree.state.downcast_ref::<MenuBarState>();
        if !state.open {
            return None;
        }

        Some(
            Menu {
                tree,
                menu_roots: &mut self.menu_roots,
                bounds_expand: self.bounds_expand,
                close_condition: self.close_condition,
                item_width: self.item_width,
                item_height: self.item_height,
                bar_bounds: layout.bounds(),
                root_bounds_list: layout.children().map(|lo| lo.bounds()).collect(),
                path_highlight: self.path_highlight,
                style: &self.style,
            }
            .overlay(),
        )
    }
}
impl<'a, Message, Renderer> From<MenuBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(value: MenuBar<'a, Message, Renderer>) -> Self {
        Self::new(value)
    }
}

#[allow(unused_results, clippy::too_many_arguments)]
fn process_root_events<Message, Renderer>(
    menu_roots: &mut [MenuTree<'_, Message, Renderer>],
    view_cursor: Cursor,
    tree: &mut Tree,
    event: &event::Event,
    layout: Layout<'_>,
    renderer: &Renderer,
    clipboard: &mut dyn Clipboard,
    shell: &mut Shell<'_, Message>,
    viewport: &Rectangle,
) -> event::Status
where
    Renderer: renderer::Renderer,
{
    menu_roots
        .iter_mut()
        .zip(&mut tree.children)
        .zip(layout.children())
        .map(|((root, t), lo)| {
            // assert!(t.tag == tree::Tag::stateless());
            root.item.as_widget_mut().on_event(
                &mut t.children[root.index],
                event.clone(),
                lo,
                view_cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            )
        })
        .fold(event::Status::Ignored, event::Status::merge)
}
