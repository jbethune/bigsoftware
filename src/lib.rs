//! This is supposed to be a very big TODO list management project.
//!
//! A lot of people are supposed to work together on building this.
//!

#![deny(missing_docs)]
#![deny(clippy::doc_markdown)]

use std::cmp::Ordering;

/// A task that needs to be done
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Todo {
    // TODO add assigned user
    title: String,
    description: String,
    urgency: Urgency,
    is_special: bool,
}

impl Todo {
    /// Create a new TODO item
    pub fn new(title: &str, description: &str, urgency: Urgency, is_special: bool) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            urgency,
            is_special,
        }
    }
}

/// Describe  how urgently something needs to be done
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    // TODO more variants, TODO maybe different ordering?
    /// No deadline
    #[default]
    NonExisting,
    /// Should be done evenually
    Low,
    /// Regular priority
    Medium,
    /// Get it done ASAP
    High,
    /// Drop everything and do this now!
    ExtremlyUrgent,
}

impl Ord for Todo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.urgency.cmp(&other.urgency)
    }
}

// TODO Demonstrate: derive this trait and unittest the result
impl PartialOrd for Todo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A list of TODO items
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TodoList {
    items: Vec<Todo>,
}

impl TodoList {
    /// Create a new list
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a task to the list
    pub fn add_task(&mut self, title: &str, description: &str, urgency: Urgency, is_special: bool) {
        self.items
            .push(Todo::new(title, description, urgency, is_special));
        self.items.sort_unstable(); // TODO find insertion position instead
    }

    /// Get the most urgent task right now
    pub fn get_task(&mut self) -> Option<Todo> {
        self.items.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let mut tasks = vec![
            Todo::new(
                "Prepare talk",
                "Talk about CI",
                Urgency::ExtremlyUrgent,
                false,
            ),
            Todo::new(
                "Learn Fortran",
                "Because Dinosaurs are cool",
                Urgency::NonExisting,
                true,
            ),
            Todo::new("Buy groceries", "Need to eat", Urgency::High, true),
            Todo::new(
                "Do the dishes",
                "Almost out of plates",
                Urgency::Medium,
                false,
            ),
        ];
        tasks.sort_unstable();
        assert_eq!(tasks[3].title, "Prepare talk");
        assert_eq!(tasks[2].description, "Need to eat");
        assert_eq!(tasks[1].urgency, Urgency::Medium);
        assert_eq!(tasks[0].description, "Because Dinosaurs are cool");
    }

    #[test]
    fn test_list() {
        let mut list = TodoList::new();
        list.add_task(
            "Write a TODO list",
            "good examples are hard to find",
            Urgency::Medium,
            false,
        );
        list.add_task(
            "Teach rustaceans to do CI",
            "So that open source will be even better",
            Urgency::High,
            false,
        );
        list.add_task(
            "Find more silly examples",
            "Because short lists are boring",
            Urgency::Low,
            false,
        );
        assert_eq!(list.get_task().unwrap().title, "Teach rustaceans to do CI");
        assert_eq!(list.get_task().unwrap().title, "Write a TODO list");
        assert_eq!(list.get_task().unwrap().title, "Find more silly examples");
        assert!(list.get_task().is_none());
    }
}
