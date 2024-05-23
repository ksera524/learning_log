use std::rc::Rc;

use crate::{
    browser,
    engine::{self, load_image, Cell, Game, Image, KeyState, Point, Rect, Renderer, Sheet, SpriteSheet},
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gloo_utils::format::JsValueSerdeExt;
use web_sys::HtmlImageElement;

use self::red_hat_boy_states::*;

const HEIGHT:i16 = 600;
pub trait Obstacle {
    fn check_intersection(&self,boy: &mut RedHatBoy);
    fn draw(&self,renderer:&Renderer);
    fn move_horizontally(&mut self,velocity:i16);
    fn right(&self) -> i16;
}

struct Platform {
    sheet:Rc<SpriteSheet>,
    position: Point,
}

impl Platform {
    fn new(sheet:Rc<SpriteSheet>,position:Point) -> Self {
        Platform {
            sheet,
            position,
        }
    }

    fn destination_box(&self) -> Rect {
        let platform = self
            .sheet
            .cell("13.png")
            .expect("13.png is not found");

        Rect::new_from_x_y(
            self.position.x, 
            self.position.y,
            platform.frame.w * 3,
            platform.frame.h)
    }

    fn bounding_boxes(&self) -> Vec<Rect> {
        const X_OFFSET:i16 = 60;
        const END_HEIGHT:i16 = 54;
        let destination = self.destination_box();
        let bounding_box_one = Rect::new_from_x_y(
            destination.x(),
            destination.y(),
            X_OFFSET,
            END_HEIGHT
        );


        let bounding_box_two = Rect::new_from_x_y(
            destination.x() + X_OFFSET, 
            destination.y(), 
            destination.width - (X_OFFSET * 2),
            destination.height);

        let bounding_box_three = Rect::new_from_x_y(
            destination.x() + destination.width - X_OFFSET,
            destination.y(), 
            X_OFFSET,
            END_HEIGHT);

        vec![bounding_box_one,bounding_box_two,bounding_box_three]
    }
}

impl Obstacle for Platform {
    fn draw(&self,renderer:&Renderer) {
        let platform = self.sheet
            .cell("13.png")
            .expect("13.png is not found");

            &self.sheet.draw(
                renderer, 
                &Rect::new_from_x_y(
                    platform.frame.x,
                    platform.frame.y, 
                    platform.frame.w * 3, 
                    platform.frame.h),
                &self.destination_box());
    }
    fn move_horizontally(&mut self,velocity:i16) {
        self.position.x += velocity;
    }

    fn check_intersection(&self,boy: &mut RedHatBoy) {
        if let Some(box_to_land_on) = self
            .bounding_boxes()
            .iter()
            .find(|&bounding_box| boy.bounding_box().intersects(bounding_box)) 
            {
                if boy.velocity_y() > 0 && boy.pos_y() < self.position.y {
                    boy.land_on(box_to_land_on.y());
                } else {
                    boy.knock_out();
                }
            }
    }


    fn right(&self) -> i16 {
        self.bounding_boxes()
            .last()
            .unwrap_or(&Rect::default())
            .right()
    }
}

pub struct Barrier {
    image: Image
}

impl Barrier {
    fn new(image:Image) -> Self {
        Barrier {
            image,
        }
    }
}

impl Obstacle for Barrier {
    fn draw(&self,renderer:&Renderer) {
        self.image.draw(renderer);
    }

    fn move_horizontally(&mut self,velocity:i16) {
        self.image.move_horizontally(velocity);
    }

    fn check_intersection(&self,boy:&mut RedHatBoy) {
        if boy.bounding_box().intersects(&self.image.bounding_box()) {
            boy.knock_out();
    }}

    fn right(&self) -> i16 {
        self.image.right()
    }
}

struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

impl RedHatBoy {
    fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet,
            image,
        }
    }

    fn draw(&self, renderer: &Renderer) {
        let sprite = self
            .current_sprint()
            .expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect::new_from_x_y(
                sprite.frame.x, 
                sprite.frame.y, 
                sprite.frame.w, 
                sprite.frame.h),
            &Rect::new_from_x_y(
                self.state_machine.context().position.x + sprite.sprite_source_size.x, 
                self.state_machine.context().position.y + sprite.sprite_source_size.y, 
                sprite.frame.w,
                sprite.frame.h)
        )
    }

    fn destination_box(&self) -> Rect {
        let sprite = self
            .current_sprint()
            .expect("Cell not found");

            Rect::new_from_x_y(
                self.state_machine.context().position.x + sprite.sprite_source_size.x,
                self.state_machine.context().position.y + sprite.sprite_source_size.y,
                sprite.frame.w,
                sprite.frame.h,
            )
    }

    fn bounding_box(&self) -> Rect {
        const X_OFFSET:i16 = 18;
        const Y_OFFSET:i16 = 14;
        const WIDTH_OFFSET:i16 = 28;

        Rect::new_from_x_y(
            self.destination_box().x() + X_OFFSET,
            self.destination_box().y() + Y_OFFSET,
            self.destination_box().width - WIDTH_OFFSET,
            self.destination_box().height - Y_OFFSET,
        )
    }

    fn frame_name(&self) -> String {
        format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        )
    }

    fn current_sprint(&self) -> Option<&Cell> {
        self.sprite_sheet.frames.get(&self.frame_name())
    }
 
    fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    fn run_right(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Run);
    }

    fn slide(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Slide);
    }

    fn jump(&mut self) {
        self.state_machine = self.state_machine.transition(Event::Jump);
    }

    fn land_on(&mut self, position: i16) {
        self.state_machine = self.state_machine.transition(Event::Land(position));
    }

    fn knock_out(&mut self) {
        self.state_machine = self.state_machine.transition(Event::KnockOut);
    }

    fn pos_y(&self) -> i16 {
        self.state_machine.context().position.y
    }

    fn velocity_y(&self) -> i16 {
        self.state_machine.context().velocity.y
    }

    fn walking_speed(&self) -> i16 {
        self.state_machine.context().velocity.x
    }
}

