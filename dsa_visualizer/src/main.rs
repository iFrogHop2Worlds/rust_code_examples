mod algorithm;
mod bfs;
mod dfs;
mod merge_sort;
mod heap_sort;
mod quicc_sort;
mod dijkstra;
mod kruskal;
mod bellman_ford;
mod floyds_cycle_detection;
mod longest_common_sequence;
mod knapsack;
mod kmp;
mod rabin_carp;
mod gradient_descent;
mod euclidean;
mod btree;
mod bplus_tree;

use std::time::Duration;
use algorithm::Algorithm;
use bfs::BFSVisualizer;
use eframe::egui;
use crate::dfs::DFSVisualizer;

fn main() {
    let ctx = egui::Context::default();
    let mut size = ctx.used_size();
    size.x = 1200.00;
    size.y = 720.00;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size(size),
        ..Default::default()
    };
    eframe::run_native(
        "DSA Visualizer",
        options,
        Box::new(|_cc| Ok(Box::new(DSAVisualizer::default()))),
    )
        .expect("Unexpected error in running the application");
}

struct DSAVisualizer {
    current_scene: String,
    current_algorithm: Option<Box<dyn Algorithm>>,
}

impl Default for DSAVisualizer {
    fn default() -> Self {
        Self {
            current_scene: String::new(),
            current_algorithm: None,
        }
    }
}

impl eframe::App for DSAVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.current_scene.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.heading("Algorithms");
                    let button_size = egui::Vec2::new(320.0, 0.0);
                    if ui.add_sized(button_size, egui::Button::new("Breadth First Search (BFS)")).clicked() {
                        self.current_scene = "BFS".to_string();
                        self.current_algorithm = Some(Box::new(BFSVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Depth First Search (DFS)")).clicked() {
                        self.current_scene = "DFS".to_string();
                        self.current_algorithm = Some(Box::new(DFSVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Dijkstra's Algorithm")).clicked() {
                        self.current_scene = "Dijkstra".to_string();
                        self.current_algorithm = Some(Box::new(dijkstra::DijkstraVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Merge Sort")).clicked() {
                        self.current_scene = "Merge Sort".to_string();
                        self.current_algorithm = Some(Box::new(merge_sort::MergeSortVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Heap Sort")).clicked() {
                        self.current_scene = "Heap Sort".to_string();
                        self.current_algorithm = Some(Box::new(heap_sort::HeapSortVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Gradient Descent")).clicked() {
                        self.current_scene = "Gradient Descent".to_string();
                        self.current_algorithm = Some(Box::new(gradient_descent::GradientDescentVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Longest Common Sequence")).clicked() {
                        self.current_scene = "Longest Common Sequence".to_string();
                        self.current_algorithm = Some(Box::new(longest_common_sequence::LCSVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("Euclidean Algorithm")).clicked() {
                        self.current_scene = "Euclidean Algorithm".to_string();
                        self.current_algorithm = Some(Box::new(euclidean::EuclideanVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("B-tree Visual")).clicked() {
                        self.current_scene = "B-tree Visual".to_string();
                        self.current_algorithm = Some(Box::new(btree::BTreeVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                    if ui.add_sized(button_size, egui::Button::new("B+ tree Visual")).clicked() {
                        self.current_scene = "B+ tree".to_string();
                        self.current_algorithm = Some(Box::new(bplus_tree::BplusTreeVisualizer::new()));
                        self.current_algorithm.as_mut().unwrap().initialize();
                    }
                });

            } else {
                if let Some(algorithm) = &mut self.current_algorithm {
                    ui.heading(format!("{} Algorithm Visualization", self.current_scene));

                    if ui.button("Back").clicked() {
                        self.current_scene.clear();
                        self.current_algorithm = None;
                        return;
                    }

                    ui.separator();
                    ui.add_space(12.0);

                    algorithm.render(ui);

                    if ui.button("Start").clicked() {
                        algorithm.toggle_auto_traverse();
                        algorithm.start();
                    }

                    if ui.button("Pause").clicked() {
                        algorithm.toggle_auto_traverse();
                    }

                    if ui.button("Resume").clicked() {
                        algorithm.toggle_auto_traverse();
                    }

                    if ui.button("Next Step").clicked() {
                        algorithm.step();
                    }

                }
            }
        });

        if let Some(algorithm) = &mut self.current_algorithm {
            if algorithm.auto_play() {
                let now = std::time::Instant::now();
                if algorithm.last_step_time().map_or(true, |t| now.duration_since(t) >= Duration::from_secs(2)) {
                    println!("running algorithm");
                    algorithm.step();
                    algorithm.set_last_step_time(Some(now));
                }
            }
        }

        ctx.request_repaint();
    }
}

