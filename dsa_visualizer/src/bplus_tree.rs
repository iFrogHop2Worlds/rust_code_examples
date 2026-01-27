use crate::algorithm::Algorithm;
use eframe::egui;
use std::time::Instant;

const B: usize = 2; // Minimum degree (t). Max keys = 2t - 1, Max children = 2t

#[derive(Clone, Debug)]
struct BTreeNode {
    keys: Vec<i32>,
    values: Vec<String>,
    children: Vec<BTreeNode>,
    is_leaf: bool,
    is_root: bool,
    // next_leaf: Option<Box<BTreeNode>>,
}

pub struct BplusTreeVisualizer {
    root: Option<BTreeNode>,
    input_value: i32,
    auto_traverse: bool,
    last_step: Option<Instant>,
    history: Vec<String>,
}

impl BplusTreeVisualizer {
    pub fn new() -> Self {
        Self {
            root: None,
            input_value: 0,
            auto_traverse: false,
            last_step: None,
            history: vec!["Initialize a new B+ Tree".to_string()],
        }
    }

    fn insert(&mut self, key: i32) {
        if let Some(mut root) = self.root.take() {
            if root.keys.len() == 2 * B - 1 {
                let mut new_root = BTreeNode {
                    keys: Vec::new(),
                    values: vec!["def".to_string()],
                    children: vec![root],
                    is_leaf: false,
                    is_root: true,
                    // next_leaf: None,
                };
                new_root.children[0].is_root = false;
                self.split_child(&mut new_root, 0);
                self.insert_non_full(&mut new_root, key);
                self.root = Some(new_root);
            } else {
                self.insert_non_full(&mut root, key);
                root.is_root = true; 
                //root.is_leaf = true;
                self.root = Some(root);
            }
        } else {
            self.root = Some(BTreeNode {
                keys: vec![key],
                values: vec!["original".to_string()],
                children: Vec::new(),
                is_leaf: true,
                is_root: true,
                //next_leaf: None,
            });
        }
    }

        fn insert_non_full(&mut self, node: &mut BTreeNode, key: i32) {
            match node.keys.binary_search(&key) {
                Ok(_) if node.is_leaf => return, 
                Ok(idx) if !node.is_leaf => {
                    self.insert_to_child(node, idx + 1, key);
                }
                Err(idx) => {
                    if node.is_leaf {
                        node.keys.insert(idx, key);
                        let record = format!(
                            "Index: {}\nName: User_{}\nMarks: {}\nAge: {}",
                            key, key, rand::random::<u8>() % 100, (key % 50) + 18
                        );
                        node.values.insert(idx, record);
                    } else {
                        self.insert_to_child(node, idx, key);
                    }
                },
                Ok(_) => todo!()
            }
        }

        fn insert_to_child(&mut self, node: &mut BTreeNode, child_idx: usize, key: i32) {
            if node.children[child_idx].keys.len() == 2 * B - 1 {
                self.split_child(node, child_idx);
                let new_idx = if key >= node.keys[child_idx] {
                    child_idx + 1
                } else {
                    child_idx
                };
                self.insert_non_full(&mut node.children[new_idx], key);
            } else {
                self.insert_non_full(&mut node.children[child_idx], key);
            }
        }

        fn split_child(&mut self, parent: &mut BTreeNode, i: usize) {
            let mut y = parent.children.remove(i);
            let mid_key: i32;

            let z = if y.is_leaf {
                let z_keys = y.keys.drain(B..).collect::<Vec<i32>>();
                let z_values = y.values.drain(B..).collect::<Vec<String>>();
                mid_key = z_keys[0]; 

                BTreeNode {
                    keys: z_keys,
                    values: z_values,
                    children: Vec::new(),
                    is_leaf: true,
                    is_root: false,
                }
            } else {
                let mut z_keys: Vec<i32> = y.keys.drain(B..).collect();
                mid_key = y.keys.pop().expect("Internal split needs separator");
                let z_children = y.children.drain(B..).collect();

                BTreeNode {
                    keys: z_keys,
                    values: Vec::new(),
                    children: z_children,
                    is_leaf: false,
                    is_root: false,
                }
            };

            parent.keys.insert(i, mid_key);
            parent.children.insert(i, y);
            parent.children.insert(i + 1, z);
        }

    fn draw_node(
        &self,
        ui: &mut egui::Ui,
        painter: &egui::Painter,
        node: &BTreeNode,
        pos: egui::Pos2,
        level: f32,
        leaf_positions: &mut Vec<egui::Rect>
    ) {
        let card_width = 110.0;
        let card_height = 85.0;
        let internal_width = 45.0;
        let internal_height = 35.0;

        let total_node_width = if node.is_leaf {
            node.keys.len() as f32 * card_width
        } else {
            node.keys.len() as f32 * internal_width
        };

        let node_rect = egui::Rect::from_min_size(
            egui::pos2(pos.x - total_node_width / 2.0, pos.y),
            egui::vec2(total_node_width, if node.is_leaf { card_height } else { internal_height }),
        );
        
        if node.is_leaf { leaf_positions.push(node_rect); }
        
        for (i, key) in node.keys.iter().enumerate() {
            let x_offset = node_rect.min.x + (i as f32 * (if node.is_leaf { card_width } else { internal_width }));
            let item_rect = egui::Rect::from_min_size(
                egui::pos2(x_offset, node_rect.min.y),
                egui::vec2(if node.is_leaf { card_width } else { internal_width }, node_rect.height()),
            );
            
            painter.rect_filled(item_rect, 0.0, egui::Color32::WHITE);
            painter.rect_stroke(item_rect, 0.0, egui::Stroke::new(1.0, egui::Color32::BLACK));

            if node.is_leaf {
                let record_text = format!(
                    "Index: {}\nName: User_{}\nVal: {}",
                    key, key, node.values[i]
                );
                painter.text(
                    item_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    record_text,
                    egui::FontId::proportional(11.0),
                    egui::Color32::BLACK
                );
            } else {
                painter.text(
                    item_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    key.to_string(),
                    egui::FontId::proportional(14.0),
                    egui::Color32::BLACK
                );
            }
        }

        if !node.is_leaf {
            let child_y = pos.y + 120.0;
            let spread_factor = 368.0 / (level + 1.0);
            let start_x = pos.x - ((node.children.len() as f32 - 1.0) * spread_factor) / 2.0;

            for (i, child) in node.children.iter().enumerate() {
                let child_pos = egui::pos2(start_x + i as f32 * spread_factor, child_y);
                let origin_x = node_rect.min.x + (i as f32 * internal_width);
                let origin = egui::pos2(origin_x, node_rect.max.y);

                painter.arrow(
                    origin,
                    child_pos - origin,
                    egui::Stroke::new(1.5, egui::Color32::BLACK)
                );

                self.draw_node(ui, painter, child, child_pos, level + 1.0, leaf_positions);
            }
        }
    }
}

impl Algorithm for BplusTreeVisualizer {
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
            let mut leaf_rects = Vec::new();
            self.draw_node(ui, &painter, root, egui::pos2(response.rect.center().x, response.rect.top() + 40.0), 0.0, &mut leaf_rects);
            
            for pair in leaf_rects.windows(2) {
                let start_rect = pair[0];
                let end_rect = pair[1];

                let start_pt = egui::pos2(start_rect.max.x, start_rect.center().y);
                let end_pt = egui::pos2(end_rect.min.x, end_rect.center().y);

                painter.arrow(
                    start_pt,
                    end_pt - start_pt,
                    egui::Stroke::new(1.5, egui::Color32::from_rgb(0, 160, 255))
                );
            }
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