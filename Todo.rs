pub struct Window{
    pub entry:TemplateChild<Entry>,
    pub tasks_list:TemplateChild<ListBox>,
    pub tasks:RefCell<Option<gio::ListStore>>,
    pub settings:OnceCell<Settings>,
}
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type ParentType = adw::ApplicationWindow;
}
pub struct TaskObject{
    #[property(name="completed",get,set,type=bool,member=completed)]
    #[property(name="content",get,set,type=String,member=content)]
    pub data:RefCell<TaskData>,
}
impl ObjectSubclass for TaskObject {
    const NAME: &'static str = "TodoTaskObject";
}
#[derive(Serialize,Deserialize)]
pub struct TaskData{
    pub completed:bool,
    pub content:String,
}
pub struct CollectionObject{
    #[property(get,set)]
    pub title:RefCell<String>,
    #[property(get,set)]
    pub tasks:OnceCell<gio::ListStore>,
}
impl ObjectSubclass for CollectionObject {
    const NAME: &'static str = "TodoCollectionObject";
}
#[derive(Serialize,Deserialize)]
pub struct CollectionData{
    pub title:String,
    pub tasks_data:Vec<TaskData>,
}