use anyhow::Result;
use crossterm::event::KeyCode;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::engine;
use crate::progress::Progress;
use crate::scenarios::{self, Feedback, ScenarioConfig};

#[derive(PartialEq)]
pub enum Screen {
    Menu,
    Brief,
    Input,
    Simulating,
    Results,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,

    // Menu
    pub menu_selection: usize,
    pub chapters: Vec<scenarios::ChapterInfo>,

    // Progress
    pub progress: Progress,

    // Active scenario
    pub scenario: Option<ScenarioConfig>,
    pub premium_input: String,
    pub input_error: Option<String>,

    // Chapter 2: policy count
    pub policy_count_input: String,
    pub input_field: usize, // 0 = premium, 1 = policy count

    // Hints
    pub hints_revealed: usize,
    pub show_hints: bool,

    // Simulation
    pub sim_results: Vec<engine::YearResult>,
    pub sim_current_year: u32,
    pub sim_total_years: u32,
    pub sim_tick: u32,
    pub sim_done: bool,
    pub sim_surplus: f64,
    pub rng: StdRng,

    // Results
    pub feedback: Option<Feedback>,
    pub result_scroll: u16,
}

impl App {
    pub fn new() -> Result<Self> {
        let progress = Progress::load().unwrap_or_default();
        Ok(Self {
            screen: Screen::Menu,
            should_quit: false,
            menu_selection: progress.current_chapter.saturating_sub(1),
            chapters: scenarios::chapter_list(),
            progress,
            scenario: None,
            premium_input: String::new(),
            input_error: None,
            policy_count_input: "100".into(),
            input_field: 0,
            hints_revealed: 0,
            show_hints: false,
            sim_results: Vec::new(),
            sim_current_year: 0,
            sim_total_years: 0,
            sim_tick: 0,
            sim_done: false,
            sim_surplus: 0.0,
            rng: StdRng::from_entropy(),
            feedback: None,
            result_scroll: 0,
        })
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        if self.show_hints {
            match key {
                KeyCode::Esc | KeyCode::Char('h') => self.show_hints = false,
                _ => {}
            }
            return false;
        }

        match self.screen {
            Screen::Menu => self.handle_menu_key(key),
            Screen::Brief => self.handle_brief_key(key),
            Screen::Input => self.handle_input_key(key),
            Screen::Simulating => self.handle_sim_key(key),
            Screen::Results => self.handle_results_key(key),
        }

        self.should_quit
    }

