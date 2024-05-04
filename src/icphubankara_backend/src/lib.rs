mod r#type;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
new*
struct Todo {
    description: String,
    done : bool

};

#(ic_cdk::update]
fn save_list(todo: Todo) -> string {

}