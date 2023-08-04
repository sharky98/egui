use egui::{Context, Modifiers, ScrollArea, Ui};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::RwLock;

use super::About;
use super::Demo;
use super::View;
use crate::is_mobile;

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Demos {
    #[cfg_attr(feature = "serde", serde(skip))]
    demos: Vec<Box<dyn Demo + Sync + Send>>,

    open: BTreeSet<String>,
}

impl Default for Demos {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::<super::paint_bezier::PaintBezier>::default(),
            Box::<super::code_editor::CodeEditor>::default(),
            Box::<super::code_example::CodeExample>::default(),
            Box::<super::context_menu::ContextMenus>::default(),
            Box::<super::dancing_strings::DancingStrings>::default(),
            Box::<super::drag_and_drop::DragAndDropDemo>::default(),
            Box::<super::font_book::FontBook>::default(),
            Box::<super::MiscDemoWindow>::default(),
            Box::<super::multi_touch::MultiTouch>::default(),
            Box::<super::painting::Painting>::default(),
            Box::<super::plot_demo::PlotDemo>::default(),
            Box::<super::scrolling::Scrolling>::default(),
            Box::<super::sliders::Sliders>::default(),
            Box::<super::strip_demo::StripDemo>::default(),
            Box::<super::table_demo::TableDemo>::default(),
            Box::<super::text_edit::TextEdit>::default(),
            Box::<super::widget_gallery::WidgetGallery>::default(),
            Box::<super::window_options::WindowOptions>::default(),
            Box::<super::tests::WindowResizeTest>::default(),
            Box::<super::window_with_panels::WindowWithPanels>::default(),
        ])
    }
}

impl Demos {
    pub fn from_demos(demos: Vec<Box<dyn Demo + Sync + Send>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::WidgetGallery::default()
                .name()
                .to_owned(),
        );

        Self { demos, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            ui.toggle_value(&mut is_open, demo.name());
            set_open(open, demo.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Tests {
    #[cfg_attr(feature = "serde", serde(skip))]
    demos: Vec<Box<dyn Demo + Sync + Send>>,

    open: BTreeSet<String>,
}

impl Default for Tests {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::<super::tests::CursorTest>::default(),
            Box::<super::highlighting::Highlighting>::default(),
            Box::<super::tests::IdTest>::default(),
            Box::<super::tests::InputTest>::default(),
            Box::<super::layout_test::LayoutTest>::default(),
            Box::<super::tests::ManualLayoutTest>::default(),
            Box::<super::tests::TableTest>::default(),
        ])
    }
}

impl Tests {
    pub fn from_demos(demos: Vec<Box<dyn Demo + Sync + Send>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::WidgetGallery::default()
                .name()
                .to_owned(),
        );

        Self { demos, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            ui.toggle_value(&mut is_open, demo.name());
            set_open(open, demo.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindowsData {
    about_is_open: bool,
    about: About,
    demos: Demos,
    tests: Tests,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Clone, Default)]
pub struct DemoWindows {
    data: Arc<RwLock<DemoWindowsData>>,
}

impl Default for DemoWindowsData {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
            demos: Default::default(),
            tests: Default::default(),
        }
    }
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        if is_mobile(ctx) {
            self.mobile_ui(ctx);
        } else {
            self.desktop_ui(ctx);
        }
    }

    fn mobile_ui(&mut self, ctx: &Context) {
        let mut about_is_open = self.data.read().unwrap().about_is_open;
        if about_is_open {
            let screen_size = ctx.input(|i| i.screen_rect.size());
            let default_width = (screen_size.x - 20.0).min(400.0);

            let close = Arc::new(RwLock::new(false));
            let close_ = close.clone();
            let clone = self.clone();
            egui::Window::new(self.data.read().unwrap().about.name())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .default_width(default_width)
                .default_height(ctx.available_rect().height() - 46.0)
                .vscroll(true)
                .open(&mut about_is_open)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    let close = close.clone();
                    clone.data.write().unwrap().about.ui(ui);
                    ui.add_space(12.0);
                    ui.vertical_centered_justified(|ui| {
                        if ui
                            .button(egui::RichText::new("Continue to the demo!").size(20.0))
                            .clicked()
                        {
                            *close.write().unwrap() = true;
                        }
                    });
                });
            self.data.write().unwrap().about_is_open &= !*close_.read().unwrap();
        } else {
            self.mobile_top_bar(ctx);
            self.show_windows(ctx);
        }
    }

    fn mobile_top_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let font_size = 16.5;

                ui.menu_button(egui::RichText::new("⏷ demos").size(font_size), |ui| {
                    ui.set_style(ui.ctx().style()); // ignore the "menu" style set by `menu_button`.
                    self.demo_list_ui(ui);
                    if ui.ui_contains_pointer() && ui.input(|i| i.pointer.any_click()) {
                        ui.close_menu();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    use egui::special_emojis::{GITHUB, TWITTER};
                    ui.hyperlink_to(
                        egui::RichText::new(TWITTER).size(font_size),
                        "https://twitter.com/ernerfeldt",
                    );
                    ui.hyperlink_to(
                        egui::RichText::new(GITHUB).size(font_size),
                        "https://github.com/emilk/egui",
                    );
                });
            });
        });
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{} egui on GitHub", GITHUB),
                    "https://github.com/emilk/egui",
                );
                ui.hyperlink_to(
                    format!("{} @ernerfeldt", TWITTER),
                    "https://twitter.com/ernerfeldt",
                );

                ui.separator();

                self.demo_list_ui(ui);
            });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                file_menu_button(ui);
            });
        });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        let data = &mut *self.data.write().unwrap();
        data.about.show(ctx, &mut data.about_is_open);
        data.demos.windows(ctx);
        data.tests.windows(ctx);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let data = &mut *self.data.write().unwrap();
                ui.toggle_value(&mut data.about_is_open, data.about.name());

                ui.separator();
                data.demos.checkboxes(ui);
                ui.separator();
                data.tests.checkboxes(ui);
                ui.separator();

                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }
}

// ----------------------------------------------------------------------------

fn file_menu_button(ui: &mut Ui) {
    let organize_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::O);
    let reset_shortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL | Modifiers::SHIFT, egui::Key::R);

    // NOTE: we must check the shortcuts OUTSIDE of the actual "File" menu,
    // or else they would only be checked if the "File" menu was actually open!

    if ui.input_mut(|i| i.consume_shortcut(&organize_shortcut)) {
        ui.ctx().memory_mut(|mem| mem.reset_areas());
    }

    if ui.input_mut(|i| i.consume_shortcut(&reset_shortcut)) {
        ui.ctx().memory_mut(|mem| *mem = Default::default());
    }

    ui.menu_button("File", |ui| {
        ui.set_min_width(220.0);
        ui.style_mut().wrap = Some(false);

        // On the web the browser controls the zoom
        #[cfg(not(target_arch = "wasm32"))]
        {
            egui::gui_zoom::zoom_menu_buttons(ui, None);
            ui.separator();
        }

        if ui
            .add(
                egui::Button::new("Organize Windows")
                    .shortcut_text(ui.ctx().format_shortcut(&organize_shortcut)),
            )
            .clicked()
        {
            ui.ctx().memory_mut(|mem| mem.reset_areas());
            ui.close_menu();
        }

        if ui
            .add(
                egui::Button::new("Reset egui memory")
                    .shortcut_text(ui.ctx().format_shortcut(&reset_shortcut)),
            )
            .on_hover_text("Forget scroll, positions, sizes etc")
            .clicked()
        {
            ui.ctx().memory_mut(|mem| *mem = Default::default());
            ui.close_menu();
        }
    });
}
