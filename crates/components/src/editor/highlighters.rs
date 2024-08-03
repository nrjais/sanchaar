use std::{collections::HashSet, ops::Range, sync::Arc};

use iced::highlighter::Highlight;
use iced_core::text::{highlighter::Format, Highlighter};
use parsers::Token;

use crate::colors;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SearchHighlighterSettings {
    pub search: String,
}

#[derive(Debug, Clone, Default)]
pub struct SearchHighlighter {
    search: String,
    current_line: usize,
    format: Format<iced::Font>,
}

impl Highlighter for SearchHighlighter {
    type Settings = SearchHighlighterSettings;

    type Highlight = Format<iced::Font>;

    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Format<iced::Font>)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        Self {
            current_line: 0,
            search: settings.search.clone(),
            format: Format {
                color: Some(colors::LIGHT_BLUE),
                font: None,
            },
        }
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.current_line = 0;
        self.search = new_settings.search.clone();
    }

    fn change_line(&mut self, _line: usize) {
        self.current_line = 0;
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let mut ranges = Vec::new();

        let mut start = 0;

        while let Some(start_index) = line[start..].find(&self.search) {
            let start_index = start + start_index;
            let end_index = start_index + self.search.len();

            ranges.push((start_index..end_index, self.format));

            start = end_index;
        }

        Box::new(ranges.into_iter())
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

pub trait IsDefineVariable: Clone + PartialEq {
    fn is_define_variable(&self, name: &str) -> bool;
}

impl IsDefineVariable for Arc<HashSet<String>> {
    fn is_define_variable(&self, name: &str) -> bool {
        self.contains(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplHighlighterSettings<T: IsDefineVariable>(T);

impl<T: IsDefineVariable> TemplHighlighterSettings<T> {
    pub fn new(vars: T) -> Self {
        Self(vars)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TemplHighlighter<T: IsDefineVariable> {
    vars: T,
    current_line: usize,
}

impl<T: IsDefineVariable + 'static> Highlighter for TemplHighlighter<T> {
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
            match span.token {
                Token::Variable(name) => {
                    let color = if self.vars.is_define_variable(&name) {
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
                _ => (),
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
        self.clone()
    }
}

impl ToFormat for Highlight {
    fn to_format(&self) -> Format<iced::Font> {
        Highlight::to_format(&self)
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
