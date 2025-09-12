use eframe::egui;
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};

#[derive(Clone)]
struct Pane {
    title: String,
}

#[derive(Default)]
struct MyBehavior;

impl Behavior<Pane> for MyBehavior {
    fn pane_ui(&mut self, ui: &mut egui::Ui, _id: TileId, pane: &mut Pane) -> UiResponse {
        // Editable field for the title
        ui.horizontal(|ui| {
            //ui.label("Title:");
            let _ = ui.add(
                egui::TextEdit::singleline(&mut pane.title)
                    .hint_text("Enter a title...")
                    .desired_width(200.0),
            );
        });
        ui.separator();

        // (You can add the main pane content here)
        UiResponse::None
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.title.clone().into()
    }
}

struct App {
    tree: Tree<Pane>,
    behavior: MyBehavior,
}

impl App {
    fn new() -> Self {
        let mut tiles = Tiles::default();

        let h_left  = tiles.insert_pane(Pane { title: "H: Left".into() });
        let h_right = tiles.insert_pane(Pane { title: "H: Right".into() });
        let horizontal = tiles.insert_horizontal_tile(vec![h_left, h_right]);

        let v_top    = tiles.insert_pane(Pane { title: "V: Top".into() });
        let v_middle = tiles.insert_pane(Pane { title: "V: Middle".into() });
        let v_bottom = tiles.insert_pane(Pane { title: "V: Bottom".into() });
        let vertical = tiles.insert_vertical_tile(vec![v_top, v_middle, v_bottom]);

        let root_tabs = tiles.insert_tab_tile(vec![horizontal, vertical]);

        let tree = Tree::new("tabs_linear_demo", root_tabs, tiles);
        let behavior = MyBehavior::default();

        Self { tree, behavior }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native = eframe::NativeOptions::default();
    eframe::run_native(
        "Tabs → Horizontal(2) / Vertical(3)",
        native,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
