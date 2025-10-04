use iced::advanced::{
    layout::{Limits, Node},
    mouse::{self, Click, Cursor, Interaction, click::Kind},
    overlay,
    renderer::{Quad, Style},
    widget::{Operation, Tree, tree},
};
use iced::widget::rule;
use iced_core::{
    self as core, Clipboard, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector,
    Widget, border,
};

/// Creates a new [`Split`] with the given `first` and `second` widgets, a split position, and a
/// function to emit messages when the split position changes.
pub fn split<'a, Message, Theme, Renderer>(
    first: impl Into<Element<'a, Message, Theme, Renderer>>,
    second: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    direction: Direction,
    f: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    Split::new(first, second, split_at, f).direction(direction)
}

/// Creates a new [`horizontal`](Direction::Horizontal) [`Split`] with the given `top` and `bottom`
/// widgets, a split position, and a function to emit messages when the split position changes.
pub fn horizontal_split<'a, Message, Theme, Renderer>(
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    f: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    Split::new(top, bottom, split_at, f).direction(Direction::Horizontal)
}

/// Creates a new [`vertical`](Direction::Vertical) [`Split`] with the given `left` and `right`
/// widgets, a split position, and a function to emit messages when the split position changes.
pub fn vertical_split<'a, Message, Theme, Renderer>(
    left: impl Into<Element<'a, Message, Theme, Renderer>>,
    right: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    f: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    Split::new(left, right, split_at, f)
}

/// How the split is oriented.
#[derive(Clone, Copy, Debug, Default)]
pub enum Direction {
    /// The separator is a horizontal [`Rule`], separating a top and bottom widget.
    ///
    /// [`Rule`]: iced_widget::Rule
    Horizontal,
    /// The separator is a vertical [`Rule`], separating a left and right widget. This is the
    /// default.
    ///
    /// [`Rule`]: iced_widget::Rule
    #[default]
    Vertical,
}

impl Direction {
    fn select<T>(self, x: T, y: T) -> (T, T) {
        match self {
            Self::Horizontal => (x, y),
            Self::Vertical => (y, x),
        }
    }

