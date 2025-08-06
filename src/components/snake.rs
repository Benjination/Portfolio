use yew::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
use gloo::timers::callback::Interval;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, JsValue};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use serde::{Serialize, Deserialize};
use gloo_net::http::Request;
// --- Leaderboard Types ---
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ScoreEntry {
    pub initials: String,
    pub score: u32,
}


const LOCAL_LEADERBOARD_KEY: &str = "snake_local_leaderboard";
const LEADERBOARD_SIZE: usize = 10;
const GLOBAL_LEADERBOARD_URL: &str = "https://example.com/api/snake-leaderboard"; // TODO: Replace with real endpoint

// --- Global Leaderboard ---
async fn fetch_global_leaderboard() -> Vec<ScoreEntry> {
    match Request::get(GLOBAL_LEADERBOARD_URL).send().await {
        Ok(resp) => {
            if let Ok(entries) = resp.json::<Vec<ScoreEntry>>().await {
                entries
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}

async fn submit_global_score(entry: &ScoreEntry) {
    let req = Request::post(GLOBAL_LEADERBOARD_URL)
        .header("Content-Type", "application/json");
    let req = req.body(serde_json::to_string(entry).unwrap());
    if let Ok(req) = req {
        let _ = req.send().await;
    }
}

fn load_local_leaderboard() -> Vec<ScoreEntry> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    if let Ok(Some(data)) = storage.get_item(LOCAL_LEADERBOARD_KEY) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

fn save_local_leaderboard(entries: &[ScoreEntry]) {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let data = serde_json::to_string(entries).unwrap();
    let _ = storage.set_item(LOCAL_LEADERBOARD_KEY, &data);
}

fn qualifies_for_leaderboard(score: u32, leaderboard: &[ScoreEntry]) -> bool {
    if leaderboard.len() < LEADERBOARD_SIZE {
        return score > 0;
    }
    leaderboard.iter().any(|entry| score > entry.score)
}

fn insert_score(leaderboard: &mut Vec<ScoreEntry>, entry: ScoreEntry) {
    leaderboard.push(entry);
    leaderboard.sort_by(|a, b| b.score.cmp(&a.score));
    leaderboard.truncate(LEADERBOARD_SIZE);
}

#[derive(Clone, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq)]
pub struct SnakeGame {
    snake: VecDeque<Position>,
    food: Position,
    direction: Direction,
    direction_queue: VecDeque<Direction>,
    score: u32,
    game_over: bool,
    paused: bool,
    started: bool,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Position { x: 10, y: 10 });
        snake.push_back(Position { x: 9, y: 10 });
        snake.push_back(Position { x: 8, y: 10 });

        Self {
            snake,
            food: Position { x: 15, y: 15 },
            direction: Direction::Right,
            direction_queue: VecDeque::new(),
            score: 0,
            game_over: false,
            paused: false,
            started: false,
        }
    }

    pub fn update(&mut self) {
        if self.game_over || self.paused {
            return;
        }

        // Process direction queue for this tick
        if let Some(next_dir) = self.direction_queue.pop_front() {
            let opposite = match self.direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            };
            if next_dir != opposite && next_dir != self.direction {
                self.direction = next_dir;
            }
        }

        // Move the snake
        let head = self.snake.front().unwrap().clone();
        let new_head = match self.direction {
            Direction::Up => Position { x: head.x, y: head.y - 1 },
            Direction::Down => Position { x: head.x, y: head.y + 1 },
            Direction::Left => Position { x: head.x - 1, y: head.y },
            Direction::Right => Position { x: head.x + 1, y: head.y },
        };

        web_sys::console::log_4(
            &"Snake update:".into(),
            &format!("head=({},{})", head.x, head.y).into(),
            &format!("new_head=({},{})", new_head.x, new_head.y).into(),
            &format!("direction={:?}", self.direction).into()
        );

        // Check wall collision
        if new_head.x < 0 || new_head.x >= 30 || new_head.y < 0 || new_head.y >= 20 {
            web_sys::console::log_2(
                &"WALL COLLISION!".into(),
                &format!("new_head=({},{}) bounds=(0-29, 0-19)", new_head.x, new_head.y).into()
            );
            self.game_over = true;
            return;
        }

        // Check self collision
        if self.snake.iter().any(|segment| segment == &new_head) {
            web_sys::console::log_1(&"SELF COLLISION!".into());
            self.game_over = true;
            return;
        }

        // Move snake
        self.snake.push_front(new_head);

        // Check food collision
        if self.snake.front().unwrap() == &self.food {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    pub fn toggle_pause(&mut self) {
        if !self.game_over {
            self.paused = !self.paused;
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        // Prevent queueing the same direction or a direct reversal
        let last_dir = self.direction_queue.back().cloned().unwrap_or(self.direction);
        let opposite = match last_dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        web_sys::console::log_4(
            &"Direction change: (queue)".into(),
            &format!("current={:?}", self.direction).into(),
            &format!("requested={:?}", new_direction).into(),
            &format!("opposite={:?}", opposite).into()
        );
        if new_direction != opposite && new_direction != last_dir {
            self.direction_queue.push_back(new_direction);
            web_sys::console::log_2(&"Direction queued:".into(), &format!("{:?}", new_direction).into());
        } else {
            web_sys::console::log_1(&"Direction change blocked - would reverse into body or repeat".into());
        }
    }

    fn spawn_food(&mut self) {
        loop {
            let x = (js_sys::Math::random() * 30.0) as i32;
            let y = (js_sys::Math::random() * 20.0) as i32;
            let new_food = Position { x, y };
            
            if !self.snake.iter().any(|segment| segment == &new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn start(&mut self) {
        self.started = true;
        self.paused = false;
    }
}

#[function_component(Snake)]
pub fn snake() -> Html {
    let game_state = use_mut_ref(|| Rc::new(RefCell::new(SnakeGame::new())));
    let force_update = use_state(|| 0);
    let canvas_ref = use_node_ref();
    let interval = use_state(|| None::<Interval>);

    // Leaderboard state
    let local_leaderboard = use_state(|| load_local_leaderboard());
    let global_leaderboard = use_state(|| Vec::<ScoreEntry>::new());
    let leaderboard_mode = use_state(|| "local".to_string());
    let show_initials_modal = use_state(|| false);
    let initials_input = use_state(|| String::new());
    let pending_score = use_state(|| 0u32);

    // Fetch global leaderboard on mount or when toggled
    {
        let leaderboard_mode = leaderboard_mode.clone();
        let global_leaderboard_effect = global_leaderboard.clone();
        use_effect_with((*leaderboard_mode).clone(), move |mode| {
            if mode == "global" {
                wasm_bindgen_futures::spawn_local(async move {
                    let entries = fetch_global_leaderboard().await;
                    global_leaderboard_effect.set(entries);
                });
            }
            || {}
        });
    }

    // Game loop and rendering
    {
        let game_state = game_state.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with((), move |_| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            // Force initial render immediately
            render_game(&context, &game_state.borrow().borrow());
            web_sys::console::log_1(&"Snake game initialized and rendered".into());
            move || {}
        });
    }

    // Game loop that runs once when started
    {
        let game_state = game_state.clone();
        let canvas_ref = canvas_ref.clone();
        let interval = interval.clone();
        let force_update = force_update.clone();
        let show_initials_modal = show_initials_modal.clone();
        let local_leaderboard = local_leaderboard.clone();
        let global_leaderboard = global_leaderboard.clone();
        let leaderboard_mode = leaderboard_mode.clone();
        let pending_score = pending_score.clone();
        use_effect_with((), move |_| {
            web_sys::console::log_1(&"Setting up game loop...".into());
            let game_interval = {
                let game_state = game_state.clone();
                let canvas_ref = canvas_ref.clone();
                let force_update = force_update.clone();
                let show_initials_modal = show_initials_modal.clone();
                let local_leaderboard = local_leaderboard.clone();
                let global_leaderboard = global_leaderboard.clone();
                let leaderboard_mode = leaderboard_mode.clone();
                let pending_score = pending_score.clone();
                Interval::new(150, move || {
                    let game_rc = game_state.borrow();
                    let mut game = game_rc.borrow_mut();
                    if game.started && !game.game_over && !game.paused {
                        game.update();
                        force_update.set(*force_update + 1);
                        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                            if let Ok(Some(context)) = canvas.get_context("2d") {
                                if let Ok(context) = context.dyn_into::<CanvasRenderingContext2d>() {
                                    render_game(&context, &game);
                                }
                            }
                        }
                    } else if game.game_over && !*show_initials_modal {
                        // On game over, check for leaderboard for both local and global
                        let local = (*local_leaderboard).clone();
                        let global = (*global_leaderboard).clone();
                        let qualifies_local = qualifies_for_leaderboard(game.score, &local);
                        let qualifies_global = qualifies_for_leaderboard(game.score, &global);
                        if *leaderboard_mode == "global" && qualifies_global {
                            pending_score.set(game.score);
                            show_initials_modal.set(true);
                        } else if *leaderboard_mode == "local" && qualifies_local {
                            pending_score.set(game.score);
                            show_initials_modal.set(true);
                        }
                    }
                })
            };
            interval.set(Some(game_interval));
            move || {
                interval.set(None);
            }
        });
    }

    // Initial render only - game loop will handle subsequent renders
    {
        let canvas_ref = canvas_ref.clone();
        let game_state = game_state.clone();
        use_effect_with((), move |_| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Ok(Some(context)) = canvas.get_context("2d") {
                    if let Ok(context) = context.dyn_into::<CanvasRenderingContext2d>() {
                        render_game(&context, &game_state.borrow().borrow());
                        web_sys::console::log_1(&"Initial render completed".into());
                    }
                }
            }
        });
    }

    // Simple global keyboard controls using gloo::events
    {
        let game_state = game_state.clone();
        let force_update = force_update.clone();
        let show_initials_modal = show_initials_modal.clone();
        use_effect_with((), move |_| {
            web_sys::console::log_1(&"Setting up keyboard event listener".into());
            let window = web_sys::window().unwrap();
            let game_state = game_state.clone();
            let force_update = force_update.clone();
            let show_initials_modal = show_initials_modal.clone();
            let listener = EventListener::new(&window, "keydown", move |event| {
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
                let key = event.key();
                let initials_modal_open = *show_initials_modal;
                // Prevent default for game keys if not in initials modal, or for R/Arrows always
                match key.as_str() {
                    "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | " " => {
                        event.prevent_default();
                    }
                    "r" | "R" => {
                        if !initials_modal_open {
                            event.prevent_default();
                        }
                    }
                    _ => {}
                }
                if initials_modal_open {
                    // Don't allow game controls while entering initials
                    return;
                }
                let game_rc = game_state.borrow();
                let mut game = game_rc.borrow_mut();
                let mut updated = false;
                web_sys::console::log_4(
                    &"Game state before:".into(), 
                    &format!("started={}, game_over={}", game.started, game.game_over).into(),
                    &"Score:".into(),
                    &game.score.into()
                );
                match key.as_str() {
                    "ArrowUp" | "w" | "W" => {
                        if !game.started {
                            game.start();
                            web_sys::console::log_1(&"Game started with UP key".into());
                        } else {
                            web_sys::console::log_1(&"Changing direction to UP".into());
                        }
                        game.change_direction(Direction::Up);
                        updated = true;
                    }
                    "ArrowDown" | "s" | "S" => {
                        if !game.started {
                            game.start();
                            web_sys::console::log_1(&"Game started with DOWN key".into());
                        } else {
                            web_sys::console::log_1(&"Changing direction to DOWN".into());
                        }
                        game.change_direction(Direction::Down);
                        updated = true;
                    }
                    "ArrowLeft" | "a" | "A" => {
                        if !game.started {
                            game.start();
                            web_sys::console::log_1(&"Game started with LEFT key".into());
                        } else {
                            web_sys::console::log_1(&"Changing direction to LEFT".into());
                        }
                        game.change_direction(Direction::Left);
                        updated = true;
                    }
                    "ArrowRight" | "d" | "D" => {
                        if !game.started {
                            game.start();
                            web_sys::console::log_1(&"Game started with RIGHT key".into());
                        } else {
                            web_sys::console::log_1(&"Changing direction to RIGHT".into());
                        }
                        game.change_direction(Direction::Right);
                        updated = true;
                    }
                    " " => {
                        if game.started {
                            game.toggle_pause();
                            updated = true;
                        }
                    }
                    "r" | "R" => {
                        if game.game_over {
                            game.reset();
                            game.start();
                            web_sys::console::log_1(&"Game restarted".into());
                            updated = true;
                        }
                    }
                    _ => {}
                }
                if updated {
                    web_sys::console::log_4(
                        &"Game state after:".into(), 
                        &format!("started={}, game_over={}", game.started, game.game_over).into(),
                        &"Score:".into(),
                        &game.score.into()
                    );
                    force_update.set(*force_update + 1);
                    web_sys::console::log_1(&"Game state updated!".into());
                }
            });
            move || drop(listener)
        });
    }

    let start_game = {
        let game_state = game_state.clone();
        let force_update = force_update.clone();
        let show_initials_modal = show_initials_modal.clone();
        Callback::from(move |_| {
            let game_rc = game_state.borrow();
            let mut game = game_rc.borrow_mut();
            if !game.started {
                game.start();
            } else if game.game_over {
                game.reset();
                game.start();
            } else if game.paused {
                game.toggle_pause();
            } else {
                game.reset();
                game.start();
            }
            show_initials_modal.set(false);
            force_update.set(*force_update + 1);
        })
    };

    let canvas_click = {
        let game_state = game_state.clone();
        let force_update = force_update.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            web_sys::console::log_1(&"Canvas clicked!".into());
            let game_rc = game_state.borrow();
            let mut game = game_rc.borrow_mut();
            if !game.started {
                game.start();
                web_sys::console::log_1(&"Game started from canvas click".into());
                force_update.set(*force_update + 1);
            }
        })
    };

    let game_rc = game_state.borrow();
    let game = game_rc.borrow();
    let on_initials_input = {
        let initials_input = initials_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
            let filtered = input.chars().filter(|c| c.is_ascii_alphabetic()).take(3).collect::<String>().to_uppercase();
            initials_input.set(filtered);
        })
    };
    let on_initials_submit = {
        let initials_input = initials_input.clone();
        let local_leaderboard = local_leaderboard.clone();
        let global_leaderboard = global_leaderboard.clone();
        let show_initials_modal = show_initials_modal.clone();
        let pending_score = pending_score.clone();
        let leaderboard_mode = leaderboard_mode.clone();
        Callback::from(move |e: yew::events::SubmitEvent| {
            e.prevent_default();
            let initials = initials_input.trim().to_string();
            if initials.len() >= 1 && initials.len() <= 3 {
                let entry = ScoreEntry { initials: initials.clone(), score: *pending_score };
                if *leaderboard_mode == "local" {
                    let mut leaderboard = (*local_leaderboard).clone();
                    if qualifies_for_leaderboard(entry.score, &leaderboard) {
                        insert_score(&mut leaderboard, entry.clone());
                        save_local_leaderboard(&leaderboard);
                        local_leaderboard.set(leaderboard);
                    }
                } else if *leaderboard_mode == "global" {
                    let global_leaderboard = global_leaderboard.clone();
                    let leaderboard_mode = leaderboard_mode.clone();
                    let entry_clone = entry.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        submit_global_score(&entry_clone).await;
                        // After submitting, refetch global leaderboard
                        if *leaderboard_mode == "global" {
                            let entries = fetch_global_leaderboard().await;
                            global_leaderboard.set(entries);
                        }
                    });
                }
                show_initials_modal.set(false);
                initials_input.set(String::new());
                pending_score.set(0);
            }
        })
    };
    let on_initials_keydown = {
        let initials_input = initials_input.clone();
        let local_leaderboard = local_leaderboard.clone();
        let show_initials_modal = show_initials_modal.clone();
        let pending_score = pending_score.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let initials = initials_input.trim().to_string();
                if initials.len() >= 1 && initials.len() <= 3 {
                    let mut leaderboard = (*local_leaderboard).clone();
                    insert_score(&mut leaderboard, ScoreEntry { initials: initials.clone(), score: *pending_score });
                    save_local_leaderboard(&leaderboard);
                    local_leaderboard.set(leaderboard);
                    show_initials_modal.set(false);
                    initials_input.set(String::new());
                    pending_score.set(0);
                }
            }
        })
    };

    html! {
        <section id="snake-game" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"./snake.exe --retro-mode"}</span>
            </div>
            <div class="game-container">
                <div class="game-info">
                    <div class="score">
                        <span class="label">{"Score: "}</span>
                        <span class="value">{game.score}</span>
                    </div>
                    <div class="status">
                        {if !game.started {
                            html! { <span class="ready">{"READY - Click the game or press Start to begin!"}</span> }
                        } else if game.game_over {
                            html! { <span class="game-over">{"GAME OVER"}</span> }
                        } else if game.paused {
                            html! { <span class="paused">{"PAUSED"}</span> }
                        } else {
                            html! { <span class="playing">{"PLAYING - Game controls active"}</span> }
                        }}
                    </div>
                </div>
                <canvas 
                    ref={canvas_ref}
                    width="600"
                    height="400"
                    class="game-canvas"
                    onclick={canvas_click}
                    style="outline: none; cursor: pointer; border: 2px solid #00ff00; display: block;"
                />
                <div class="game-controls">
                    <div class="controls-info">
                        <h4>{"Controls:"}</h4>
                        <ul>
                            <li>{"Press Arrow Keys or WASD to start/move"}</li>
                            <li>{"Space - Pause/Resume"}</li>
                            <li>{"R - Restart (when game over)"}</li>
                            <li>{"Or click the button to start!"}</li>
                        </ul>
                    </div>
                    <div class="game-buttons">
                        {if !game.started {
                            html! {
                                <button class="restart-btn" onclick={start_game}>
                                    {"🎮 Start Game"}
                                </button>
                            }
                        } else if game.game_over {
                            html! {
                                <button class="restart-btn" onclick={start_game}>
                                    {"🎮 Restart Game"}
                                </button>
                            }
                        } else if game.paused {
                            html! {
                                <button class="restart-btn" onclick={start_game}>
                                    {"▶️ Resume Game"}
                                </button>
                            }
                        } else {
                            html! {
                                <button class="restart-btn" onclick={start_game}>
                                    {"🔄 New Game"}
                                </button>
                            }
                        }}
                    </div>
                </div>
                {if game.game_over {
                    html! {
                        <div class="gameover-overlay">
                            <div class="leaderboard-section gameover-leaderboard">
                                <div class="leaderboard-toggle">
                                    <button class={if *leaderboard_mode == "local" {"active"} else {""}}
                                        onclick={{
                                            let leaderboard_mode = leaderboard_mode.clone();
                                            Callback::from(move |_| leaderboard_mode.set("local".to_string()))
                                        }}>{"Local"}</button>
                                    <button class={if *leaderboard_mode == "global" {"active"} else {""}}
                                        onclick={{
                                            let leaderboard_mode = leaderboard_mode.clone();
                                            Callback::from(move |_| leaderboard_mode.set("global".to_string()))
                                        }}>{"Global"}</button>
                                </div>
                                <h4>{if *leaderboard_mode == "local" {"Local Leaderboard"} else {"Global Leaderboard"}}</h4>
                                <table class="leaderboard-table">
                                    <thead>
                                        <tr><th>{"Rank"}</th><th>{"Initials"}</th><th>{"Score"}</th></tr>
                                    </thead>
                                    <tbody>
                                        {if *leaderboard_mode == "local" {
                                            local_leaderboard.iter().enumerate().map(|(i, entry)| html! {
                                                <tr>
                                                    <td>{i+1}</td>
                                                    <td>{&entry.initials}</td>
                                                    <td>{entry.score}</td>
                                                </tr>
                                            }).collect::<Vec<_>>()
                                        } else {
                                            global_leaderboard.iter().enumerate().map(|(i, entry)| html! {
                                                <tr>
                                                    <td>{i+1}</td>
                                                    <td>{&entry.initials}</td>
                                                    <td>{entry.score}</td>
                                                </tr>
                                            }).collect::<Vec<_>>()
                                        }}
                                    </tbody>
                                </table>
                            </div>
                            {if *show_initials_modal {
                                html! {
                                    <div class="initials-modal-overlay">
                                        <div class="modal-content">
                                            <h3>{"New High Score!"}</h3>
                                            <form onsubmit={on_initials_submit.clone()}>
                                                <label for="initials-input">{"Enter your initials (1-3 letters): "}</label>
                                                <input
                                                    id="initials-input"
                                                    maxlength="3"
                                                    value={(*initials_input).clone()}
                                                    oninput={on_initials_input}
                                                    onkeydown={on_initials_keydown}
                                                    style="text-transform: uppercase; font-family: 'Fira Code', monospace;"
                                                    autofocus=true
                                                />
                                                <button type="submit">{"Save"}</button>
                                            </form>
                                        </div>
                                    </div>
                                }
                            } else { html!{} }}
                        </div>
                    }
                } else { html!{} }}
            </div>
        </section>
    }
}