#[derive(Copy, Clone)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
    Sliding(RedHatBoyState<Sliding>),
    Jumping(RedHatBoyState<Jumping>),
    Falling(RedHatBoyState<Falling>),
    KnockOut(RedHatBoyState<KnockOut>),
}

pub enum Event {
    Run,
    Slide,
    Update,
    Jump,
    KnockOut,
    Land(i16),
}

impl RedHatBoyStateMachine {
    fn transition(self, event: Event) -> Self {
        match (self, event) {
            (RedHatBoyStateMachine::Idle(state), Event::Run) => state.run().into(),
            (RedHatBoyStateMachine::Running(state), Event::Jump) => state.jump().into(),
            (RedHatBoyStateMachine::Running(state), Event::Slide) => state.slide().into(),
            (RedHatBoyStateMachine::Running(state),Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Sliding(state),Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Jumping(state),Event::KnockOut) => state.knock_out().into(),
            (RedHatBoyStateMachine::Running(state),Event::Land(position)) => state.land_on(position).into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Land(position)) => state.land_on(position).into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Land(position)) => state.land_on(position).into(),
            (RedHatBoyStateMachine::Idle(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Running(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Sliding(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Jumping(state), Event::Update) => state.update().into(),
            (RedHatBoyStateMachine::Falling(state),Event::Update) => state.update().into(),
            _ => self,
        }
    }

    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
            RedHatBoyStateMachine::Sliding(state) => state.frame_name(),
            RedHatBoyStateMachine::Jumping(state) => state.frame_name(),
            RedHatBoyStateMachine::Falling(state) => state.frame_name(),
            RedHatBoyStateMachine::KnockOut(state) => state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.context(),
            RedHatBoyStateMachine::Running(state) => state.context(),
            RedHatBoyStateMachine::Sliding(state) => state.context(),
            RedHatBoyStateMachine::Jumping(state) => state.context(),
            RedHatBoyStateMachine::Falling(state) => state.context(),
            RedHatBoyStateMachine::KnockOut(state) => state.context(),
        }
    }

    fn update(self) -> Self {
        self.transition(Event::Update)
    }
}

impl From<RedHatBoyState<Idle>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Idle>) -> Self {
        RedHatBoyStateMachine::Idle(state)
    }
}

