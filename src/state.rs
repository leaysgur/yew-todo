#[derive(Debug, Clone)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

impl Todo {
    fn new(title: &str) -> Self {
        Todo {
            title: title.to_string(),
            done: false,
        }
    }

    fn toggle(&mut self) {
        self.done = !self.done;
    }
}

pub struct State {
    pub todos: Vec<Todo>,
}

impl State {
    pub fn add_todo(&mut self, value: String) {
        if value.is_empty() {
            return;
        }

        self.todos.push(Todo::new(&value));
    }

    pub fn toggle_todo(&mut self, idx: usize) {
        let todo = self.todos.get_mut(idx).unwrap();
        todo.toggle();
    }

    pub fn total_len(&self) -> usize {
        self.todos.len()
    }

    pub fn done_len(&self) -> usize {
        self.todos.iter().filter(|t| t.done).count()
    }
}
