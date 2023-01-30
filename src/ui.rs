use druid::{Widget, WidgetExt, Env, EventCtx, Point, Menu, MenuItem, UnitPoint, Code, Event, Color};
use druid::widget::{Label, Flex, TextBox, Button, List, Checkbox, ZStack, Padding, Controller};

use crate::data::{TodoState, TodoItem};
use crate::saver::Saver;

pub fn ui_builder() -> impl Widget<TodoState> {
    let header = Flex::row()
        .with_flex_child(TextBox::new()
                    .lens(TodoState::new_text)
                    .expand_width()
                    .controller(Enter {}), 1.)
        .with_child(
            Button::new("->")
            .on_click(|_ctx, data: &mut TodoState, _env| {
                if data.new_text.trim() != "" {
                    let text = data.new_text.clone();
                    data.new_text = "".to_string();
                    data.todos.push_front(TodoItem { checked: false, text })
                }
            }))
        .with_child(Saver {});

    let todos = List::new(|| {
        let bg = Color::rgba8(0, 0, 0, 50);
        Flex::row()
            .with_child(Label::new(|data: &TodoItem, _: &Env| data.text.clone() ))
            .with_default_spacer()
            .with_child(Checkbox::new("").lens(TodoItem::checked))
            .with_flex_spacer(0.1)
            .with_child(Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut TodoItem, _env| {
                let data_clone = data.clone();
                let menu: Menu<TodoState> = Menu::empty()
                    .entry(MenuItem::new("Remove").on_activate(move |_, main_data: &mut TodoState, _| {
                        let location = main_data.todos.index_of(&data_clone).unwrap();
                        main_data.todos.remove(location);
                    }));
                ctx.show_context_menu(menu, Point::new(0., 0.))
            })).background(bg)

    }).lens(TodoState::todos).scroll().vertical();

    let clear_complete = Button::new("Clear Completed")
        .on_click(|_, data: &mut TodoState, _| {
            data.todos.retain(|item| !item.checked)
        });

    ZStack::new(Flex::column().with_child(header).with_flex_child(todos, 1.)).with_aligned_child(Padding::new(5., clear_complete), UnitPoint::BOTTOM_RIGHT)
}

struct Enter;

impl<W: Widget<TodoState>> Controller<TodoState, W> for Enter {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut TodoState, env: &Env) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {
                if data.new_text.trim() != "" {
                    let text = data.new_text.clone();
                    data.new_text = "".to_string();
                    data.todos.push_front(TodoItem { checked: false, text });
                }
                
            }
        }
        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &TodoState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &TodoState, data: &TodoState, env: &Env) {
        child.update(ctx, old_data, data, env)
    }
}
