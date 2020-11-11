use crochee_plugin::Plugin;

pub struct ExamplePlugin {
    pub name: String,
    pub id: usize,
    pub path: String,
    pub content: String,
}

pub fn load_plugin(name: String, id: usize, path: String, content: String) -> impl Plugin {
    ExamplePlugin {
        name,
        id,
        path,
        content,
    }
}

impl Plugin for ExamplePlugin {
    fn setup(&self) {
        println!("name:{},id:{}", self.name, self.id)
    }

    fn run(&self) {
        println!("content:{}", self.content)
    }

    fn teardown(&self) {
        println!("path:{}", self.path)
    }
}