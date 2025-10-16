use iced::advanced::widget::{Operation, tree};
use iced::advanced::{Clipboard, Layout, Shell, Widget, layout, overlay, renderer, widget};
use iced::widget::{button, column, container, text};
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme, Vector, mouse};

use crate::components::min_dimension::min_width;

#[derive(Debug, Clone)]
pub struct MenuButton<'a, M: Clone + 'a> {
    content: &'a str,
    message: M,
}

pub fn menu_item<'a, M: Clone + 'a>(content: &'static str, message: M) -> MenuButton<'a, M> {
    MenuButton { content, message }
}

fn menu_button<'a, M: Clone + 'a>(entry: MenuButton<'a, M>, length: Length) -> Element<'a, M> {
    button(text(entry.content).size(15))
        .padding([2, 4])
        .width(length)
        .style(|theme, status| match status {
            button::Status::Pressed | button::Status::Hovered => button::secondary(theme, status),
            _ => button::text(theme, status),
        })
        .on_press(entry.message)
        .into()
}

pub fn context_menu<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    entries: Vec<MenuButton<'a, Message>>,
) -> ContextMenu<'a, Message>
where
    Message: 'a + Clone,
{
    let build_menu = |length: Length| {
        container(
            column(
                entries
                    .iter()
                    .cloned()
                    .map(|entry| menu_button(entry, length)),
            )
            .spacing(2),
        )
        .padding(4)
        .style(container::bordered_box)
    };

    let menu = min_width(build_menu(Length::Shrink), build_menu(Length::Fill), 150.);

    ContextMenu {
        base: base.into(),
        menu: menu.into(),
        button: mouse::Button::Right,
    }
}

pub struct ContextMenu<'a, Message> {
    base: Element<'a, Message>,
    menu: Element<'a, Message>,
    button: mouse::Button,
}

impl<'a, Message> ContextMenu<'a, Message> {
    pub fn button(self, button: mouse::Button) -> Self {
        ContextMenu {
            base: self.base,
            menu: self.menu,
            button,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum State {
    Closed,
    Open(Point),
}

impl State {
    fn open(self) -> Option<Point> {
        match self {
            State::Closed => None,
            State::Open(point) => Some(point),
        }
    }
}

impl<'a, Message> Widget<Message, Theme, Renderer> for ContextMenu<'a, Message> {
    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.base.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.base
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::Closed)
    }

    fn children(&self) -> Vec<widget::Tree> {
        vec![widget::Tree::new(&self.base), widget::Tree::new(&self.menu)]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[self.base.as_widget(), self.menu.as_widget()]);
    }

    fn operate(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        let state = tree.state.downcast_mut::<State>();

        operation.custom(None, layout.bounds(), state);

        self.base
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        match &event {
            Event::Mouse(mouse::Event::ButtonPressed(b)) if *b == self.button => {
                if let Some(position) = cursor.position_over(layout.bounds()) {
                    *state = State::Open(position);
                }
            }
            _ => (),
        }

        self.base.as_widget_mut().update(
            &mut tree.children[0],
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
        _tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            Default::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<State>();

        let (first, second) = tree.children.split_at_mut(1);

        let base = self.base.as_widget_mut().overlay(
            &mut first[0],
            layout,
            renderer,
            viewport,
            translation,
        );

        let overlay = state.open().map(|position| {
            overlay::Element::new(Box::new(Overlay {
                content: &mut self.menu,
                tree: &mut second[0],
                state,
                position: position + translation + Vector::new(4.0, 4.0),
            }))
        });

        Some(overlay::Group::with_children(base.into_iter().chain(overlay).collect()).overlay())
    }
}

impl<'a, Message> From<ContextMenu<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(context_menu: ContextMenu<'a, Message>) -> Self {
        Element::new(context_menu)
    }
}

struct Overlay<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut widget::Tree,
    state: &'b mut State,
    position: Point,
}

impl<'a, 'b, Message> overlay::Overlay<Message, Theme, Renderer> for Overlay<'a, 'b, Message> {
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds)
            .width(Length::Fill)
            .height(Length::Fill);

        let node = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);

        let viewport = Rectangle::new(Point::ORIGIN, bounds);
        let mut bounds = Rectangle::new(self.position, node.size());

        if bounds.x < viewport.x {
            bounds.x = viewport.x;
        } else if viewport.x + viewport.width < bounds.x + bounds.width {
            bounds.x = viewport.x + viewport.width - bounds.width;
        }

        if bounds.y < viewport.y {
            bounds.y = viewport.y;
        } else if viewport.y + viewport.height < bounds.y + bounds.height {
            bounds.y = viewport.y + viewport.height - bounds.height;
        }

        node.move_to(bounds.position())
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.content
            .as_widget_mut()
            .operate(self.tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        if let Event::Mouse(mouse::Event::ButtonPressed(_)) = &event
            && cursor.position_over(layout.bounds()).is_none()
        {
            *self.state = State::Closed;
        }

        if let Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) = &event
            && cursor.position_over(layout.bounds()).is_some()
        {
            *self.state = State::Closed;
        }

        self.content.as_widget_mut().update(
            self.tree,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );
    }
}
