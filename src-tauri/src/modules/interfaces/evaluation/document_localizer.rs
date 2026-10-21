use crate::modules::interfaces::document::InterfaceDocument;
use crate::modules::interfaces::document::WidgetNode;
use crate::modules::interfaces::rules::ConditionChange;
use std::borrow::Cow;
use std::collections::BTreeMap;

pub(crate) struct DocumentLocalizer;

impl DocumentLocalizer {
    pub(in crate::modules::interfaces) fn apply<'a>(
        document: &'a InterfaceDocument,
        locale: &str,
    ) -> Cow<'a, InterfaceDocument> {
        let Some(texts) = document.localization.dictionary(locale) else {
            return Cow::Borrowed(document);
        };

        let mut result = document.clone();

        Self::text(&mut result.name, &texts);
        Self::text(&mut result.description, &texts);
        Self::nodes(&mut result.roots, &texts);

        Cow::Owned(result)
    }

    fn text(value: &mut String, texts: &BTreeMap<&str, &str>) {
        if let Some(translated) = texts.get(value.as_str()) {
            *value = (*translated).to_owned();
        }
    }

    fn nodes(nodes: &mut [WidgetNode], texts: &BTreeMap<&str, &str>) {
        for node in nodes {
            Self::text(&mut node.name, texts);
            Self::text(&mut node.group.header_text, texts);
            Self::text(&mut node.appearance.text, texts);
            Self::text(&mut node.style.right_text, texts);
            Self::text(&mut node.sounds.show_text, texts);
            Self::text(&mut node.sounds.hide_text, texts);

            for button in &mut node.header_buttons {
                Self::text(&mut button.label, texts);
            }

            for condition in &mut node.conditions {
                for change in &mut condition.changes {
                    match change {
                        ConditionChange::Text { value } | ConditionChange::IconLabel { value } => {
                            Self::text(value, texts)
                        }
                        ConditionChange::Sound { speech_text, .. } => {
                            Self::text(speech_text, texts)
                        }
                        _ => {}
                    }
                }
            }

            Self::nodes(&mut node.children, texts);
        }
    }
}
