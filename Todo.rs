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

new
setup_settings
setup_collections
  create_collection_row
restore_data
  collections
  set_current_collection
    tasks
      current_collection
    create_task_row
    tasks
    set_task_list_visible
    set_task_list_visible
    select_collection_row
      collections
      current_collection
setup_callbacks
  new_task
  new_task
  settings
  set_filter
    filter
      settings
  set_stack
  set_stack
  collections
  set_current_collection
  select_collection_row
setup_actions
  settings
  collections
  current_collection
  collections
  collections
  set_current_collection
  tasks
  new_collection
    collections
    set_current_collection
  









