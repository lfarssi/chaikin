use macroquad::prelude::*;
use chaikin::chaikin_algo;
#[macroquad::main("Chaikin")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut steps:Vec<Vec<(f32, f32)>>  = Vec::new();
    let mut last_time = get_time();
    let mut current_step = 0;   
    let mut entered = false;
    let mut is_curved = false;
    loop {
        clear_background(BLACK);
        if !entered{
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                points.push((mx, my));
            }
        }
        

        for &(x, y) in &points {
            draw_circle(x, y, 3.0, GRAY);
            draw_circle(x, y, 1.0, BLACK);
        }
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        if is_key_pressed(KeyCode::Enter) {
            entered = true;
            steps.clear();
            let mut points_algo =points.clone();
            for _ in 0..7{
                steps.push(points_algo.clone());
                points_algo = chaikin_algo(points_algo);
            }
            last_time = get_time();
            current_step = 0;
        }
        if entered && !steps.is_empty(){
                if get_time() - last_time > 0.5{
                    current_step+=1;
                    if current_step >= steps.len() {
                        current_step = 0;
                    }
                    last_time=get_time();
                }
                let step = &steps[current_step];
                for i in 0..step.len() - 1 {
                    let (x1, y1) = step[i];
                    let (x2, y2) = step[i + 1];
                    draw_line(x1,y1,x2,y2,1.0, RED);
                }
                
            
            }
        if is_key_pressed(KeyCode::Delete){
            points.clear();
            entered=false;
            steps.clear();
            current_step = 0;

        }

        next_frame().await;
    }
}
