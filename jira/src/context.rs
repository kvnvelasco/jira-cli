pub trait Context: Sized {
    fn get_pk(&self) -> String;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;

    fn get_last_accesed(&self) -> i64;
    fn set_last_accesed(&mut self, timestamp: i64);

    fn get_time_spent(&self) -> i64;
    fn set_time_spent(&mut self, duration: i64);
}

pub trait ContextProvider<T: Context>: Sized {
    fn get_contexts(&self) -> Vec<T>;
    fn clean_contexts(&self);
    fn get_active_context(&self) -> Option<T>;
}
