use eframe::egui;
use egui_tiles::{Behavior, TileId, Tiles, Tree, UiResponse};

#[derive(Clone)]
struct Pane {
    title: String,
}

struct MyBehavior;

impl Behavior<Pane> for MyBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.title.clone().into()
    }
    fn pane_ui(&mut self, ui: &mut egui::Ui, _id: TileId, pane: &mut Pane) -> UiResponse {
        ui.label(format!("Content of '{}'", pane.title));
        UiResponse::None
    }
}

struct App {
    tree: Tree<Pane>,
}

impl App {
    fn new() -> Self {
        let mut tiles = Tiles::default();

        // 3 panneaux simples
        let a = tiles.insert_pane(Pane { title: "A left".into() });
        let b = tiles.insert_pane(Pane { title: "B right-top".into() });
        let c = tiles.insert_pane(Pane { title: "C right-bottom".into() });

        // Split vertical à droite [B ; C]
        let right_v = tiles.insert_vertical_tile(vec![b, c]);

        // Split horizontal: [A | right_v]
        let root = tiles.insert_horizontal_tile(vec![a, right_v]);

        // Crée l'arbre
        let tree = Tree::new("tiles_demo", root, tiles);
        Self { tree }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut behavior = MyBehavior;
            self.tree.ui(&mut behavior, ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    let native = eframe::NativeOptions::default();
    eframe::run_native(
        "Tiles Lab",
        native,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
