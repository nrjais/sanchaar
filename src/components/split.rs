use iced_core::{
    Animation, Clipboard, Color, Element, Event, Layout, Length, Point, Rectangle, Shell, Size,
    Vector, Widget,
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
/// widgets, a split position, and a function to emit messages when the [`Split`] is dragged.
pub fn horizontal_split<'a, Message, Theme, Renderer>(
    top: impl Into<Element<'a, Message, Theme, Renderer>>,
    bottom: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    f: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Split::new(top, bottom, split_at, f).direction(Direction::Horizontal)
}

/// Creates a new [`vertical`](Direction::Vertical) [`Split`] with the given `left` and `right`
/// widgets, a split position, and a function to emit messages when the [`Split`] is dragged.
pub fn vertical_split<'a, Message, Theme, Renderer>(
    left: impl Into<Element<'a, Message, Theme, Renderer>>,
    right: impl Into<Element<'a, Message, Theme, Renderer>>,
    split_at: f32,
    f: impl Fn(f32) -> Message + 'a,
) -> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    Split::new(left, right, split_at, f)
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

    pub fn opposite(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// What `split_at` represents. This affects how the [`Split`] behaves when resized in the layout
/// direction.
#[derive(Clone, Copy, Debug, Default)]
pub enum Strategy {
    /// `split_at` is the portion of the entire [`Split`]'s width that the `start` widget takes up.
    /// This is the default.
    #[default]
    Relative,
    /// `split_at` is the width of the `start` widget in pixels.
    Start,
    /// `split_at` is the width of the `end` widget in pixels.
    End,
}

struct State {
    hovering: bool,
    dragging: bool,
    last_click: Option<Click>,
    update_mix: bool,
    mix: Animation<bool>,
    now: Instant,
    duration: Duration,
    delay: Duration,
}

impl State {
    fn new(duration: Duration, delay: Duration) -> Self {
        Self {
            hovering: false,
            dragging: false,
            last_click: None,
            update_mix: false,
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

/// Resizeable splits for [`iced`](https://github.com/iced-rs/iced).
#[expect(missing_debug_implementations, clippy::struct_field_names)]
pub struct Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    children: [Element<'a, Message, Theme, Renderer>; 2],
    split_at: f32,
    strategy: Strategy,
    direction: Direction,
    handle_width: f32,
    duration: Duration,
    delay: Duration,
    class: Theme::Class<'a>,
    on_drag: Box<dyn Fn(f32) -> Message + 'a>,
    on_double_click: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Split<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    /// Creates a new [`Split`] with the given `start` and `end` widgets, a split position, and a
    /// function to emit messages when the [`Split`] is dragged.
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
            handle_width: 11.0,
            duration: Duration::from_millis(100),
            delay: Duration::from_millis(100),
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

    /// Sets the message emitted when the [`Split`] is double-clicked.
    #[must_use]
    pub fn on_double_click(mut self, on_double_click: Message) -> Self {
        self.on_double_click = Some(on_double_click);
        self
    }

    /// Sets the width of the [`Split`] between the `start` and `end` widgets.
    #[must_use]
    pub fn handle_width(mut self, handle_width: f32) -> Self {
        self.handle_width = handle_width;
        self
    }

    /// Sets the duration of the focus and unfocus transitions.
    #[must_use]
    pub fn focus_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the delay of the focus and unfocus transitions.
    #[must_use]
    pub fn focus_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// Sets the [`Style`] of the [`Split`] between the `start` and `end` widgets.
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the [`Class`](Catalog::Class) of the [`Split`] between the `start` and `end` widgets.
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

        match event {
            Event::Mouse(event) => match event {
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

                    let hovering = cursor.is_over(Rectangle {
                        x,
                        y,
                        width,
                        height,
                    });

                    if hovering != state.hovering {
                        state.hovering = hovering;

                        if !state.dragging {
                            state.update_mix = true;
                            shell.request_redraw();
                        }
                    }
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

                    if !state.hovering {
                        state.update_mix = true;
                        shell.request_redraw();
                    }
                }
                _ => {}
            },
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.now = *now;

                if state.update_mix {
                    state.update_mix = false;
                    state
                        .mix
                        .go_mut(state.hovering || state.dragging, state.now);
                }

                if state.mix.is_animating(state.now) {
                    shell.request_redraw();
                }
            }
            _ => {}
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

        let color = mix(
            style.unfocused.color,
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
        let layout = layout + (self.handle_width - width) / 2.0;
        let (x, y) = self.direction.select(0.0, layout);
        let (x, y) = ((x + bounds.x).round(), (y + bounds.y).round());
        let (width, height) = self.direction.select(cross_direction, width);

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
    Theme: Catalog + 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(value: Split<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub unfocused: Styled,
    pub focused: Styled,
    pub snap: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Styled {
    pub color: Color,
    pub width: f32,
    pub radius: Radius,
}

pub trait Catalog: Sized {
    type Class<'a>;
    #[must_use]
    fn default<'a>() -> Self::Class<'a>;
    #[must_use]
    fn style(&self, class: &Self::Class<'_>) -> Style;
}

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

#[must_use]
pub fn default(theme: &iced_core::Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        unfocused: Styled {
            color: palette.background.strong.color,
            width: 1.0,
            radius: 0.5.into(),
        },
        focused: Styled {
            color: palette.primary.base.color,
            width: 5.0,
            radius: 2.5.into(),
        },
        snap: true,
    }
}

fn mix(a: Color, b: Color, factor: f32) -> Color {
    let b_amount = factor.clamp(0.0, 1.0);
    let a_amount = 1.0 - b_amount;

    let a_linear = a.into_linear().map(|c| c * a_amount);
    let b_linear = b.into_linear().map(|c| c * b_amount);

    Color::from_linear_rgba(
        a_linear[0] + b_linear[0],
        a_linear[1] + b_linear[1],
        a_linear[2] + b_linear[2],
        a_linear[3] + b_linear[3],
    )
}