impl From<RedHatBoyState<Running>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Running>) -> Self {
        RedHatBoyStateMachine::Running(state)
    }
}

impl From<RedHatBoyState<Sliding>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Sliding>) -> Self {
        RedHatBoyStateMachine::Sliding(state)
    }
}

impl From<RedHatBoyState<Jumping>> for RedHatBoyStateMachine {
    fn from(state: RedHatBoyState<Jumping>) -> Self {
        RedHatBoyStateMachine::Jumping(state)
    }
}

impl From<RedHatBoyState<Falling>> for RedHatBoyStateMachine {
    fn from(state:RedHatBoyState<Falling>) -> Self {
        RedHatBoyStateMachine::Falling(state)
    }
}

impl From<RedHatBoyState<KnockOut>> for RedHatBoyStateMachine {
    fn from(state:RedHatBoyState<KnockOut>) -> Self {
        RedHatBoyStateMachine::KnockOut(state)
    }
}

impl From<SlidingEndState> for RedHatBoyStateMachine {
    fn from(end_state: SlidingEndState) -> Self {
        match end_state {
            SlidingEndState::Sliding(sliding_state) => sliding_state.into(),
            SlidingEndState::Complete(running_state) => running_state.into(),
        }
    }
}

impl From<JumpingEndState> for RedHatBoyStateMachine {
    fn from(end_state: JumpingEndState) -> Self {
        match end_state {
            JumpingEndState::Jumping(jumping_state) => jumping_state.into(),
            JumpingEndState::Landing(standing_state) => standing_state.into(),
        }
    }
}

impl From<FallingEndState> for RedHatBoyStateMachine {
    fn from(end_state: FallingEndState) -> Self {
        match end_state {
            FallingEndState::Falling(falling_state) => falling_state.into(),
            FallingEndState::Complete(knock_out_state) => knock_out_state.into(),
        }
    }
}

pub struct Walk {
    boy: RedHatBoy,
    backgrounds: [Image;2],
    obstacles: Vec<Box<dyn Obstacle>>,
    obstacle_sheet:Rc<SpriteSheet>,
}

impl Walk {
    fn velocity(&self) -> i16 {
        -self.boy.walking_speed()
    }
}

pub enum WalkTheDog {
    Loading,
    Loaded(Walk),
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}

const LOW_PLATFORM:i16 = 420;
const HIGH_PLATFORM:i16 = 375;
const FIRST_PLATFORM:i16 = 370;
#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let json = browser::fetch_json("rhb.json").await?;
                let background = load_image("BG.png").await?;
                let background_width = background.width() as i16;
                let stone = load_image("Stone.png").await?;
                let tiles = browser::fetch_json("tiles.json").await?;
                let sprite_sheet = Rc::new(SpriteSheet::new(
                    tiles.into_serde::<Sheet>()?,
                    engine::load_image("tiles.png").await?,
                ));
                let platform = Platform::new(
                    sprite_sheet.clone(),
                    Point { 
                        x: FIRST_PLATFORM,
                        y: HIGH_PLATFORM
                    },
                );

                let rhb = RedHatBoy::new(
                    json.into_serde::<Sheet>()?,
                    engine::load_image("rhb.png").await?,
                );

                Ok(Box::new(WalkTheDog::Loaded(Walk {
                    boy: rhb,
                    backgrounds: [
                        Image::new(background.clone(),Point { x: 0, y: 0 }),
                        Image::new(background,Point { x: background_width, y: 0 }),
                    ],
                    obstacles: vec![
                        Box::new(Barrier::new(Image::new(stone,Point { x: 150, y: 546 }))),
                        Box::new(platform),
                    ],
                    obstacle_sheet: sprite_sheet,
                })))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Game already initialized")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let WalkTheDog::Loaded(walk) = self {
            if keystate.is_pressed("ArrowRight") {
                walk.boy.run_right();
            }
            if keystate.is_pressed("ArrowDown") {
                walk.boy.slide();
            }
            if keystate.is_pressed("Space") {
                walk.boy.jump();
            }

            walk.boy.update();

            let velocity = walk.velocity();
            let [first_background,second_background] = &mut walk.backgrounds;
            first_background.move_horizontally(velocity);
            second_background.move_horizontally(velocity);

            if first_background.right() < 0 {
                first_background.set_x(second_background.right());
            }

            if second_background.right() < 0 {
                second_background.set_x(first_background.right());
            }

            walk.obstacles.retain(|obstacle| {
                obstacle.right() > 0
            });

            walk.obstacles.iter_mut().for_each(|obstacle| {
                obstacle.move_horizontally(velocity);
                obstacle.check_intersection(&mut walk.boy);
            });
        }
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect::new_from_x_y(0,0,600,600));

        if let WalkTheDog::Loaded(walk) = self {
            walk.backgrounds.iter().for_each(|background| {
                background.draw(renderer);
            });
            walk.boy.draw(renderer);
            walk.obstacles.iter().for_each(|obstacle| {
                obstacle.draw(renderer);
            })
        }
    }
}