fn render_game(context: &CanvasRenderingContext2d, game: &SnakeGame) {
    web_sys::console::log_3(
        &"Rendering game:".into(),
        &format!("started={}", game.started).into(),
        &format!("snake_length={}", game.snake.len()).into()
    );
    
    let cell_size = 20.0;
    let canvas_width = 600.0;
    let canvas_height = 400.0;
    
    // Clear canvas
    context.set_fill_style(&JsValue::from("#0a0a0a"));
    context.fill_rect(0.0, 0.0, canvas_width, canvas_height);
    
    // Draw grid lines
    context.set_stroke_style(&JsValue::from("#001100"));
    context.set_line_width(1.0);
    
    for i in 0..=30 {
        let x = i as f64 * cell_size;
        context.begin_path();
        context.move_to(x, 0.0);
        context.line_to(x, 400.0);
        context.stroke();
    }
    
    for i in 0..=20 {
        let y = i as f64 * cell_size;
        context.begin_path();
        context.move_to(0.0, y);
        context.line_to(600.0, y);
        context.stroke();
    }
    
    // Draw snake
    for (i, segment) in game.snake.iter().enumerate() {
        let x = segment.x as f64 * cell_size;
        let y = segment.y as f64 * cell_size;
        
        if i == 0 {
            // Head - brighter green
            context.set_fill_style(&JsValue::from("#00ff00"));
        } else {
            // Body - darker green
            context.set_fill_style(&JsValue::from("#00cc00"));
        }
        
        context.fill_rect(x + 1.0, y + 1.0, cell_size - 2.0, cell_size - 2.0);
    }
    
    // Draw food
    let food_x = game.food.x as f64 * cell_size;
    let food_y = game.food.y as f64 * cell_size;
    context.set_fill_style(&JsValue::from("#ff0000"));
    context.fill_rect(food_x + 2.0, food_y + 2.0, cell_size - 4.0, cell_size - 4.0);
    
    // Start screen overlay
    if !game.started {
        context.set_fill_style(&JsValue::from("rgba(0, 0, 0, 0.9)"));
        context.fill_rect(0.0, 0.0, 600.0, 400.0);
        
        context.set_fill_style(&JsValue::from("#00ff00"));
        context.set_font("32px 'Fira Code', monospace");
        context.set_text_align("center");
        context.fill_text("SNAKE GAME", 300.0, 160.0).unwrap();
        
        context.set_fill_style(&JsValue::from("#ffff00"));
        context.set_font("16px 'Fira Code', monospace");
        context.fill_text("Press Arrow Keys or WASD to start/move", 300.0, 200.0).unwrap();
        
        context.set_fill_style(&JsValue::from("#888888"));
        context.set_font("14px 'Fira Code', monospace");
        context.fill_text("Press SPACE to pause", 300.0, 240.0).unwrap();
    }
    
    // Game over overlay
    else if game.game_over {
        context.set_fill_style(&JsValue::from("rgba(0, 0, 0, 0.8)"));
        context.fill_rect(0.0, 0.0, 600.0, 400.0);
        
        context.set_fill_style(&JsValue::from("#ff0000"));
        context.set_font("32px 'Fira Code', monospace");
        context.set_text_align("center");
        context.fill_text("GAME OVER", 300.0, 180.0).unwrap();
        
        context.set_fill_style(&JsValue::from("#00ff00"));
        context.set_font("16px 'Fira Code', monospace");
        context.fill_text("Press R to restart", 300.0, 220.0).unwrap();
    }
    
    // Pause overlay
    else if game.paused {
        context.set_fill_style(&JsValue::from("rgba(0, 0, 0, 0.6)"));
        context.fill_rect(0.0, 0.0, 600.0, 400.0);
        
        context.set_fill_style(&JsValue::from("#ffff00"));
        context.set_font("24px 'Fira Code', monospace");
        context.set_text_align("center");
        context.fill_text("PAUSED", 300.0, 200.0).unwrap();
        
        context.set_fill_style(&JsValue::from("#888888"));
        context.set_font("14px 'Fira Code', monospace");
        context.fill_text("Press SPACE to resume", 300.0, 230.0).unwrap();
    }
}
