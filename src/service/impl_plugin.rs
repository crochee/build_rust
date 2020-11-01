use crochee_plugin::Plugin;

pub struct ExamplePlugin {
    pub name: String,
    pub id: usize,
    pub path: String,
    pub content: String,
}

pub fn load_plugin(name: String, id: usize, path: String, content: String) -> *mut dyn Plugin {
    let ep = ExamplePlugin {
        name,
        id,
        path,
        content,
    };
    Box::into_raw(Box::new(ep))
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