mod red_hat_boy_states {
    use crate::engine::Point;

    use super::HEIGHT;

    const GRAVITY: i16 = 1;
    const FLOOR: i16 = 479;
    const PLAYER_HEGITH:i16 = HEIGHT - FLOOR;
    const STARTING_POINT:i16 = -20;
    const TERMINAL_VELOCITY : i16 = 20;
    const IDLE_FRAME_NAME: &str = "Idle";
    const IDLE_FRAMES: u8 = 29;
    const RUN_FRAME_NAME: &str = "Run";
    const RUN_FRAMES: u8 = 23;
    const RUNNING_SPEED: i16 = 4;
    const SLIDE_FRAME_NAME: &str = "Slide";
    const SLIDE_FRAMES: u8 = 14;
    const JUMPING_FRAME_NAME: &str = "Jump";
    const JUMPING_FRAMES: u8 = 35;
    const JUMP_SPEED: i16 = -25;
    const FALLING_FRAME_NAME: &str = "Dead";
    const FALLING_FRAMES: u8 = 29;

    #[derive(Copy, Clone)]
    pub struct RedHatBoyState<S> {
        context: RedHatBoyContext,
        _state: S,
    }

    impl<S> RedHatBoyState<S> {
        pub fn context(&self) -> &RedHatBoyContext {
            &self.context
        }
    }

    #[derive(Copy, Clone)]
    pub struct Idle;

    impl RedHatBoyState<Idle> {
        pub fn new() -> Self {
            RedHatBoyState {
                context: RedHatBoyContext {
                    frame: 0,
                    position: Point { 
                        x: STARTING_POINT, 
                        y: FLOOR },
                    velocity: Point { x: 0, y: 0 },
                },
                _state: Idle {},
            }
        }

        pub fn run(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame().run_right(),
                _state: Running {},
            }
        }

        pub fn frame_name(&self) -> &str {
            IDLE_FRAME_NAME
        }

