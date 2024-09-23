use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let flappy: Texture2D = load_texture("assets/pngegg.png").await.unwrap();
    fn jump(){




    }
    let mut xflappy: f32 = 0.0;
    let mut yflappy: f32 = 0.0;
    let mut flappy_state: bool = true;
    let mut flappy_grav: bool = true;

    set_fullscreen(true);
    loop {


        if is_key_down(KeyCode::Space) {
            yflappy -= 20.0;
            flappy_grav = false;
        }else{
            flappy_grav = true;
        }

        if flappy_state == true {
            xflappy += 10.0;
            
        }

        if flappy_grav == true {
            yflappy += 3.0;
        }
        



        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_texture(&flappy, xflappy, yflappy, WHITE);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
