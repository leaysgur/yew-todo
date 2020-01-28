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
    pub value: String,
}

impl State {
    pub fn update(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn add_todo(&mut self) {
        self.todos.push(Todo::new(&self.value));
        self.value = "".to_string();
    }

    pub fn toggle_todo(&mut self, idx: usize) {
        let todo = self.todos.get_mut(idx).unwrap();
        todo.toggle();
    }
}
