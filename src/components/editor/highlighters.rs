use std::{collections::HashSet, ops::Range, sync::Arc};

use iced::highlighter::Highlight;
use iced_core::text::{Highlighter, highlighter::Format};
use parsers::Token;

use crate::components::colors;

pub trait IsDefined: Clone + PartialEq {
    fn is_defined(&self, name: &str) -> bool;
}

impl IsDefined for Arc<HashSet<String>> {
    fn is_defined(&self, name: &str) -> bool {
        self.contains(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplHighlighterSettings<T: IsDefined>(T);

impl<T: IsDefined> TemplHighlighterSettings<T> {
    pub fn new(vars: T) -> Self {
        Self(vars)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TemplHighlighter<T: IsDefined> {
    vars: T,
    current_line: usize,
}

impl<T: IsDefined + 'static> Highlighter for TemplHighlighter<T> {
    type Settings = TemplHighlighterSettings<T>;

    type Highlight = Format<iced::Font>;

    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Format<iced::Font>)> + 'a>;

    fn new(s: &Self::Settings) -> Self {
        Self {
            vars: s.0.clone(),
            current_line: 0,
        }
    }

    fn update(&mut self, s: &Self::Settings) {
        self.vars = s.0.clone();
        self.current_line = 0;
    }

    fn change_line(&mut self, _line: usize) {
        self.current_line = 0;
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let mut ranges = Vec::new();
        let parsed = parsers::parse_template(line);

        for span in parsed {
            if let Token::Variable(name) = span.token {
                let color = if self.vars.is_defined(&name) {
                    colors::LIGHT_GREEN
                } else {
                    colors::RED
                };

                ranges.push((
                    span.start..span.end,
                    Format {
                        color: Some(color),
                        font: None,
                    },
                ));
            }
        }

        Box::new(ranges.into_iter())
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

pub struct StackedHighlighter<H1, H2> {
    first: H1,
    second: H2,
}

trait ToFormat {
    fn to_format(&self) -> Format<iced::Font>;
}

impl ToFormat for Format<iced::Font> {
    fn to_format(&self) -> Format<iced::Font> {
        *self
    }
}

impl ToFormat for Highlight {
    fn to_format(&self) -> Format<iced::Font> {
        Highlight::to_format(self)
    }
}

impl<H1, H2> Highlighter for StackedHighlighter<H1, H2>
where
    H1: Highlighter,
    H2: Highlighter,
    H1::Highlight: ToFormat,
    H2::Highlight: ToFormat,
{
    type Settings = (H1::Settings, H2::Settings);

    type Highlight = Format<iced::Font>;

    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Format<iced::Font>)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            first: H1::new(&settings.0),
            second: H2::new(&settings.1),
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.first.update(&new_settings.0);
        self.second.update(&new_settings.1);
    }

    fn change_line(&mut self, line: usize) {
        self.first.change_line(line);
        self.second.change_line(line);
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let first = self
            .first
            .highlight_line(line)
            .map(|(range, highlight)| (range, highlight.to_format()));
        let second = self
            .second
            .highlight_line(line)
            .map(|(range, highlight)| (range, highlight.to_format()));

        Box::new(first.chain(second))
    }

    fn current_line(&self) -> usize {
        self.first.current_line()
    }
}
