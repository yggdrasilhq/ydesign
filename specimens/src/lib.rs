//! Reusable proposed list component. Demo state is kept outside the component.
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ListDensity {
    #[default]
    Editorial,
    Compact,
}

/// The parent owns width and alignment. This component never adds a second
/// max-width or reserves an icon slot for items that have no icon.
#[component]
pub fn ChapterList(
    chapters: Vec<Chapter>,
    on_open: EventHandler<String>,
    #[props(default)] current: Option<String>,
    #[props(default)] density: ListDensity,
) -> Element {
    let class = match density {
        ListDensity::Editorial => "chapter-list editorial",
        ListDensity::Compact => "chapter-list compact",
    };
    rsx! {
        ol { class,
            for (index, chapter) in chapters.iter().enumerate() {
                li { key: "{chapter.id}",
                    button {
                        id: "chapter-{chapter.id}",
                        class: "chapter-link",
                        r#type: "button",
                        aria_current: if current.as_ref() == Some(&chapter.id) { "page" } else { "false" },
                        onclick: {
                            let id = chapter.id.clone();
                            move |_| on_open.call(id.clone())
                        },
                        span { class: "ordinal", aria_hidden: "true", "{index + 1:02}" }
                        span { class: "chapter-copy",
                            span { class: "chapter-title", "{chapter.title}" }
                            if !chapter.description.is_empty() {
                                span { class: "chapter-description", "{chapter.description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn fixtures(long: bool) -> Vec<Chapter> {
    let titles = [
        "Start with the person", "Color and material", "Typography",
        "List views", "Commands in context", "Forms", "Feedback",
        "Navigation", "Identity", "Motion", "Inheritance", "Review",
    ];
    titles.iter().enumerate().map(|(i, title)| Chapter {
        id: format!("c{}", i + 1),
        title: if long && i == 3 {
            "List views that remain readable when chapter titles need more than one line".into()
        } else { (*title).into() },
        description: if i == 7 { String::new() } else if long && i == 3 {
            "The same chapter identity, a deliberately longer description, and enough content to test wrapping without shrinking the text or hiding the words that distinguish this entry.".into()
        } else { format!("A working study of {}.", title.to_lowercase()) },
    }).collect()
}

#[derive(Default, Clone, PartialEq)]
pub struct Study {
    pub selected: Option<String>,
    pub last_opened: Option<String>,
    pub long_labels: bool,
    pub compact: bool,
    pub narrow: bool,
    pub critique: String,
}

impl Study {
    pub fn open(&mut self, id: &str) -> bool {
        if !fixtures(self.long_labels).iter().any(|c| c.id == id) { return false; }
        self.selected = Some(id.into());
        self.last_opened = Some(id.into());
        true
    }
    pub fn back(&mut self) { self.selected = None; }
    pub fn reset(&mut self) { *self = Self::default(); }
    pub fn review_text(&self) -> String {
        format!("Book: yggui\nSpecimen: list-views/v1-proposal\nScenario: long={}, compact={}, narrow={}\nChapter: {}\nObservation: {}",
            self.long_labels, self.compact, self.narrow,
            self.last_opened.as_deref().unwrap_or("contents"), self.critique)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigation_preserves_fixture_and_reset_restores_every_field() {
        let mut state = Study { long_labels: true, narrow: true, ..Study::default() };
        assert!(!state.open("untrusted"));
        assert!(state.open("c4"));
        state.back();
        assert!(state.selected.is_none());
        assert_eq!(state.last_opened.as_deref(), Some("c4"));
        assert!(state.long_labels && state.narrow);
        state.critique = "Title wraps, focus must return here.".into();
        assert!(state.review_text().contains("Chapter: c4"));
        state.reset();
        assert!(state == Study::default());
    }
    #[test]
    fn fixture_ids_survive_long_label_variant() {
        assert_eq!(fixtures(false).iter().map(|c| &c.id).collect::<Vec<_>>(),
            fixtures(true).iter().map(|c| &c.id).collect::<Vec<_>>());
        assert_eq!(fixtures(false).len(), 12);
        assert!(fixtures(false).iter().any(|c| c.description.is_empty()));
    }
}