    pub fn toggle(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// What `split_at` represents. This becomes relevant when the widget is resized in the layout
/// direction.
#[derive(Clone, Copy, Debug, Default)]
pub enum Strategy {
    /// `split_at` is the portion of the entire split's width that the `start` widget takes up. This
    /// is the default.
    #[default]
    Relative,
    /// `split_at` is the width of the `start` widget in pixels.
    Start,
    /// `split_at` is the width of the `end` widget in pixels.
    End,
}

#[derive(Default)]
struct State {
    hovering: bool,
    dragging: bool,
    last_click: Option<Click>,
}

/// Resizeable splits for [`iced`](https://github.com/iced-rs/iced).
#[expect(missing_debug_implementations, clippy::struct_field_names)]
pub struct Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    children: [Element<'a, Message, Theme, Renderer>; 2],
    split_at: f32,
    strategy: Strategy,
    direction: Direction,
    line_width: f32,
    handle_width: f32,
    class: Theme::Class<'a>,
    on_drag: Box<dyn Fn(f32) -> Message + 'a>,
    on_double_click: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    /// Creates a new [`Split`] with the given `start` and `end` widgets, a split position, and a
    /// function to emit messages when the split is dragged.
    #[must_use]
    pub fn new(
        start: impl Into<Element<'a, Message, Theme, Renderer>>,
        end: impl Into<Element<'a, Message, Theme, Renderer>>,
        split_at: f32,
        on_drag: impl Fn(f32) -> Message + 'a,
    ) -> Self {
        Self {
            children: [start.into(), end.into()],
            split_at,
            strategy: Strategy::default(),
            direction: Direction::default(),
            line_width: 1.0,
            handle_width: 11.0,
            class: Theme::default(),
            on_drag: Box::from(on_drag),
            on_double_click: None,
        }
    }

    /// Sets the [`Direction`] of the [`Split`].
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the [`Strategy`] of the [`Split`].
    #[must_use]
    pub fn strategy(mut self, strategy: Strategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the message emitted when the split is double-clicked.
    #[must_use]
    pub fn on_double_click(mut self, on_double_click: Message) -> Self {
        self.on_double_click = Some(on_double_click);
        self
    }

    /// Sets the width of the [`Rule`] between the `start` and `end` widgets. This should be less
    /// than or equal to the `handle_width`.
    ///
    /// [`Rule`]: iced_widget::Rule
    #[must_use]
    pub fn line_width(mut self, line_width: f32) -> Self {
        debug_assert!(self.handle_width >= line_width);
        self.line_width = line_width;
        self
    }

    /// Sets the width of the [`Rule`]'s handle between the `start` and `end` widgets. This should
    /// be greater than or equal to the `line_width`.
    ///
    /// [`Rule`]: iced_widget::Rule
    #[must_use]
    pub fn handle_width(mut self, handle_width: f32) -> Self {
        debug_assert!(handle_width >= self.line_width);
        self.handle_width = handle_width;
        self
    }

    /// Sets the [`Style`] of the [`Rule`] between the `start` and `end` widgets.
    ///
    /// [`Style`]: iced_widget::rule::Style
    /// [`Rule`]: iced_widget::Rule
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> rule::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<rule::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as rule::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the [`Class`] of the [`Rule`] between the `start` and `end` widgets.
    ///
    /// [`Class`]: iced_widget::rule::Catalog::Class
    /// [`Rule`]: iced_widget::Rule
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    fn start_layout(&self, layout_direction: f32) -> f32 {
        match self.strategy {
            Strategy::Relative => layout_direction * self.split_at,
            Strategy::Start => self.split_at,
            Strategy::End => layout_direction - self.split_at - self.handle_width,
        }
        .min(layout_direction - self.handle_width)
        .max(0.0)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let max_limits = limits.max();

        let (cross_direction, layout_direction) =
            self.direction.select(max_limits.width, max_limits.height);

        let start_layout = self.start_layout(layout_direction);
        let (start_width, start_height) = self.direction.select(cross_direction, start_layout);
        let start_limits = Limits::new(Size::ZERO, Size::new(start_width, start_height));

        let end_layout = layout_direction - start_layout - self.handle_width;
        let (end_width, end_height) = self.direction.select(cross_direction, end_layout);
        let end_limits = Limits::new(Size::ZERO, Size::new(end_width, end_height));

        let (offset_width, offset_height) =
            self.direction.select(0.0, start_layout + self.handle_width);

        let children = vec![
            self.children[0]
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &start_limits),
            self.children[1]
                .as_widget_mut()
                .layout(&mut tree.children[1], renderer, &end_limits)
                .translate(Vector::new(offset_width, offset_height)),
        ];

        Node::with_children(max_limits, children)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .for_each(|((child, tree), layout)| {
                child.as_widget_mut().update(
                    tree, event, layout, cursor, renderer, clipboard, shell, viewport,
                );
            });

        if shell.is_event_captured() {
            return;
        }

        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if let Event::Mouse(event) = event {
            match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) if state.hovering => {
                    state.last_click = cursor.position().map(|position| {
                        Click::new(position, mouse::Button::Left, state.last_click)
                    });
                    state.dragging = true;
                    shell.capture_event();
                }
                mouse::Event::CursorMoved {
                    position: Point { x, y },
                    ..
                } => {
                    let (cross_direction, layout_direction) =
                        self.direction.select(bounds.width, bounds.height);

                    if state.dragging {
                        let layout = self.direction.select(y - bounds.y, x - bounds.x).0
                            - self.handle_width / 2.0;

                        let split_at = match self.strategy {
                            Strategy::Relative => layout / layout_direction,
                            Strategy::Start => layout,
                            Strategy::End => layout_direction - layout - self.handle_width,
                        };

                        shell.publish((self.on_drag)(split_at));
                        shell.capture_event();
                    }

                    let layout = self.start_layout(layout_direction);
                    let (x, y) = self.direction.select(0.0, layout);
                    let (x, y) = (x + bounds.x, y + bounds.y);
                    let (width, height) = self.direction.select(cross_direction, self.handle_width);

                    state.hovering = cursor.is_over(Rectangle {
                        x,
                        y,
                        width,
                        height,
                    });
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) if state.dragging => {
                    if let Some(on_double_click) = &self.on_double_click
                        && let Some(click) = state.last_click
                        && click.kind() == Kind::Double
                        && cursor.is_over(layout.bounds())
                    {
                        shell.publish(on_double_click.clone());
                    }

                    state.dragging = false;
                    shell.capture_event();
                }
                _ => {}
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .for_each(|((child, tree), layout)| {
                child
                    .as_widget()
                    .draw(tree, renderer, theme, style, layout, cursor, viewport);
            });

        let bounds = layout.bounds();
        let style = theme.style(&self.class);

        let (cross_direction, layout_direction) =
            self.direction.select(bounds.width, bounds.height);

        let (offset, length) = style.fill_mode.fill(cross_direction);

        let layout = self.start_layout(layout_direction);
        let layout = layout + offset + (self.handle_width - self.line_width) / 2.0;
        let (x, y) = self.direction.select(0.0, layout);
        let (x, y) = ((x + bounds.x).round(), (y + bounds.y).round());
        let (width, height) = self.direction.select(length, self.line_width);

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x,
                    y,
                    width,
                    height,
                },
                border: border::rounded(style.radius),
                snap: style.snap,
                ..Quad::default()
            },
            style.color,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        let state = tree.state.downcast_ref::<State>();

        if state.hovering || state.dragging {
            match self.direction {
                Direction::Horizontal => Interaction::ResizingVertically,
                Direction::Vertical => Interaction::ResizingHorizontally,
            }
        } else {
            self.children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .map(|((child, tree), layout)| {
                    child
                        .as_widget()
                        .mouse_interaction(tree, layout, cursor, viewport, renderer)
                })
                .max()
                .unwrap_or_default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            self.children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
        });
    }
}

impl<'a, Message, Theme, Renderer> From<Split<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: rule::Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    fn from(value: Split<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
