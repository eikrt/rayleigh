use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::pixels;
use std::time::Duration;


const SCREEN_HEIGHT: u32 = 600;
const SCREEN_WIDTH: u32 = 800;
const TILE_SIZE: f32 = 64.0;
const MAP_SIZE: usize = 32;


struct Player {
    x: f32,
    y: f32,
    fov: f32,
    half_fov: f32,
    angle: f32,
    speed_movement: f32,
    speed_rotation: f32,
}


fn render(canvas: &mut WindowCanvas, player: &mut Player) {
    let bgcolor = Color::RGB(0,0,0);
    let wallcolor = Color::RGB(50,150,50);
    canvas.set_draw_color(bgcolor);
    canvas.clear();
    canvas.set_draw_color(wallcolor);
    canvas.present();
    let mut map:[[i32;MAP_SIZE];MAP_SIZE] = [[1;MAP_SIZE];MAP_SIZE];
    map[1][1] = 1;
    map[1][2] = 0;
    map[1][3] = 0;
    map[1][4] = 0;
    map[1][5] = 0;
    map[1][6] = 0;
    
    map[1][7] = 0;
    map[2][7] = 0;
    map[4][7] = 0;
    map[5][7] = 0;
    map[5][7] = 0;
    map[6][7] = 0;
    map[5][7] = 0;
    // render raycast
    
    let half_width = SCREEN_WIDTH / 2;
    let half_height = SCREEN_HEIGHT as f32 / 2.0;
    let increment_angle = player.fov / SCREEN_WIDTH as f32;
    let mut ray_angle: f32 = player.angle - player.half_fov;
    for i in 0..SCREEN_WIDTH {
	let mut ray_x = player.x;
	let mut ray_y = player.y;
	let ray_cos = ray_angle.cos() / TILE_SIZE;
	let ray_sin = ray_angle.sin() / TILE_SIZE;
	let mut wall = 0;
	while wall == 0 {

	    ray_x += ray_cos;
	    ray_y += ray_sin;
	    if ray_x < 0.0 || ray_y < 0.0 || ray_x > MAP_SIZE as f32|| ray_y > MAP_SIZE as f32 {break;}
	    wall = map[ray_y.floor() as usize][ray_x.floor() as usize];
	}
	if wall == 0 {
	    continue;
	}
	let mut distance = (((player.x - ray_x).powf(2.0)) + (player.y - ray_y).powf(2.0)).sqrt();

	// fish eye

	distance = distance * (ray_angle - player.angle).cos();
	//slice height
	let wall_height = (half_height as f32 / distance).floor();
	
	
	
	
//	canvas.fill_rect(Rect::new(i as i32,(half_height as i32),1,(wall_height as f32) as u32));
	//	canvas.fill_rect(Rect::new(i as i32,(half_height as i32 - wall_height as i32),1,(wall_height as f32) as u32));
	
	canvas.draw_line(Line::new(i as i32, 1, 1, 1, wall_color));	
	ray_angle += increment_angle; 
	
    }
    
    canvas.present();
}



fn main_loop() -> Result<(), String> {
   //initialising windows and canvas 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rayleigh", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    //initialising gameplay things

    let mut player = Player {
	
	x: 2.0,
	y: 2.0,
	fov: 60.0,
	half_fov: 30.0,
	angle: 120.0,
	speed_movement: 0.1,
	speed_rotation:0.1,

    };


    

    //calculate render data

    
    
    


    
// event handling
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
	// event handling
	for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
		
		Event::KeyDown{keycode: Some(Keycode::W), ..} => {
		    
		    let player_cos = player.angle.cos() * player.speed_movement;
		    let player_sin = player.angle.sin() * player.speed_movement;
		    player.x += player_cos;
		    player.y += player_sin;
		}
		Event::KeyDown{keycode: Some(Keycode::A), ..} => {
		player.angle += player.speed_rotation;    
		}
		Event::KeyDown{keycode: Some(Keycode::S), ..} => {
		    
		    let player_cos = player.angle.cos() * player.speed_movement;
		    let player_sin = player.angle.sin() * player.speed_movement;
		    player.x -= player_cos;
		    player.y -= player_sin;
		}
		Event::KeyDown{keycode: Some(Keycode::D), ..} => {
		    
		    player.angle -= player.speed_rotation;    
		}
                _ => {}
            }
        }
	// render
        render(&mut canvas, &mut player);
	// sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn main() {

main_loop();
}
