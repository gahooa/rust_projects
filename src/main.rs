use eframe::epi;
use eframe::egui;
//use rand::Rng;
use crate::egui::Rect;
use crate::egui::Pos2;
use egui::Stroke;
use egui::Color32;

struct Application {
    vx: f32,
    vy: f32,
    px: f32,
    py: f32,
    screen_rect: Rect,
    co: i32,
}

impl Default for Application {
    fn default() -> Self {
        Self {
           vx: 0.0,
           vy: 0.0,
           px: 400.0,
           py: 300.0,
           screen_rect: Rect{min: Pos2{x: 0.0, y: 0.0}, max: Pos2{x: 1000.0, y: 700.0}},
           co: 0,
        }
    }
}

impl epi::App for Application {
    fn name(&self) -> &str {
        "playground"
    }
 

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        ctx.set_pixels_per_point(1.5);
        //sets ppp

        ctx.request_repaint();
        //makes tick contstant
        if self.co > 0 {
            self.co -= 1
        } 
        if self.vy > 50.0 {
            self.vy -= 1.0
        } else if self.vy < -50.0 {
            self.vy += 1.0
        }
        if self.vx > 50.0 {
            self.vx -= 1.0
        } else if self.vx < -50.0 {
            self.vx += 1.0
        }
        //cooldown
        if self.py > 1490.0 || self.py < -1500.0 {
            self.vy = 0.0-self.vy
        }
        //wall bouncing
        if self.px > 1490.0 || self.px < -1500.0 {
            self.vx = 0.0-self.vx
        }
        //wall bouncing
        if self.py > 1490.0 {
            self.py = 1490.0
        } else if self.py < -1500.0 {
            self.py = -1500.0
        } 
        //makes it so you cant glitch through walls in x axis
        if self.px > 1490.0 {
            self.px = 1490.0
        } else if self.px < -1500.0 {
            self.px = -1500.0
        } 
        //same as above except y axis
        if self.vx > 0.0 {
            self.vx -= 0.1;
        }
        if self.vx < 0.0 {
            self.vx += 0.1;
        }
        if self.vx.abs() < 0.1 {
            self.vx = 0.0
        }
        //x deceleration
        if self.vy > 0.0 {
            self.vy -= 0.1;
        }
        if self.vy < 0.0 {
            self.vy += 0.1;
        }
        if self.vy.abs() < 0.1 {
            self.vy = 0.0
        }
        //y deceleration
        if ctx.input().key_down(egui::Key::W) {
            self.vy += -0.5;
        }
        if ctx.input().key_down(egui::Key::S) {
            self.vy += 0.5;
        }
        if ctx.input().key_down(egui::Key::A) {
            self.vx += -0.5;
        }
        if ctx.input().key_down(egui::Key::D) {
            self.vx += 0.5;
        }
        //direction accelerations
        if ctx.input().key_pressed(egui::Key::Space) {
            if self.co == 0 {
                self.vx = self.vx*2.0;
                self.vy = self.vy*2.0;
                self.co = 100;
            }
        }
        //speed jump with cooldown
        if ctx.input().key_down(egui::Key::Q) {
            self.vx = self.vx/2.0;
            self.vy = self.vy/2.0;
        }
        //slowdown
        self.px += self.vx;
        self.py += self.vy;
        //moves player

        egui::CentralPanel::default().show(ctx, |ui| {
            
            let painter = ui.painter();

            self.screen_rect = painter.clip_rect();

            for l in 0..10 {
                let i = l as f32;
                for k in 0..10 {
                    let j = k as f32;
                    painter.rect(
                        Rect{
                            min: Pos2{
                                x:(self.screen_rect.max.x/2.0)-self.px+i*300.0-1500.0,
                                y:(self.screen_rect.max.y/2.0)-self.py+j*300.0-1500.0
                            },
                            max: Pos2{
                                x:(self.screen_rect.max.x/2.0)-self.px+i*300.0-1210.0,
                                y:(self.screen_rect.max.y/2.0)-self.py+j*300.0-1210.0
                            },
                        }, 
                        0.0, 
                        Color32::DARK_GREEN, 
                        Stroke{
                            width: 12.0, 
                            color: Color32::from_rgb(128, 128, 128)
                        }
                    );
                }
            }
            painter.clip_rect();

            painter.circle(
                egui::Pos2{
                    x:self.screen_rect.max.x/2.0,
                    y:self.screen_rect.max.y/2.0
                }, 
                10.0, 
                Color32::BLACK, 
                Stroke{
                    width: 2.0, 
                    color: Color32::from_rgb(255, 255, 255)
                }
            );
            
            if ui.button("Quit").clicked() {
                frame.quit()
            };
        });
    }
}

fn main() {
    let app = Application::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x:1000.0, y:700.0}),
        ..eframe::NativeOptions::default()
    };
    
    eframe::run_native(Box::new(app), native_options);
}