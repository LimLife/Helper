use crate::new_type_id::ContentID;

pub trait Select {
    fn select(&mut self, content_id: ContentID);
    fn deselect(&mut self);
    fn is_select(&self, content_id: ContentID) -> bool;
    fn toggle(&mut self, content_id: ContentID);
}
