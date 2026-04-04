use iced::Pixels;
use iced_core::{
    Animation, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Size, Vector,
    Widget,
    border::{self, Radius},
    layout::{Limits, Node},
    mouse::{self, Click, Cursor, Interaction, click::Kind},
    overlay,
    renderer::{self, Quad},
    time::{Duration, Instant},
    widget::{Operation, Tree, tree},
    window,
};

/// Creates a new [`horizontal`](Direction::Horizontal) [`Split`] with the given `top` and `bottom`
/// widgets, a split position, and a function to emit messages when the split gets dragged.
pub fn horizontal_split<'a, Message, Theme, Renderer>(
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    on_drag: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Split::new(top, bottom, split_at)
        .direction(Direction::Horizontal)
        .on_drag(on_drag)
}

/// Creates a new [`vertical`](Direction::Vertical) [`Split`] with the given `left` and `right`
/// widgets, a split position, and a function to emit messages when the split gets dragged.
pub fn vertical_split<'a, Message, Theme, Renderer>(
    left: impl Into<Element<'a, Message, Theme, Renderer>>,
    right: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    on_drag: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Split::new(left, right, split_at).on_drag(on_drag)
}

/// How the [`Split`] is oriented.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// The separator is a horizontal line, separating a top and bottom widget.
    Horizontal,
    /// The separator is a vertical line, separating a left and right widget. This is the
    /// default.
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

    pub fn opposite(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// How the [`Split`] behaves when dragged or resized.
#[derive(Clone, Copy, Debug, Default)]
pub enum Strategy {
    /// `split_at` is the position of the [`Split`]'s separator relative to its width. This is the
    /// default.
    #[default]
    Relative,
    /// `split_at` is the width of the `start` widget in pixels.
    Start,
    /// `split_at` is the width of the `end` widget in pixels.
    End,
}

impl State {
    fn new(duration: Duration, delay: Duration) -> Self {
        Self {
            status: Status::None,
            last_click: None,
            mix: Animation::new(false).duration(duration).delay(delay),
            now: Instant::now(),
            duration,
            delay,
        }
    }

    fn diff(&mut self, duration: Duration, delay: Duration) {
        if self.duration != duration || self.delay != delay {
            self.mix = self.mix.clone().delay(delay).duration(duration);
        }
    }
}

/// Resizeable splits for `iced`.
#[expect(missing_debug_implementations, clippy::struct_field_names)]
pub struct Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    children: [Element<'a, Message, Theme, Renderer>; 2],
    split_at: f32,
    strategy: Strategy,
    direction: Direction,
    handle_width: f32,
    spacing: f32,
    duration: Duration,
    delay: Duration,
    class: Theme::Class<'a>,
    on_drag: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_drag_start: Option<Box<dyn Fn() -> Message + 'a>>,
    on_drag_end: Option<Box<dyn Fn() -> Message + 'a>>,
    on_double_click: Option<Box<dyn Fn() -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    /// Creates a new [`Split`] with the given `start` and `end` widgets and a split position.
    #[must_use]
    pub fn new(
        start: impl Into<Element<'a, Message, Theme, Renderer>>,
        end: impl Into<Element<'a, Message, Theme, Renderer>>,
        split_at: f32,
    ) -> Self {
        Self {
            children: [start.into(), end.into()],
            split_at,
            strategy: Strategy::default(),
            direction: Direction::default(),
            handle_width: 11.0,
            spacing: 0.0,
            duration: Duration::from_millis(100),
            delay: Duration::from_millis(100),
            class: Theme::default(),
            on_drag: None,
            on_drag_start: None,
            on_drag_end: None,
            on_double_click: None,
        }
    }

    /// Sets the function to emit messages when the [`Split`] gets dragged.
    #[must_use]
    pub fn on_drag(self, on_drag: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_drag_maybe(Some(on_drag))
    }

    /// Sets the function to emit messages when the [`Split`] gets dragged, if `Some`.
    #[must_use]
    pub fn on_drag_maybe(mut self, on_drag_maybe: Option<impl Fn(f32) -> Message + 'a>) -> Self {
        self.on_drag = on_drag_maybe.map(|on_drag| Box::from(on_drag) as _);
        self
    }

    /// Sets the message emitted when the [`Split`] starts getting dragged.
    #[must_use]
    pub fn on_drag_start(self, on_drag_start: Message) -> Self
    where
        Message: Clone,
    {
        self.on_drag_start_maybe(Some(on_drag_start))
    }

    /// Sets the message emitted when the [`Split`] starts getting dragged, if `Some`.
    #[must_use]
    pub fn on_drag_start_maybe(self, on_drag_start_maybe: Option<Message>) -> Self
    where
        Message: Clone,
    {
        self.on_drag_start_with_maybe(
            on_drag_start_maybe.map(|on_drag_start| move || on_drag_start.clone()),
        )
    }

    /// Sets the function to emit messages when the [`Split`] starts getting dragged.
    #[must_use]
    pub fn on_drag_start_with(self, on_drag_start_with: impl Fn() -> Message + 'a) -> Self {
        self.on_drag_start_with_maybe(Some(on_drag_start_with))
    }

    /// Sets the function to emit messages when the [`Split`] starts getting dragged, if `Some`.
    #[must_use]
    pub fn on_drag_start_with_maybe(
        mut self,
        on_drag_start_with_maybe: Option<impl Fn() -> Message + 'a>,
    ) -> Self {
        self.on_drag_start =
            on_drag_start_with_maybe.map(|on_drag_start_with| Box::from(on_drag_start_with) as _);
        self
    }

    /// Sets the message emitted when the [`Split`] finishes getting dragged.
    #[must_use]
    pub fn on_drag_end(self, on_drag_end: Message) -> Self
    where
        Message: Clone,
    {
        self.on_drag_end_maybe(Some(on_drag_end))
    }

    /// Sets the message emitted when the [`Split`] finishes getting dragged, if `Some`.
    #[must_use]
    pub fn on_drag_end_maybe(self, on_drag_end_maybe: Option<Message>) -> Self
    where
        Message: Clone,
    {
        self.on_drag_end_with_maybe(
            on_drag_end_maybe.map(|on_drag_end| move || on_drag_end.clone()),
        )
    }

    /// Sets the function to emit messages when the [`Split`] finishes getting dragged.
    #[must_use]
    pub fn on_drag_end_with(self, on_drag_end_with: impl Fn() -> Message + 'a) -> Self {
        self.on_drag_end_with_maybe(Some(on_drag_end_with))
    }

    /// Sets the function to emit messages when the [`Split`] finishes getting dragged, if `Some`.
    #[must_use]
    pub fn on_drag_end_with_maybe(
        mut self,
        on_drag_end_with_maybe: Option<impl Fn() -> Message + 'a>,
    ) -> Self {
        self.on_drag_end =
            on_drag_end_with_maybe.map(|on_drag_end_with| Box::from(on_drag_end_with) as _);
        self
    }

    /// Sets the message emitted when the [`Split`] is double-clicked.
    #[must_use]
    pub fn on_double_click(self, on_double_click: Message) -> Self
    where
        Message: Clone,
    {
        self.on_double_click_maybe(Some(on_double_click))
    }

    /// Sets the message emitted when the [`Split`] is double-clicked, if `Some`.
    #[must_use]
    pub fn on_double_click_maybe(self, on_double_click_maybe: Option<Message>) -> Self
    where
        Message: Clone,
    {
        self.on_double_click_with_maybe(
            on_double_click_maybe.map(|on_double_click| move || on_double_click.clone()),
        )
    }

    /// Sets the function to emit messages when the [`Split`] is double-clicked.
    #[must_use]
    pub fn on_double_click_with(self, on_double_click_with: impl Fn() -> Message + 'a) -> Self {
        self.on_double_click_with_maybe(Some(on_double_click_with))
    }

    /// Sets the function to emit messages when the [`Split`] is double-clicked, if `Some`.
    #[must_use]
    pub fn on_double_click_with_maybe(
        mut self,
        on_double_click_with_maybe: Option<impl Fn() -> Message + 'a>,
    ) -> Self {
        self.on_double_click = on_double_click_with_maybe
            .map(|on_double_click_with| Box::from(on_double_click_with) as _);
        self
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

    /// Sets the width of the [`Split`]'s handle.
    #[must_use]
    pub fn handle_width(mut self, handle_width: impl Into<Pixels>) -> Self {
        self.handle_width = handle_width.into().0;
        self
    }

    /// Sets the spacing between the [`Split`]'s handle and content.
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the duration of the [`Split`]'s focus and unfocus transitions.
    #[must_use]
    pub fn focus_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the delay of the [`Split`]'s focus and unfocus transitions.
    #[must_use]
    pub fn focus_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets the [`Style`] of the [`Split`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the [`Class`](Catalog::Class) of the [`Split`].
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    fn hovering(&self, bounds: Rectangle, cursor: Cursor) -> Status {
        let (cross_direction, layout_direction) =
            self.direction.select(bounds.width, bounds.height);

        let layout = self.start_layout(layout_direction) + self.spacing;
        let (x, y) = self.direction.select(0.0, layout);
        let (x, y) = (x + bounds.x, y + bounds.y);
        let (width, height) = self.direction.select(cross_direction, self.handle_width);

        if cursor.is_over(Rectangle {
            x,
            y,
            width,
            height,
        }) {
            Status::Hovering
        } else {
            Status::None
        }
    }

    fn focused(&self, state: &State) -> bool {
        self.on_drag.is_some() && state.status != Status::None
    }

    fn separation(&self) -> f32 {
        2.0 * self.spacing + self.handle_width
    }

    fn start_layout(&self, layout_direction: f32) -> f32 {
        let separation = self.separation();
        match self.strategy {
            Strategy::Relative => layout_direction * self.split_at - separation / 2.0,
            Strategy::Start => self.split_at,
            Strategy::End => layout_direction - self.split_at - separation,
        }
        .min(layout_direction - separation)
        .max(0.0)
    }
}

struct State {
    status: Status,
    last_click: Option<Click>,
    mix: Animation<bool>,
    now: Instant,
    duration: Duration,
    delay: Duration,
}

#[derive(PartialEq)]
enum Status {
    Dragging,
    Grabbed,
    DoubleClicked,
    Hovering,
    None,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Split<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
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
        tree::State::new(State::new(self.duration, self.delay))
    }

    fn diff(&self, tree: &mut Tree) {
        tree.state
            .downcast_mut::<State>()
            .diff(self.duration, self.delay);

        tree.diff_children(&self.children);
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let max_limits = limits.max();

        let (cross_direction, layout_direction) =
            self.direction.select(max_limits.width, max_limits.height);

        let start_layout = self.start_layout(layout_direction);
        let (start_width, start_height) = self.direction.select(cross_direction, start_layout);
        let start_limits = Limits::new(Size::ZERO, Size::new(start_width, start_height));

        let separation = self.separation();
        let end_layout = layout_direction - start_layout - separation;
        let (end_width, end_height) = self.direction.select(cross_direction, end_layout);
        let end_limits = Limits::new(Size::ZERO, Size::new(end_width, end_height));

        let (offset_width, offset_height) = self.direction.select(0.0, start_layout + separation);

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
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .for_each(|((child, tree), layout)| {
                child
                    .as_widget_mut()
                    .update(tree, event, layout, cursor, renderer, shell, viewport);
            });

        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = *now;

            state.mix.go_mut(self.focused(state), state.now);
            if state.mix.is_animating(state.now) {
                shell.request_redraw();
            }

            return;
        }

        if shell.is_event_captured() {
            return;
        }

        let bounds = layout.bounds();

        if let Event::Mouse(event) = event {
            match event {
                mouse::Event::ButtonPressed(mouse::Button::Left) if self.focused(state) => {
                    state.last_click = cursor.position().map(|position| {
                        Click::new(position, mouse::Button::Left, state.last_click)
                    });

                    state.status = state
                        .last_click
                        .filter(|click| click.kind() == Kind::Double)
                        .map_or(Status::Grabbed, |_| Status::DoubleClicked);

                    shell.capture_event();
                }
                mouse::Event::CursorMoved {
                    position: Point { x, y },
                    ..
                } => {
                    if let Some(on_drag) = &self.on_drag
                        && matches!(
                            state.status,
                            Status::Dragging | Status::Grabbed | Status::DoubleClicked
                        )
                    {
                        let layout_direction = self.direction.select(bounds.width, bounds.height).1;

                        let layout = self.direction.select(x - bounds.x, y - bounds.y).1;

                        let separation = self.separation();
                        let split_at = match self.strategy {
                            Strategy::Relative => layout / layout_direction,
                            Strategy::Start => layout - separation / 2.0,
                            Strategy::End => layout_direction - layout - separation / 2.0,
                        };

                        if split_at != self.split_at {
                            if state.status != Status::Dragging {
                                state.status = Status::Dragging;
                                if let Some(on_drag_start) = &self.on_drag_start {
                                    shell.publish(on_drag_start());
                                }
                            }

                            shell.publish(on_drag(split_at));
                            shell.capture_event();
                        }
                    } else {
                        let focused = self.focused(state);

                        state.status = self.hovering(bounds, cursor);

                        if self.focused(state) != focused {
                            shell.request_redraw();
                        }
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => match state.status {
                    Status::Dragging => {
                        if let Some(on_drag_end) = &self.on_drag_end {
                            shell.publish(on_drag_end());
                            shell.capture_event();
                        }

                        let focused = self.focused(state);

                        state.status = self.hovering(bounds, cursor);

                        if self.focused(state) != focused {
                            shell.request_redraw();
                        }
                    }
                    Status::DoubleClicked => {
                        if let Some(on_double_click) = &self.on_double_click {
                            shell.publish(on_double_click());
                            shell.capture_event();
                        }

                        state.status = Status::Hovering;
                    }
                    Status::Grabbed => state.status = Status::Hovering,
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
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

        let style = theme.style(&self.class);
        let state = tree.state.downcast_ref::<State>();

        let color = style.unfocused.color.mix(
            style.focused.color,
            state.mix.interpolate(0.0, 1.0, state.now),
        );

        let width = state
            .mix
            .interpolate(style.unfocused.width, style.focused.width, state.now);

        let radius = Radius {
            top_left: state.mix.interpolate(
                style.unfocused.radius.top_left,
                style.focused.radius.top_left,
                state.now,
            ),
            top_right: state.mix.interpolate(
                style.unfocused.radius.top_right,
                style.focused.radius.top_right,
                state.now,
            ),
            bottom_right: state.mix.interpolate(
                style.unfocused.radius.bottom_right,
                style.focused.radius.bottom_right,
                state.now,
            ),
            bottom_left: state.mix.interpolate(
                style.unfocused.radius.bottom_left,
                style.focused.radius.bottom_left,
                state.now,
            ),
        };

        let bounds = layout.bounds();
        let (cross_direction, layout_direction) =
            self.direction.select(bounds.width, bounds.height);

        let layout = self.start_layout(layout_direction);
        let layout = layout + self.spacing + (self.handle_width - width) / 2.0;
        let (x, y) = self.direction.select(0.0, layout);
        let (x, y) = (x + bounds.x, y + bounds.y);
        let (width, height) = self.direction.select(cross_direction, width);
        let (width, height) = if style.snap {
            let unit = 1.0 / renderer.scale_factor().unwrap_or(1.0);
            (width.max(unit), height.max(unit))
        } else {
            (width, height)
        };

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x,
                    y,
                    width,
                    height,
                },
                border: border::rounded(radius),
                snap: style.snap,
                ..Quad::default()
            },
            color,
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

        if self.focused(state) {
            match self.direction {
                Direction::Horizontal => Interaction::ResizingRow,
                Direction::Vertical => Interaction::ResizingColumn,
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
    Message: 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: Split<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

/// The [style](Style) of a [`Split`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`StyleSheet`] of the [`Split`] while it's unfocused.
    pub unfocused: StyleSheet,
    /// The [`StyleSheet`] of the [`Split`] while it's focused.
    pub focused: StyleSheet,
    /// Whether the separator should be snapped to the pixel grid.
    pub snap: bool,
}

/// The [stylesheet](StyleSheet) of a [`Split`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StyleSheet {
    /// The color of the separator.
    pub color: Color,
    /// The width of the separator.
    pub width: f32,
    /// The radius of the corners of the separator.
    pub radius: Radius,
}

/// The [theme catalog](Catalog) of a [`Split`].
pub trait Catalog {
    /// The [`item class`](Self::Class) of the [`Catalog`].
    type Class<'a>;

    /// The default [`Class`](Self::Class) produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a [`Class`](Self::Class).
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

/// A styling function for a [`Split`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for iced_core::Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default styling of a [`Split`].
#[must_use]
pub fn default(theme: &iced_core::Theme) -> Style {
    let palette = theme.palette();

    Style {
        unfocused: StyleSheet {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 0.5.into(),
        },
        focused: StyleSheet {
            color: palette.primary.base.color,
            width: 5.0,
            radius: 2.5.into(),
        },
        snap: true,
    }
}
