data.json:Vec<CollectionData>

pub struct Window{
    pub settings:OnceCell<Settings>,
    pub entry:TemplateChild<Entry>,
    pub tasks_list:TemplateChild<ListBox>,
    pub collections_list:TemplateChild<ListBox>,
    pub leaflet:TemplateChild<Leaflet>,
    pub stack:TemplateChild<Stack>,
    pub back_button:TemplateChild<Button>,
    pub collections:OnceCell<ListStore<CollectionObject>>,
    pub current_collection:RefCell<Option<CollectionObject>>,
    pub current_filter_model:RefCell<Option<FilterListModel>>,
    pub tasks_changed_handler_id:RefCell<Option<SignalHandlerId>>
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
    pub tasks:OnceCell<ListStore<TaskObject>>,
}
impl ObjectSubclass for CollectionObject {
    const NAME: &'static str = "TodoCollectionObject";
}
#[derive(Serialize,Deserialize)]
pub struct CollectionData{
    pub title:String,
    pub tasks_data:Vec<TaskData>,
}
