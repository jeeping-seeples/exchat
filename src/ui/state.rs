//! In-memory state, chat transcript types, and simulated research loop.

use crate::config::Settings;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub domain: String,
    pub title: String,
    pub snippet: String,
    pub cite_number: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResearchStage {
    Refining,
    Searching { query: String },
    Judging,
    Writing,
    Done,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResearchState {
    pub stage: ResearchStage,
    pub query: String,
    pub results: Vec<SearchResult>,
    pub rounds_used: u8,
    pub max_rounds: u8,
    pub started_at: Instant,
    pub next_step_at: Instant,
}

impl ResearchState {
    pub fn new(query: String, max_rounds: u8) -> Self {
        let now = Instant::now();
        Self {
            stage: ResearchStage::Refining,
            query,
            results: Vec::new(),
            rounds_used: 1,
            max_rounds,
            started_at: now,
            next_step_at: now + Duration::from_millis(600),
        }
    }

    /// Advance simulated state if time elapsed. Returns true if state changed.
    pub fn tick(&mut self, full_answer: &str) -> (bool, Option<String>) {
        if self.stage == ResearchStage::Done {
            return (false, None);
        }

        let now = Instant::now();
        if now < self.next_step_at {
            return (false, None);
        }

        match &self.stage {
            ResearchStage::Refining => {
                let q = self.query.clone();
                self.stage = ResearchStage::Searching {
                    query: format!("what is {}", q.trim()),
                };
                self.next_step_at = now + Duration::from_millis(700);
                (true, None)
            }
            ResearchStage::Searching { query: _ } => {
                // Add mock search results
                self.results = vec![
                    SearchResult {
                        domain: "wikipedia.org".into(),
                        title: format!("Overview and background for {}", self.query),
                        snippet: format!(
                            "Key concepts, definitions, and documented history regarding {}.",
                            self.query
                        ),
                        cite_number: 1,
                    },
                    SearchResult {
                        domain: "docs.rs".into(),
                        title: "Rust Documentation and Reference Library".into(),
                        snippet: "Standard library references, best practices, and verified specifications.".into(),
                        cite_number: 2,
                    },
                ];
                self.stage = ResearchStage::Judging;
                self.next_step_at = now + Duration::from_millis(600);
                (true, None)
            }
            ResearchStage::Judging => {
                self.stage = ResearchStage::Writing;
                self.next_step_at = now + Duration::from_millis(700);
                (true, None)
            }
            ResearchStage::Writing => {
                self.stage = ResearchStage::Done;
                (true, Some(full_answer.to_string()))
            }
            ResearchStage::Done => (false, None),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceCitation {
    pub number: usize,
    pub title: String,
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub research: Option<ResearchState>,
    pub sources: Vec<SourceCitation>,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    #[allow(dead_code)]
    pub id: String,
    pub title: String,
    #[allow(dead_code)]
    pub date_str: String,
    pub messages: Vec<Message>,
}

impl ChatSession {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            date_str: "Today".into(),
            messages: Vec::new(),
        }
    }
}

pub struct AppState {
    pub settings: Settings,
    pub show_settings_window: bool,
    pub chats: Vec<ChatSession>,
    pub active_chat_index: usize,
    pub input_text: String,
    pub history_search_query: String,
    pub status_message: String,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let mut sample_chat =
            ChatSession::new("chat-1".into(), "Research on Rust memory safety".into());
        sample_chat.messages.push(Message {
            role: MessageRole::User,
            content: "How does Rust guarantee memory safety without a garbage collector?".into(),
            research: None,
            sources: Vec::new(),
        });
        sample_chat.messages.push(Message {
            role: MessageRole::Assistant,
            content: "Rust achieves memory safety through its ownership model, strict compile-time borrow checker rules, and lifetime annotations [1]. References must always be valid, and mutable aliasing is strictly disallowed at any given time [2].".into(),
            research: Some(ResearchState {
                stage: ResearchStage::Done,
                query: "how rust guarantees memory safety".into(),
                results: vec![
                    SearchResult {
                        domain: "doc.rust-lang.org".into(),
                        title: "The Rust Programming Language - Ownership".into(),
                        snippet: "Ownership is Rust's most unique feature and has deep implications for the rest of the language.".into(),
                        cite_number: 1,
                    },
                    SearchResult {
                        domain: "doc.rust-lang.org".into(),
                        title: "References and Borrowing".into(),
                        snippet: "At any given time, you can have either one mutable reference or any number of immutable references.".into(),
                        cite_number: 2,
                    },
                ],
                rounds_used: 1,
                max_rounds: 4,
                started_at: Instant::now(),
                next_step_at: Instant::now(),
            }),
            sources: vec![
                SourceCitation {
                    number: 1,
                    title: "The Rust Programming Language - Ownership".into(),
                    domain: "doc.rust-lang.org".into(),
                },
                SourceCitation {
                    number: 2,
                    title: "References and Borrowing".into(),
                    domain: "doc.rust-lang.org".into(),
                },
            ],
        });

        let second_chat = ChatSession::new("chat-2".into(), "Egui layout and widgets".into());

        Self {
            settings,
            show_settings_window: false,
            chats: vec![sample_chat, second_chat],
            active_chat_index: 0,
            input_text: String::new(),
            history_search_query: String::new(),
            status_message: "Ready".into(),
        }
    }

    #[allow(dead_code)]
    pub fn active_chat_mut(&mut self) -> Option<&mut ChatSession> {
        self.chats.get_mut(self.active_chat_index)
    }

    pub fn active_chat(&self) -> Option<&ChatSession> {
        self.chats.get(self.active_chat_index)
    }

    pub fn new_chat(&mut self) {
        let id = format!("chat-{}", self.chats.len() + 1);
        self.chats
            .insert(0, ChatSession::new(id, "New Chat".into()));
        self.active_chat_index = 0;
        self.status_message = "Started new chat".into();
    }

    pub fn send_prompt(&mut self) {
        let text = self.input_text.trim().to_string();
        if text.is_empty() {
            return;
        }

        if self.chats.is_empty() {
            self.new_chat();
        }

        let max_rounds = self.settings.max_search_rounds;

        if let Some(chat) = self.chats.get_mut(self.active_chat_index) {
            // If this is the first message of a default-named chat, update title
            if chat.messages.is_empty() && chat.title == "New Chat" {
                let first_line = text.lines().next().unwrap_or("New Chat");
                let truncated: String = first_line.chars().take(30).collect();
                chat.title = truncated;
            }

            // Push user message
            chat.messages.push(Message {
                role: MessageRole::User,
                content: text.clone(),
                research: None,
                sources: Vec::new(),
            });

            // Push assistant message with active research state
            chat.messages.push(Message {
                role: MessageRole::Assistant,
                content: String::new(),
                research: Some(ResearchState::new(text, max_rounds)),
                sources: Vec::new(),
            });
        }

        self.input_text.clear();
        self.status_message = "Refining prompt...".into();
    }

    /// Advance active research loops across messages
    pub fn tick_research(&mut self) {
        if let Some(chat) = self.chats.get_mut(self.active_chat_index) {
            for msg in chat.messages.iter_mut() {
                if let Some(ref mut research) = msg.research {
                    if research.stage != ResearchStage::Done {
                        let query = research.query.clone();
                        let default_answer = format!(
                            "Here is the synthesized findings regarding '{}'. Based on verified sources [1], the concepts align with current industry standards and technical references [2].",
                            query
                        );
                        let (changed, completed_answer) = research.tick(&default_answer);
                        if changed {
                            match &research.stage {
                                ResearchStage::Refining => {
                                    self.status_message = "Refining prompt...".into();
                                }
                                ResearchStage::Searching { query } => {
                                    self.status_message = format!("Searching: {}", query);
                                }
                                ResearchStage::Judging => {
                                    self.status_message = "Judging evidence...".into();
                                }
                                ResearchStage::Writing => {
                                    self.status_message = "Synthesizing answer...".into();
                                }
                                ResearchStage::Done => {
                                    self.status_message = "Ready".into();
                                }
                            }
                        }

                        if let Some(ans) = completed_answer {
                            msg.content = ans;
                            msg.sources = research
                                .results
                                .iter()
                                .map(|r| SourceCitation {
                                    number: r.cite_number,
                                    title: r.title.clone(),
                                    domain: r.domain.clone(),
                                })
                                .collect();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_research_loop_transitions() {
        let mut research = ResearchState::new("rust lifetimes".into(), 4);
        assert_eq!(research.stage, ResearchStage::Refining);

        // Force time to advance
        research.next_step_at = Instant::now() - Duration::from_millis(10);
        let (changed, ans) = research.tick("Final answer");
        assert!(changed);
        assert!(ans.is_none());
        assert!(matches!(research.stage, ResearchStage::Searching { .. }));

        // Search -> Judging
        research.next_step_at = Instant::now() - Duration::from_millis(10);
        let (changed, ans) = research.tick("Final answer");
        assert!(changed);
        assert!(ans.is_none());
        assert_eq!(research.stage, ResearchStage::Judging);
        assert!(!research.results.is_empty());

        // Judging -> Writing
        research.next_step_at = Instant::now() - Duration::from_millis(10);
        let (changed, ans) = research.tick("Final answer");
        assert!(changed);
        assert!(ans.is_none());
        assert_eq!(research.stage, ResearchStage::Writing);

        // Writing -> Done
        research.next_step_at = Instant::now() - Duration::from_millis(10);
        let (changed, ans) = research.tick("Final answer");
        assert!(changed);
        assert_eq!(ans.as_deref(), Some("Final answer"));
        assert_eq!(research.stage, ResearchStage::Done);
    }

    #[test]
    fn test_app_state_send_prompt() {
        let mut state = AppState::new(Settings::default());
        state.input_text = "What is RAII?".into();
        state.send_prompt();
        let chat = state.active_chat_mut().unwrap();
        assert_eq!(chat.messages.len(), 4); // 2 sample + 1 user + 1 assistant
        assert_eq!(chat.messages[2].role, MessageRole::User);
        assert_eq!(chat.messages[2].content, "What is RAII?");
        assert_eq!(chat.messages[3].role, MessageRole::Assistant);
        assert!(chat.messages[3].research.is_some());
    }
}
