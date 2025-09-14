// Simple example: panes have editable titles; container tab titles are NOT editable.
// Layout:
// Tabs
// ├── Linear (Horizontal)
// │   ├── Pane: "H: Left"
// │   └── Pane: "H: Right"
// └── Linear (Vertical)
//     ├── Pane: "V: Top"
//     ├── Pane: "V: Middle"
//     └── Pane: "V: Bottom"

use eframe::egui;
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};

#[derive(Clone)]
struct Pane {
    title: String,
}

// Minimal behavior: we only customize pane UI and pane tab title.
// Container (tab/linear) titles are left to egui_tiles' default rendering.
#[derive(Default)]
struct MyBehavior;

impl Behavior<Pane> for MyBehavior {
    // Draw the pane's content. Here we expose a single-line text box
    // that edits the pane title live.
    fn pane_ui(&mut self, ui: &mut egui::Ui, _id: TileId, pane: &mut Pane) -> UiResponse {
        ui.horizontal(|ui| {
            let _ = ui.add(
                egui::TextEdit::singleline(&mut pane.title)
                    .hint_text("Enter a title…")
                    .desired_width(200.0),
            );
        });
        ui.separator();
        UiResponse::None
    }

    // This is the text used for the pane's tab label.
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
        // Build tiles: first the two panes in a horizontal split
        let mut tiles: Tiles<Pane> = Tiles::default();

        let h_left  = tiles.insert_pane(Pane { title: "H: Left".into() });
        let h_right = tiles.insert_pane(Pane { title: "H: Right".into() });
        let horizontal = tiles.insert_horizontal_tile(vec![h_left, h_right]);

        // Then three panes in a vertical split
        let v_top    = tiles.insert_pane(Pane { title: "V: Top".into() });
        let v_middle = tiles.insert_pane(Pane { title: "V: Middle".into() });
        let v_bottom = tiles.insert_pane(Pane { title: "V: Bottom".into() });
        let vertical = tiles.insert_vertical_tile(vec![v_top, v_middle, v_bottom]);

        // Put both splits into a tab container
        let root_tabs = tiles.insert_tab_tile(vec![horizontal, vertical]);

        // Create the tree with a stable id string (used for persistence if needed)
        let tree = Tree::new("tabs_linear_demo", root_tabs, tiles);

        Self {
            tree,
            behavior: MyBehavior::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Render the tiles with our behavior. Pane titles are editable;
            // container tabs keep the default titles (e.g. "Horizontal", "Vertical").
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
