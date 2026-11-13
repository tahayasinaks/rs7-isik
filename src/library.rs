use crate::lighting::{CATEGORY, EFFECTS};
pub const PAGE_SIZE: usize = 6;
pub fn fold(s: &str) -> String {
    s.chars()
        .flat_map(char::to_lowercase)
        .filter(|c| *c != '\u{0307}')
        .map(|c| match c {
            'ı' => 'i',
            'ğ' => 'g',
            'ü' => 'u',
            'ş' => 's',
            'ö' => 'o',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}
#[derive(Default)]
pub struct Library {
    pub category: Option<usize>,
    pub page: usize,
    pub filtered: Vec<usize>,
    query: String,
    last_category: Option<usize>,
}
impl Library {
    pub fn update(&mut self, query: &str) {
        let query = fold(query);
        if query != self.query || self.category != self.last_category {
            self.page = 0;
        }
        self.query = query;
        self.last_category = self.category;
        self.filtered = (0..EFFECTS.len())
            .filter(|&i| {
                self.category.is_none_or(|c| CATEGORY[i] == c)
                    && fold(EFFECTS[i]).contains(&self.query)
            })
            .collect();
        self.page = self.page.min(self.pages() - 1);
    }
    pub fn pages(&self) -> usize {
        self.filtered.len().div_ceil(PAGE_SIZE).max(1)
    }
    pub fn visible(&self) -> &[usize] {
        let begin = (self.page * PAGE_SIZE).min(self.filtered.len());
        &self.filtered[begin..(begin + PAGE_SIZE).min(self.filtered.len())]
    }
    pub fn slot(&self, slot: usize) -> Option<usize> {
        self.visible().get(slot).copied()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn turkish_search_and_page_boundaries() {
        let mut l = Library::default();
        l.update("");
        assert_eq!(l.filtered.len(), EFFECTS.len());
        l.page = l.pages() - 1;
        l.update("");
        assert_eq!(l.visible().len(), (EFFECTS.len() - 1) % PAGE_SIZE + 1);
        l.update("ISIK");
        assert!(l
            .filtered
            .iter()
            .all(|&i| fold(EFFECTS[i]).contains("isik")));
        assert_eq!(l.page, 0);
        l.update("şİmŞeK");
        assert_eq!(l.filtered, vec![39]);
        l.update("bulunmayan efekt");
        assert!(l.visible().is_empty());
        assert_eq!(l.pages(), 1);
        assert_eq!(l.slot(0), None);
    }
    #[test]
    fn category_never_leaks_other_effects() {
        let mut l = Library::default();
        for c in 0..7 {
            l.category = Some(c);
            l.update("");
            assert!(!l.filtered.is_empty());
            assert!(l.filtered.iter().all(|&i| CATEGORY[i] == c));
        }
    }
}