        pub fn update(mut self) -> Self {
            self.context = self.context.update(IDLE_FRAMES);
            self
        }
    }

    #[derive(Copy, Clone)]
    pub struct Running;

    impl RedHatBoyState<Running> {
        pub fn frame_name(&self) -> &str {
            RUN_FRAME_NAME
        }
        pub fn update(mut self) -> Self {
            self.context = self.context.update(RUN_FRAMES);
            self
        }

        pub fn slide(self) -> RedHatBoyState<Sliding> {
            RedHatBoyState {
                context: self.context.reset_frame(),
                _state: Sliding {},
            }
        }

        pub fn jump(self) -> RedHatBoyState<Jumping> {
            RedHatBoyState {
                context: self.context.set_vertical_velocity(JUMP_SPEED).reset_frame(),
                _state: Jumping {},
            }
        }

        pub fn land_on(self,position:i16) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.set_on(position),
                _state: Running {},
            }
        }

        pub fn knock_out(self) -> RedHatBoyState<Falling> {
            RedHatBoyState {
                context: self.context.reset_frame().stop(),
                _state: Falling {},
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct Sliding;

    impl RedHatBoyState<Sliding> {
        pub fn frame_name(&self) -> &str {
            SLIDE_FRAME_NAME
        }

        pub fn update(mut self) -> SlidingEndState {
            self.context = self.context.update(SLIDE_FRAMES);

            if self.context.frame >= SLIDE_FRAMES {
                SlidingEndState::Complete(self.stand())
            } else {
                SlidingEndState::Sliding(self)
            }
        }

        pub fn stand(self) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame(),
                _state: Running {},
            }
        }

        pub fn land_on(self, position:i16) -> RedHatBoyState<Sliding> {
            RedHatBoyState {
                context: self.context.set_on(position),
                _state: Sliding {},
            }
        }

        pub fn knock_out(self) -> RedHatBoyState<Falling> {
            RedHatBoyState {
                context: self.context.reset_frame().stop(),
                _state: Falling {}
            }
        }
    }

    pub enum SlidingEndState {
        Sliding(RedHatBoyState<Sliding>),
        Complete(RedHatBoyState<Running>),
    }

    #[derive(Copy, Clone)]
    pub struct Jumping;

    impl RedHatBoyState<Jumping> {
        pub fn frame_name(&self) -> &str {
            JUMPING_FRAME_NAME
        }

        pub fn update(mut self) -> JumpingEndState {
            self.context = self.context.update(JUMPING_FRAMES);

            if self.context.position.y >= FLOOR {
                JumpingEndState::Landing(self.land_on(HEIGHT.into()))
            } else {
                JumpingEndState::Jumping(self)
            }
        }

        pub fn land_on(self, position:i16) -> RedHatBoyState<Running> {
            RedHatBoyState {
                context: self.context.reset_frame().set_on(position),
                _state: Running {},
            }
        }

        pub fn knock_out(self) -> RedHatBoyState<Falling> {
            RedHatBoyState {
                context: self.context.reset_frame().stop(),
                _state: Falling {}
            }
        }
    }

    pub enum JumpingEndState {
        Jumping(RedHatBoyState<Jumping>),
        Landing(RedHatBoyState<Running>),
    }

    #[derive(Copy, Clone)]
    pub struct Falling;

    impl RedHatBoyState<Falling> {
        pub fn frame_name(&self) -> &str {
            FALLING_FRAME_NAME
        }

        pub fn update(mut self) -> FallingEndState {
            self.context = self.context.update(FALLING_FRAMES);

            if self.context.frame >= FALLING_FRAMES {
                FallingEndState::Complete(self.knock_out())
            } else {
                FallingEndState::Falling(self)
            }
        }

        fn knock_out(self) -> RedHatBoyState<KnockOut> {
            RedHatBoyState {
                context: self.context,
                _state: KnockOut {},
            }
        }
    }

    pub enum FallingEndState {
        Falling(RedHatBoyState<Falling>),
        Complete(RedHatBoyState<KnockOut>),
    }
    
    #[derive(Copy, Clone)]
    pub struct KnockOut;

    impl RedHatBoyState<KnockOut> {
        pub fn frame_name(&self) -> &str {
            FALLING_FRAME_NAME
        }       
    }

    #[derive(Copy, Clone)]
    pub struct RedHatBoyContext {
        pub frame: u8,
        pub position: Point,
        pub velocity: Point,
    }

    impl RedHatBoyContext {
        pub fn update(mut self, frame_count: u8) -> Self {
            if self.velocity.y < TERMINAL_VELOCITY {
                self.velocity.y += GRAVITY;
            }

            if self.frame < frame_count {
                self.frame += 1;
            } else {
                self.frame = 0;
            }

            self.position.y += self.velocity.y;

            if self.position.y > FLOOR {
                self.position.y = FLOOR;
            }

            self
        }

        fn reset_frame(mut self) -> Self {
            self.frame = 0;
            self
        }

        fn run_right(mut self) -> Self {
            self.velocity.x += RUNNING_SPEED;
            self
        }

        fn set_vertical_velocity(mut self, velocity: i16) -> Self {
            self.velocity.y = velocity;
            self
        }

        fn stop(mut self) -> Self {
            self.velocity.x = 0;
            self.velocity.y = 0;
            self
        }

        fn set_on(mut self, position: i16) -> Self{
            let position = position - PLAYER_HEGITH;
            self.position.y = position;
            self
        }
    }
}
