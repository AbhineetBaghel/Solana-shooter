

turbo::cfg!(
    r#"
    name = "Coin Cat"
    version = "1.0.0"
    author = "Turbo"
    description = "Catch falling coins!"
    [settings]
    resolution = [256, 144]
"#
);


turbo::init! {
    struct GameState {
        frame: u32,
        last_munch_at: u32,
        cat_x: f32,
        cat_y: f32,
        cat_r: f32,
        taco_y: f32,
        lives: u32,
        coins: Vec<struct Coin  {
            x: f32,
            y: f32,
            vel: f32,
            radius: f32,
        }>,
        
        missiles: Vec<struct Missile {
            x: f32,
            y: f32,
            vel: f32,
            color: u32,
            radius: f32,
        }>,
    
        balls: Vec<struct Ball {
            x: f32,
            y: f32,
            vel_y: f32,
            color: u32,
            radius: f32,
        }>,
        score: u32,
    } = {
        Self {
            frame: 0,
            last_munch_at: 0,
            cat_x: 128.0,
            cat_y: 135.0,
            cat_r: 1.0,
            taco_y: 112.0,
            coins: vec![],
            score: 0,
            balls: Vec::new(),
            lives: 100,
            missiles: vec!(),
        }
    }
}


turbo::go! {
   
    let mut state = GameState::load();
    
    if state.lives > 0 {
        
        if gamepad(0).left.pressed() {
            state.cat_x -= 2.0;
        }
        if gamepad(0).right.pressed() {
            state.cat_x += 2.0;
        }

        
        if gamepad(0).up.pressed() {
            
            let ball = Ball {
                x: state.cat_x, 
                y: state.cat_y, 
                vel_y: -5.0, 
                color: 0xff00ff00, 
                radius: 2.0, 
            };
            state.balls.push(ball); 
        }
    }

    if rand() % 170 == 0 {
        let missile = Missile {
            x: (rand() % 256) as f32,
            y: 0.0,
            vel: ((rand() % 3) + 1) as f32,
            radius: ((rand() % 10) + 5) as f32,
            color : 0xff0000ff,
        };
        state.missiles.push(missile);
    }

    state.missiles.retain_mut(|missile| {
        if missile.y >= 144.0 {
           
            false
        } else {
          
            missile.y += missile.vel;
            true
        }
    });

    
    state.balls.iter_mut().for_each(|ball| {
        ball.y += ball.vel_y; 
        
    });

   
    
    for ball in &state.balls {
        state.coins.retain_mut(|coin| {
            let dx = ball.x - coin.x;
            let dy = ball.y - coin.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= ball.radius + coin.radius {
                state.score += 1; 
                false 
            } else {
                true 
            }
        });
    }

    
    for ball in &state.balls {
        circ!(
            x = ball.x as i32,
            y = ball.y as i32,
            d = (ball.radius * 2.0) as u32,
            fill = ball.color
        );
    }

    
    if rand() % 30 == 0 {
      
        let coin = Coin  {
            x: (rand() % 256) as f32,
            y: 0.0,
            vel: ((rand() % 1) + 1) as f32,
            radius: ((rand() % 10) + 5) as f32,
        };
        state.coins.push(coin);
    }

    
    let cat_center = (state.cat_x + state.cat_r, state.cat_y + state.cat_r);
    state.coins.retain_mut(|coin| {
        coin.y += coin.vel;

        
        let coin_center = (coin.x + coin.radius, coin.y + coin.radius);

        
        let dx = cat_center.0 - coin_center.0;
        let dy = cat_center.1 - coin_center.1;

        let distance = (dx * dx + dy * dy).sqrt();
        let radii_sum = state.cat_r + coin.radius;
        let radii_diff = (state.cat_r - coin.radius).abs();

        if radii_diff <= distance && distance <= radii_sum {
          
            state.score += 1;
            state.last_munch_at = state.frame;
            false 
        } else if coin.y < 144.0 + coin.radius * 2.0 {
            true 
        } else {
            false 
        }
    });

    state.coins.retain_mut(|coin| {
        if coin.y >= state.cat_y {
           
            if state.lives > 0 {
                state.lives -= 1;
                return false;
            }
        }
       
        coin.y += coin.vel;
        coin.y < 144.0 + coin.radius * 2.0
    });

   

   

    if state.score == 10 {
        clear(0x000000ff);
    }
    if state.score == 20 {
        clear(0x00ffffff);
    }
    if state.score == 50 {
        clear(0x0000ffff);
    }
    if state.score == 70 {
        clear(0x00ff00ff);
    }
    if state.score == 100 {
        clear(0xff00ffff);
    }
    if state.score == 150 {
        clear(0x000000ff);
    }

    if state.lives > 0 {
        
        let frame = (state.frame as i32) / 2;
        for col in 0..9{
            for row in 0..8{
                let x = col * 32;
                let y = row * 32;
                let x = ((x + frame) % (272 + 16)) - 32;
               
                sprite!("heart", x = x-100, y = y-350, fps = fps::FAST);
            }
        }
    }
    if state.lives == 0 {
        text!("Game Over", x = 90, y = 60, font = Font::L, color = 0xff0000ff);
    }

   
    if state.frame >= 64 && state.frame.saturating_sub(state.last_munch_at) <= 60 {
        rect!(w = 30, h = 10, x = (state.cat_x as i32) + 32, y = state.cat_y as i32);
        circ!(d = 10, x = (state.cat_x as i32) + 28, y = state.cat_y as i32);
        rect!(w = 10, h = 5, x = (state.cat_x as i32) + 28, y = (state.cat_y as i32) + 5);
        circ!(d = 10, x = (state.cat_x as i32) + 56, y = state.cat_y as i32);
        text!(
            "MUNCH!",
            x = (state.cat_x as i32) + 33,
            y = (state.cat_y as i32) + 3,
            font = Font::S,
            color = 0x000000ff
        );
    }

   
    sprite!(
        "final",
        x = (state.cat_x - state.cat_r) as i32,
        y = (state.cat_y - 16.0) as i32,
        fps = fps::FAST
    );
    if state.lives > 0 {
        
        for coin in &state.coins {
            circ!(
                x = coin.x as i32,
                y = (coin.y as i32) + 1,
                d = (coin.radius + 2.0) as u32,
                fill = 0x000000aa
            ); 
            circ!(
                x = coin.x as i32,
                y = coin.y as i32,
                d = (coin.radius + 1.0) as u32,
                fill = 0xf4d29cff
            ); 
            circ!(
                x = coin.x as i32,
                y = coin.y as i32,
                d = coin.radius as u32,
                fill = 0xdba463ff
            ); 
        }
    }

    for ball in &state.balls {
        circ!(
            x = ball.x as i32,
            y = (ball.y as i32) + 1,
            d = (ball.radius + 2.0) as u32,
            fill = 0xffffffff
        ); 
    }

    for missile in &state.missiles {
       
        rect!(
            x = missile.x as i32,
            y = missile.y as i32,
            w = 10 as u32,
            h = 20 as u32,
            fill = 0xff0000ff 
        );

        let cat_hit_box = (state.cat_x - state.cat_r, state.cat_y - state.cat_r);
    let missile_hit_box = (missile.x, missile.y);

    
    if check_collision(cat_hit_box, missile_hit_box, state.cat_r, missile.radius) {
        state.score = state.score.saturating_sub(10);
        
        
        text!("-10", x = 50, y = 60, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 122, y = 98, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 190, y = 65, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 110, y = 50, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 45, y = 122, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 167, y = 250, font = Font::L, color = 0xff0000ff); // Display -10 on collision
        text!("-10", x = 90, y = 10, font = Font::L, color = 0xff0000ff);
        text!("-10", x = 45, y = 20, font = Font::L, color = 0xff0000ff);

    
    }
    
    }

    fn check_collision(cat_hit_box: (f32, f32), missile_hit_box: (f32, f32), cat_radius: f32, missile_radius: f32) -> bool {
        let dx = cat_hit_box.0 - missile_hit_box.0;
        let dy = cat_hit_box.1 - missile_hit_box.1;
        let distance = (dx * dx + dy * dy).sqrt();
        let radii_sum = cat_radius + missile_radius;
    
        distance <= radii_sum

        
    }

    // sprite!(
    //     "taco",
    //     x = (state.cat_x - state.cat_r) as i32,
    //     y = (state.taco_y - 8.0) as i32,
    //     fps = fps::FAST
    // );

    
    text!(&format!("Score: {}", state.score), x = 10, y = 10, font = Font::L, color = 0xffffffff); 
    text!(&format!("Lives: {}", state.lives), x = 10, y = 30, font = Font::L, color = 0xffffffff);

 
    state.frame += 1;
    state.save();
}
