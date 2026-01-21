use std::collections::VecDeque;
use std::time::{Duration, Instant};
use eframe::egui;
use crate::algorithm::Algorithm;

pub struct BFSVisualizer {
    nodes: Vec<(usize, usize, usize)>, // Node ID, Parent ID, Level
    current_node: Option<usize>,
    visited: Vec<usize>,
    bfs_queue: VecDeque<usize>,
    tree_created: bool,
    auto_traverse: bool,
    last_step_time: Option<Instant>,
}

impl BFSVisualizer {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_node: None,
            visited: Vec::new(),
            bfs_queue: VecDeque::new(),
            tree_created: false,
            auto_traverse: false,
            last_step_time: None,
        }
    }

    fn create_example_tree(&mut self) {
        self.nodes.clear();
        self.nodes.push((1, 0, 0));
        let mut queue = VecDeque::new();
        queue.push_back((1, 0));

        let mut current_id = 2;
        while let Some((parent_id, level)) = queue.pop_front() {
            if level < 4 {
                // Create left child
                self.nodes.push((current_id, parent_id, level + 1));
                queue.push_back((current_id, level + 1));
                current_id += 1;

                // Create right child
                self.nodes.push((current_id, parent_id, level + 1));
                queue.push_back((current_id, level + 1));
                current_id += 1;
            }
        }

        self.current_node = None;
        self.visited.clear();
        self.bfs_queue.clear();
        self.tree_created = true;
        self.auto_traverse = false;
    }

    fn start_bfs(&mut self) {
        if self.tree_created {
            self.visited.clear();
            self.bfs_queue.clear();
            self.current_node = None;
            self.bfs_queue.push_back(1);
            self.auto_traverse = true;
            self.last_step_time = None;
        }
    }

    fn bfs_step(&mut self) {
        if let Some(current) = self.bfs_queue.pop_front() {
            self.current_node = Some(current);
            self.visited.push(current); // Mark the node as visited
            for &(child_id, parent_id, _) in self.nodes.iter() {
                if parent_id == current && !self.visited.contains(&child_id) {
                    self.bfs_queue.push_back(child_id);
                }
            }
        } else {
            self.current_node = None;
            self.auto_traverse = false;
        }
    }

        fn render_tree(&self, ui: &mut egui::Ui) {
            let total_levels = self.nodes.iter().map(|(_, _, level)| level).max().unwrap_or(&0) + 1;
            let node_spacing = 70.0;
            let screen_width = ui.available_width();
            let width = (2usize.pow(total_levels as u32 - 1) as f32 * node_spacing).max(800.0);
            let height = (total_levels as f32 * 120.0).max(600.0);
            let level_spacing = height / (total_levels as f32 + 1.0);
            let node_radius = 30.0;

            let mut levels: Vec<Vec<(usize, usize)>> = vec![Vec::new(); total_levels];
            for &(node_id, parent_id, level) in &self.nodes {
                levels[level].push((node_id, parent_id));
            }

            let mut positions = Vec::new();
            let tree_width = (levels.last().unwrap_or(&Vec::new()).len() as f32 - 1.0) * node_spacing;
            let center_offset = (screen_width - tree_width) / 2.0;

            for (level, nodes) in levels.iter().enumerate() {
                let level_count = nodes.len();
                let x_spacing = width / (level_count as f32 + 1.0);

                for (i, &(node_id, _)) in nodes.iter().enumerate() {
                    let x = center_offset + x_spacing * (i as f32 + 1.0);
                    let y = level_spacing * (level as f32 + 1.0);
                    positions.push((node_id, x, y));
                }
            }

            for &(node_id, parent_id, _) in &self.nodes {
                if let Some(&(_, x, y)) = positions.iter().find(|&&(id, _, _)| id == node_id) {
                    let color = if self.current_node == Some(node_id) {
                        egui::Color32::GREEN
                    } else if self.visited.contains(&node_id) {
                        egui::Color32::LIGHT_BLUE
                    } else {
                        egui::Color32::RED
                    };

                    ui.painter().circle_filled(egui::pos2(x, y), node_radius, color);

                    if parent_id != 0 {
                        if let Some(&(_, px, py)) = positions.iter().find(|&&(id, _, _)| id == parent_id) {
                            ui.painter().line_segment(
                                [egui::pos2(px, py + (node_radius)), egui::pos2(x, y - (node_radius))],
                                egui::Stroke::new(2.0, egui::Color32::GRAY),
                            );
                        }
                    }

                    ui.painter().text(
                        egui::pos2(x, y - 10.0),
                        egui::Align2::CENTER_BOTTOM,
                        format!("{}", node_id),
                        egui::TextStyle::Body.resolve(ui.style()),
                        egui::Color32::BLACK,
                    );
                }
            }
        }
}

impl Algorithm for BFSVisualizer {
    fn initialize(&mut self) {
        self.create_example_tree();
    }

    fn step(&mut self) {
        self.bfs_step();
    }

    fn render(&mut  self, ui: &mut egui::Ui) {
        self.render_tree(ui);
    }

    fn auto_play(&self) -> bool {
        self.auto_traverse
    }

    fn toggle_auto_traverse(&mut self) {
        self.auto_traverse = !self.auto_traverse;
    }

    fn start(&mut self)  {
        self.start_bfs();
    }
    fn last_step_time(&self) -> Option<Instant> {
        self.last_step_time
    }

    fn set_last_step_time(&mut self, time: Option<Instant>) {
        self.last_step_time = time;
    }
}