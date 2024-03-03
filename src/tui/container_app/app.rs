use crate::tui::container_app;
use crate::tui::data::{container_constraint_len_calculator, Container};
use crate::tui::stream::Message;
use crate::tui::style::{TableColors, ITEM_HEIGHT, PALETTES};
use crate::tui::table_ui::TuiTableState;
use crate::tui::ui_loop::{AppBehavior, Apps};
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use futures::{stream, Stream};
use ratatui::prelude::*;
use ratatui::widgets::{ScrollbarState, TableState};
use std::io;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct App {
    pub(crate) state: TableState,
    pub(crate) items: Vec<Container>,
    pub(crate) longest_item_lens: (u16, u16, u16, u16, u16),
    pub(crate) scroll_state: ScrollbarState,
    pub(crate) colors: TableColors,
    color_index: usize,
    table_height: usize,
    pub(crate) filter: String,
}

impl TuiTableState for App {
    type Item = Container;

    fn get_items(&self) -> &[Self::Item] {
        &self.items
    }

    fn get_state(&mut self) -> &mut TableState {
        &mut self.state
    }

    fn set_state(&mut self, state: TableState) {
        self.state = state;
    }

    fn get_scroll_state(&self) -> &ScrollbarState {
        &self.scroll_state
    }

    fn set_scroll_state(&mut self, scroll_state: ScrollbarState) {
        self.scroll_state = scroll_state;
    }

    fn get_table_colors(&self) -> &TableColors {
        &self.colors
    }

    fn set_table_colors(&mut self, colors: TableColors) {
        self.colors = colors;
    }

    fn get_color_index(&self) -> usize {
        self.color_index
    }

    fn set_color_index(&mut self, color_index: usize) {
        self.color_index = color_index;
    }

    fn reset_selection_state(&mut self) {
        self.state = TableState::default().with_selected(0);
        self.scroll_state = ScrollbarState::new(self.items.len().saturating_sub(1) * ITEM_HEIGHT);
    }
    fn get_table_height(&self) -> usize {
        self.table_height
    }

    fn set_table_height(&mut self, table_height: usize) {
        self.table_height = table_height;
    }

    fn get_filter(&self) -> String {
        self.filter.clone()
    }

    fn set_filter(&mut self, filter: String) {
        self.filter = filter;
    }
}

impl AppBehavior for container_app::app::App {
    async fn handle_event(&mut self, event: &Message) -> Result<Option<Apps>, io::Error> {
        let mut app_holder = Some(Apps::Container { app: self.clone() });
        match event {
            Message::Key(Event::Key(key)) => {
                if key.kind == KeyEventKind::Press {
                    use KeyCode::{Char, Down, Esc, Up};
                    match key.code {
                        Char('q') | Esc => {
                            app_holder = None;
                        }
                        Char('j') | Down => {
                            self.next();
                            //todo: stop all this cloning
                            app_holder = Some(Apps::Container { app: self.clone() });
                        }
                        Char('k') | Up => {
                            self.previous();
                            app_holder = Some(Apps::Container { app: self.clone() });
                        }
                        Char('c' | 'C') => {
                            self.next_color();
                            app_holder = Some(Apps::Container { app: self.clone() });
                        }
                        Char('f' | 'F') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.page_forward();
                        }
                        Char('b' | 'B') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            self.page_backward();
                        }
                        _k => {}
                    }
                }
            }
            Message::Container(data_vec) => {
                let new_app = Self {
                    longest_item_lens: container_constraint_len_calculator(data_vec),
                    items: data_vec.clone(),
                    ..self.clone()
                };
                let new_app_holder = Apps::Container { app: new_app };
                app_holder = Some(new_app_holder);
            }
            _ => {}
        }
        Ok(app_holder)
    }
    fn draw_ui<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
        terminal.draw(|f| container_app::ui::ui(f, &mut self.clone()))?;
        Ok(())
    }

    fn stream(&self, _should_stop: Arc<AtomicBool>) -> impl Stream<Item = Message> {
        stream::empty()
    }
}

impl App {
    pub fn new(data_vec: Vec<Container>) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: container_constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new(data_vec.len().saturating_sub(1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 2,
            table_height: 0,
            items: data_vec,
            filter: "".to_string(),
        }
    }

    // pub fn get_event_details(&mut self) -> Vec<(String, String, Option<String>)> {
    //     vec![]
    // }

    pub fn get_left_details(&mut self) -> Vec<(String, String, Option<String>)> {
        self.get_selected_item().map_or_else(Vec::new, |container| {
            container
                .mounts
                .iter()
                .map(|label| (label.name.clone(), label.value.clone(), None))
                .collect()
        })
    }

    pub fn get_right_details(&mut self) -> Vec<(String, String, Option<String>)> {
        self.get_selected_item().map_or_else(Vec::new, |container| {
            container
                .envvars
                .iter()
                .map(|label| (label.name.clone(), label.value.clone(), None))
                .collect()
        })
    }
}
