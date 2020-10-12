use std::collections::BTreeSet;

use crate::app::ActiveList;

/// Filters is meant to be a struct which holds filters
pub struct Filters<I>
where
    I: IntoIterator,
{
    pub contexts: I,
    pub tags: I,
}

impl<I> Filters<I>
where
    I: IntoIterator,
{
    pub fn new(contexts: I, tags: I) -> Self {
        Self { contexts, tags }
    }

    pub fn get_mut(&mut self, al: ActiveList) -> &mut I {
        match al {
            ActiveList::Contexts => &mut self.contexts,
            ActiveList::Tags => &mut self.tags,
            ActiveList::Tasks => panic!("tasks is not a filter"),
        }
    }

    pub fn get(&self, al: ActiveList) -> &I {
        match al {
            ActiveList::Contexts => &self.contexts,
            ActiveList::Tags => &self.tags,
            ActiveList::Tasks => panic!("tasks is not a filter"),
        }
    }
}

impl<'a> Filters<BTreeSet<&'a str>> {
    pub fn include(&self, item: &str) -> bool {
        self.include_for_filter(item, ActiveList::Contexts)
            && self.include_for_filter(item, ActiveList::Tags)
    }

    fn include_for_filter(&self, item: &str, active_list: ActiveList) -> bool {
        let filters = self.get(active_list);
        if filters.is_empty() {
            return true;
        }

        if filters.iter().any(|&f| item.contains(f)) {
            return true;
        }

        false
    }
}

/*

pub trait Empty {
    fn is_empty(&self) -> bool;
}

impl<T> Empty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> Empty for BTreeSet<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

*/
