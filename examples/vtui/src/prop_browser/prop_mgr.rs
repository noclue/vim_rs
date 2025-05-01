use vim_rs::types::structs::{ManagedObjectReference, ObjectSpec};
use tui_tree_widget::TreeState;
use std::rc::Rc;
use std::cell::RefCell;
use vim_rs::core::pc_cache::{CacheManager, SharedRefCacheProxy};
use ratatui::Frame;
use ratatui::layout::Rect;
use crossterm::event::{KeyCode, KeyEvent};
use std::ops::DerefMut;
use log::{debug, warn};
use crate::event::{AppEvent, EventHandler};
use crate::prop_browser::prop_browser::{PropertyBrowser, PropertyBrowserState};

pub struct PropertyBrowserManager {
    /// Cache manager for managing object caches.
    cache_mgr: Rc<RefCell<CacheManager>>,
    /// Property collector filter for the current view
    filter: ManagedObjectReference,
    /// Browser state for managing the current view.
    browser_state: Rc<RefCell<PropertyBrowserState>>,
    /// Object reference for the current view
    obj: ManagedObjectReference,
}

impl PropertyBrowserManager {
    pub async fn new(
        cache_mgr: Rc<RefCell<CacheManager>>,
        obj: ManagedObjectReference,
    ) -> anyhow::Result<Self> {
        let browser_state = Rc::new(RefCell::new(PropertyBrowserState::new(obj.clone(), None).await?));

        let filter = cache_mgr
            .borrow_mut()
            .add_cache(
                Box::new(SharedRefCacheProxy::new(browser_state.clone())),
                vec![ObjectSpec {
                    obj: obj.clone(),
                    skip: Some(false),
                    select_set: None,
                }],
            )
            .await?;

        Ok(Self {
            cache_mgr,
            filter,
            browser_state,
            obj,
        })
    }

    pub async fn from_history_record(
        record: HistoryRecord,
        cache_mgr: Rc<RefCell<CacheManager>>,
    ) -> anyhow::Result<Self> {
        let browser_state = Rc::new(RefCell::new(PropertyBrowserState::new(record.obj.clone(), Some(record.state)).await?));

        let filter = cache_mgr
            .borrow_mut()
            .add_cache(
                Box::new(SharedRefCacheProxy::new(browser_state.clone())),
                vec![ObjectSpec {
                    obj: record.obj.clone(),
                    skip: Some(false),
                    select_set: None,
                }],
            )
            .await?;

        let mgr = Self {
            cache_mgr,
            filter,
            browser_state,
            obj: record.obj,
        };

        Ok(mgr)
    }
    
    pub async fn load_history_record(&mut self, entry: HistoryRecord) -> anyhow::Result<()> {
        let _ = self.load_int(entry.obj, Some(entry.state)).await?;
        Ok(())
    }

    pub fn save_state(&mut self, events: &mut EventHandler) {
        let tree_state = self.browser_state.borrow_mut().replace_tree_state(None);
        let entry = HistoryRecord {
            obj: self.obj.clone(),
            state: tree_state,
        };
        events.send(AppEvent::PropertyManagerHistory(entry));
    }

    pub fn render(&mut self, frame: &mut Frame, body_area: Rect) {
        let props = PropertyBrowser::new();
        frame.render_stateful_widget(
            props,
            body_area,
            self.browser_state.borrow_mut().deref_mut(),
        );
    }

    pub async fn handle_key(&mut self, key: &KeyEvent, events: &mut EventHandler) -> anyhow::Result<bool> {
        match key.code {
            KeyCode::Char('w') | KeyCode::Up => {
                self.browser_state.borrow_mut().up();
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.browser_state.borrow_mut().down();
            }
            KeyCode::Char('a') | KeyCode::Left => {
                self.browser_state.borrow_mut().left();
            }
            KeyCode::Char('d') | KeyCode::Right => {
                self.browser_state.borrow_mut().right();
            }
            KeyCode::Enter => {
                self.enter(events).await?;
            }
            _ => {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn load(&mut self, obj: ManagedObjectReference, events: &mut EventHandler) -> anyhow::Result<bool> {
        // Check if the object is already loaded
        if self.obj.value == obj.value {
            return Ok(false);
        }
        let old_obj = self.obj.clone();
        let res = self.load_int(obj, None).await?;
        self.add_history(old_obj, res, events);
        Ok(true)
    }
    async fn load_int(
        &mut self,
        obj: ManagedObjectReference,
        new_tree_state: Option<TreeState<String>>,
    ) -> anyhow::Result<TreeState<String>> {
        self.cache_mgr
            .borrow_mut()
            .remove_cache(&self.filter)
            .await?;
        self.obj = obj;

        let tree_state = self
            .browser_state
            .borrow_mut()
            .set_obj(self.obj.clone(), new_tree_state)?;

        self.filter = self
            .cache_mgr
            .borrow_mut()
            .add_cache(
                Box::new(SharedRefCacheProxy::new(self.browser_state.clone())),
                vec![ObjectSpec {
                    obj: self.obj.clone(),
                    skip: Some(false),
                    select_set: None,
                }],
            )
            .await?;

        Ok(tree_state)
    }

    async fn enter(&mut self, events: &mut EventHandler) -> anyhow::Result<bool> {
        let Some(selected) = self.browser_state.borrow().get_selected_object() else {
            return Ok(false);
        };

        self.load(selected, events).await?;
        Ok(true)
    }

    fn add_history(
        &mut self,
        selected_object: ManagedObjectReference,
        tree_state: TreeState<String>,
        events: &mut EventHandler,
    ) {
        let entry = HistoryRecord {
            obj: selected_object,
            state: tree_state,
        };
        events.send(AppEvent::PropertyManagerHistory(entry));
    }
}

impl Drop for PropertyBrowserManager {
    fn drop(&mut self) {
        let cache_mgr = self.cache_mgr.clone();
        let filter = self.filter.clone();
        // Schedule task to remove the cache
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                debug!("Terminating PropertyBrowserManager. Releasing filter");
                cache_mgr.borrow_mut().remove_cache(&filter).await.unwrap_or_else(|e| {
                    warn!("Failed to remove PropertyBrowserManager filter: {:?}, {}", filter, e);
                });
            });
        });
    }
}

#[derive(Debug)]
pub struct HistoryRecord {
    obj: ManagedObjectReference,
    state: TreeState<String>,
}