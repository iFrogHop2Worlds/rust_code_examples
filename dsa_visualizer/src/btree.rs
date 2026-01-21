use crate::algorithm::Algorithm;
use eframe::egui;
use std::time::Instant;

const B: usize = 2; // Minimum degree (t). Max keys = 2t - 1, Max children = 2t

#[derive(Clone)]
struct BTreeNode {
    keys: Vec<i32>,
    children: Vec<BTreeNode>,
    is_leaf: bool,
}

pub struct BTreeVisualizer {
    root: Option<BTreeNode>,
    input_value: i32,
    auto_traverse: bool,
    last_step: Option<Instant>,
    history: Vec<String>,
}

impl BTreeVisualizer {
    pub fn new() -> Self {
        Self {
            root: None,
            input_value: 0,
            auto_traverse: false,
            last_step: None,
            history: vec!["Initialize a new B-Tree".to_string()],
        }
    }

    fn insert(&mut self, key: i32) {
        if let Some(mut root) = self.root.take() {
            if root.keys.len() == 2 * B - 1 {
                let mut new_root = BTreeNode {
                    keys: Vec::new(),
                    children: vec![root],
                    is_leaf: false,
                };
                self.split_child(&mut new_root, 0);
                self.insert_non_full(&mut new_root, key);
                self.root = Some(new_root);
            } else {
                self.insert_non_full(&mut root, key);
                self.root = Some(root);
            }
        } else {
            self.root = Some(BTreeNode {
                keys: vec![key],
                children: Vec::new(),
                is_leaf: true,
            });
        }
    }

    fn insert_non_full(&mut self, node: &mut BTreeNode, key: i32) {
        let mut i = node.keys.len();
        if node.is_leaf {
            node.keys.push(0);
            while i > 0 && key < node.keys[i - 1] {
                node.keys[i] = node.keys[i - 1];
                i -= 1;
            }
            node.keys[i] = key;
        } else {
            while i > 0 && key < node.keys[i - 1] {
                i -= 1;
            }
            if node.children[i].keys.len() == 2 * B - 1 {
                self.split_child(node, i);
                if key > node.keys[i] {
                    i += 1;
                }
            }
            self.insert_non_full(&mut node.children[i], key);
        }
    }

    fn split_child(&mut self, parent: &mut BTreeNode, i: usize) {
        let mut y = parent.children.remove(i);
        let mut z_keys: Vec<i32> = y.keys.drain(B..).collect();
        let mid_key = y.keys.pop().expect("Node should have keys to split");

        let mut z_children = Vec::new();
        if !y.is_leaf {
            z_children = y.children.drain(B..).collect();
        }

        let z = BTreeNode {
            keys: z_keys,
            children: z_children,
            is_leaf: y.is_leaf,
        };

        parent.keys.insert(i, mid_key);
        parent.children.insert(i, y);
        parent.children.insert(i + 1, z);
    }

    fn draw_node(&self, ui: &mut egui::Ui, painter: &egui::Painter, node: &BTreeNode, pos: egui::Pos2, level: f32) {
        let key_width = 35.0;
        let key_height = 25.0;
        let total_node_width = node.keys.len() as f32 * key_width;

        let rect = egui::Rect::from_min_size(
            egui::pos2(pos.x - total_node_width / 2.0, pos.y),
            egui::vec2(total_node_width, key_height),
        );

        painter.rect_stroke(rect, 1.0, ui.visuals().window_stroke());

        for (i, key) in node.keys.iter().enumerate() {
            let key_rect_x = rect.min.x + i as f32 * key_width;
            let key_center = egui::pos2(key_rect_x + key_width / 2.0, rect.center().y);

            painter.text(
                key_center,
                egui::Align2::CENTER_CENTER,
                key.to_string(),
                egui::FontId::proportional(14.0),
                ui.visuals().text_color()
            );

            if i < node.keys.len() - 1 {
                let x = key_rect_x + key_width;
                painter.line_segment(
                    [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                    ui.visuals().window_stroke()
                );
            }
        }

        if !node.is_leaf {
            let child_y = pos.y + 80.0;
            let spread_factor = 250.0 / (level + 1.0).powi(2).max(1.0);
            let total_children_width = (node.children.len() as f32 - 1.0) * spread_factor;
            let start_x = pos.x - total_children_width / 2.0;

            for (i, child) in node.children.iter().enumerate() {
                let child_pos = egui::pos2(start_x + i as f32 * spread_factor, child_y);

                let anchor_x = rect.min.x + (i as f32 * key_width);
                painter.line_segment(
                    [egui::pos2(anchor_x, rect.max.y), egui::pos2(child_pos.x, child_pos.y)],
                    ui.visuals().window_stroke()
                );

                self.draw_node(ui, painter, child, child_pos, level + 1.0);
            }
        }
    }
}

impl Algorithm for BTreeVisualizer {
    fn initialize(&mut self) {
        self.root = None;
        self.history.clear();
    }

    fn step(&mut self) {
        let val = rand::random::<u32>() % 100;
        self.history.push(format!("Inserted {}", val));
        self.insert(val as i32);
    }

    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut self.input_value));
            if ui.button("Insert Value").clicked() {
                self.insert(self.input_value);
                self.history.push(format!("Manual Insert: {}", self.input_value));
            }
            if ui.button("Clear").clicked() {
                self.initialize();
            }
        });

        ui.separator();

        let (response, painter) = ui.allocate_painter(
            ui.available_size(),
            egui::Sense::hover(),
        );

        if let Some(ref root) = self.root {
            self.draw_node(ui, &painter, root, egui::pos2(response.rect.center().x, response.rect.top() + 40.0), 0.0);
        } else {
            painter.text(response.rect.center(), egui::Align2::CENTER_CENTER, "Tree is empty", egui::FontId::proportional(20.0), ui.visuals().text_color());
        }
    }

    fn auto_play(&self) -> bool { self.auto_traverse }
    fn toggle_auto_traverse(&mut self) { self.auto_traverse = !self.auto_traverse; }
    fn start(&mut self) { self.initialize(); }
    fn last_step_time(&self) -> Option<Instant> { self.last_step }
    fn set_last_step_time(&mut self, time: Option<Instant>) { self.last_step = time; }
}