    fn handle_menu_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_selection > 0 {
                    self.menu_selection -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_selection < self.chapters.len() - 1 {
                    self.menu_selection += 1;
                }
            }
            KeyCode::Enter => {
                let chapter = self.chapters[self.menu_selection].number;
                if self.progress.is_unlocked(chapter) {
                    if let Some(config) = scenarios::get_scenario(chapter) {
                        self.scenario = Some(config);
                        self.screen = Screen::Brief;
                        self.hints_revealed = 0;
                        self.show_hints = false;
                        self.premium_input.clear();
                        self.input_error = None;
                        self.policy_count_input = "100".into();
                        self.input_field = 0;
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_brief_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.screen = Screen::Menu;
                self.scenario = None;
            }
            KeyCode::Enter => {
                self.screen = Screen::Input;
            }
            KeyCode::Char('h') => {
                self.show_hints = true;
                if let Some(ref config) = self.scenario {
                    if self.hints_revealed < config.hints.len() {
                        self.hints_revealed += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_input_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.screen = Screen::Brief;
                self.input_error = None;
            }
            KeyCode::Tab => {
                if self.active_chapter() == 2 {
                    self.input_field = (self.input_field + 1) % 2;
                }
            }
            KeyCode::Enter => self.submit_input(),
            KeyCode::Char('h') => {
                self.show_hints = true;
                if let Some(ref config) = self.scenario {
                    if self.hints_revealed < config.hints.len() {
                        self.hints_revealed += 1;
                    }
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
                self.active_input_mut().push(c);
            }
            KeyCode::Backspace => {
                self.active_input_mut().pop();
            }
            _ => {}
        }
    }

    fn active_input_mut(&mut self) -> &mut String {
        if self.active_chapter() == 2 && self.input_field == 1 {
            &mut self.policy_count_input
        } else {
            &mut self.premium_input
        }
    }

    pub fn active_chapter(&self) -> usize {
        self.scenario.as_ref().map(|s| s.chapter).unwrap_or(0)
    }

    fn submit_input(&mut self) {
        let premium: f64 = match self.premium_input.parse() {
            Ok(v) if v > 0.0 => v,
            _ => {
                self.input_error = Some("Enter a valid premium amount".into());
                return;
            }
        };

        let config = match &self.scenario {
            Some(c) => c,
            None => return,
        };

        let num_policies = if config.chapter == 2 {
            match self.policy_count_input.parse::<u32>() {
                Ok(v) if v > 0 && v <= 1000 => v,
                _ => {
                    self.input_error = Some("Enter 1\u{2013}1000 policies".into());
                    return;
                }
            }
        } else {
            1
        };

        let policies = scenarios::create_policies(config, premium, num_policies);

        self.sim_results.clear();
        self.sim_current_year = 0;
        self.sim_total_years = config.num_years;
        self.sim_tick = 0;
        self.sim_done = false;
        self.sim_surplus = self.progress.company_surplus;
        self.feedback = None;
        self.input_error = None;

        // Pre-generate all years
        let mut surplus = self.sim_surplus;
        for year in 1..=config.num_years {
            let result = engine::simulate_year(
                year,
                &policies,
                config.expense_ratio,
                surplus,
                &mut self.rng,
            );
            surplus = result.ending_surplus;
            self.sim_results.push(result);
        }

        self.screen = Screen::Simulating;
    }

    fn handle_sim_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.scenario = None;
            }
            KeyCode::Char(' ') | KeyCode::Enter if self.sim_done => {
                self.generate_feedback();
                self.screen = Screen::Results;
                self.result_scroll = 0;
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                // Skip animation — reveal all
                self.sim_current_year = self.sim_total_years;
                self.sim_done = true;
            }
            _ => {}
        }
    }

    fn handle_results_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.screen = Screen::Menu;
                self.scenario = None;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.result_scroll = self.result_scroll.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.result_scroll += 1;
            }
            KeyCode::Char('r') => {
                self.screen = Screen::Input;
                self.premium_input.clear();
                self.input_error = None;
            }
            KeyCode::Enter => {
                if let Some(ref feedback) = self.feedback {
                    if feedback.passed {
                        let chapter = self.active_chapter();
                        self.progress.mark_complete(chapter);
                        if let Some(last) = self.sim_results.last() {
                            self.progress.company_surplus = last.ending_surplus;
                        }
                    }
                }
                self.screen = Screen::Menu;
                self.scenario = None;
            }
            _ => {}
        }
    }

    fn generate_feedback(&mut self) {
        if let Some(ref config) = self.scenario {
            let premium: f64 = self.premium_input.parse().unwrap_or(0.0);
            self.feedback = Some(scenarios::get_feedback(
                config.chapter,
                &self.sim_results,
                premium,
                config,
            ));
        }
    }

    pub fn tick(&mut self) {
        if self.screen == Screen::Simulating && !self.sim_done {
            self.sim_tick += 1;
            if self.sim_tick >= 5 {
                self.sim_tick = 0;
                if self.sim_current_year < self.sim_total_years {
                    self.sim_current_year += 1;
                }
                if self.sim_current_year >= self.sim_total_years {
                    self.sim_done = true;
                }
            }
        }
    }

    pub fn save_progress(&self) -> Result<()> {
        self.progress.save()
    }
}
