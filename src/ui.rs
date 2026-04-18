use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, Cell, Clear, List, ListItem, Padding, Paragraph,
        Row, Table, Wrap,
    },
    Frame,
};

use crate::app::{App, Screen};
use crate::scenarios::FeedbackKind;

fn fmt_money(v: f64) -> String {
    let abs = v.abs();
    let integer = abs as u64;
    let s = integer.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    let formatted: String = result.chars().rev().collect();
    if v < 0.0 {
        format!("-${}", formatted)
    } else {
        format!("${}", formatted)
    }
}

fn fmt_money_signed(v: f64) -> String {
    let abs = v.abs();
    let integer = abs as u64;
    let s = integer.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    let formatted: String = result.chars().rev().collect();
    if v < 0.0 {
        format!("-${}", formatted)
    } else {
        format!("+${}", formatted)
    }
}

const BLUE: Color = Color::Rgb(70, 130, 180);
const GREEN: Color = Color::Green;
const RED: Color = Color::Red;
const YELLOW: Color = Color::Yellow;
const DIM: Color = Color::DarkGray;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    match app.screen {
        Screen::Menu => draw_menu(frame, app, area),
        Screen::Brief => draw_brief(frame, app, area),
        Screen::Input => draw_input(frame, app, area),
        Screen::Simulating => draw_simulating(frame, app, area),
        Screen::Results => draw_results(frame, app, area),
    }

    if app.show_hints {
        draw_hints_popup(frame, app, area);
    }
}

fn draw_menu(frame: &mut Frame, app: &App, area: Rect) {
    let outer = Block::default()
        .title(" ACTUARY ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .padding(Padding::new(2, 2, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Header
            Constraint::Min(5),   // Chapter list
            Constraint::Length(2), // Controls
        ])
        .split(inner);

    // Header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Pacific Shield Insurance Co.", Style::default().fg(BLUE).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("Property & Casualty"),
        Line::from(format!("Surplus: {}", fmt_money(app.progress.company_surplus))),
    ]);
    frame.render_widget(header, chunks[0]);

    // Chapter list
    let items: Vec<ListItem> = app
        .chapters
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let completed = app.progress.completed_chapters.contains(&ch.number);
            let unlocked = app.progress.is_unlocked(ch.number);
            let has_scenario = crate::scenarios::get_scenario(ch.number).is_some();

            let status = if completed {
                Span::styled(" \u{2713} ", Style::default().fg(GREEN))
            } else if unlocked && has_scenario {
                Span::styled(" \u{25cb} ", Style::default().fg(DIM))
            } else if unlocked {
                Span::styled(" - ", Style::default().fg(DIM))
            } else {
                Span::styled(" \u{1f512} ", Style::default().fg(DIM))
            };

            let style = if i == app.menu_selection {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else if !unlocked || !has_scenario {
                Style::default().fg(DIM)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                status,
                Span::styled(format!("{}. {}", ch.number, ch.title), style),
                Span::styled(format!("  {}", ch.subtitle), Style::default().fg(DIM)),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(" Chapters ").borders(Borders::ALL).border_style(Style::default().fg(DIM)));
    frame.render_widget(list, chunks[1]);

    // Controls
    let controls = Paragraph::new(Line::from(vec![
        Span::styled(" \u{2191}\u{2193} ", Style::default().fg(BLUE)),
        Span::raw("Navigate  "),
        Span::styled(" Enter ", Style::default().fg(BLUE)),
        Span::raw("Start  "),
        Span::styled(" q ", Style::default().fg(BLUE)),
        Span::raw("Quit"),
    ]));
    frame.render_widget(controls, chunks[2]);
}

fn draw_brief(frame: &mut Frame, app: &App, area: Rect) {
    let config = match &app.scenario {
        Some(c) => c,
        None => return,
    };

    let title = format!(" Chapter {}: {} ", config.chapter, config.title);
    let outer = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .padding(Padding::new(2, 2, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // Brief text + data
            Constraint::Length(2), // Controls
        ])
        .split(inner);

    // Build brief content
    let mut lines: Vec<Line> = Vec::new();

    for &text in &config.brief {
        if text.is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(text));
        }
    }

    lines.push(Line::from(""));

    // Property details
    lines.push(Line::from(Span::styled(
        "\u{2500}\u{2500} Property Details \u{2500}\u{2500}",
        Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(format!("  Value: {}", fmt_money(config.property_value))));
    lines.push(Line::from("  Type:  Single-family residential"));
    lines.push(Line::from(""));

    // Loss data table
    lines.push(Line::from(Span::styled(
        "\u{2500}\u{2500} Historical Loss Data (per year) \u{2500}\u{2500}",
        Style::default().fg(BLUE).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(vec![
        Span::styled("  Peril      ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("Frequency    ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("Avg Severity", Style::default().add_modifier(Modifier::BOLD)),
    ]));
    for peril in &config.perils {
        lines.push(Line::from(format!(
            "  {:<10} {:<12} {:.0}% of value",
            peril.name,
            format!("{:.1}%", peril.frequency * 100.0),
            peril.severity_pct * 100.0
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "  Expense ratio: {:.0}% of premium",
        config.expense_ratio * 100.0
    )));

    let brief = Paragraph::new(lines).wrap(Wrap { trim: false });
    frame.render_widget(brief, chunks[0]);

    // Controls
    let hint_count = format!("{}/{}", app.hints_revealed, config.hints.len());
    let controls = Paragraph::new(Line::from(vec![
        Span::styled(" Enter ", Style::default().fg(BLUE)),
        Span::raw("Set Premium  "),
        Span::styled(" h ", Style::default().fg(YELLOW)),
        Span::raw(format!("Hints ({})  ", hint_count)),
        Span::styled(" Esc ", Style::default().fg(BLUE)),
        Span::raw("Back"),
    ]));
    frame.render_widget(controls, chunks[1]);
}

fn draw_input(frame: &mut Frame, app: &App, area: Rect) {
    let config = match &app.scenario {
        Some(c) => c,
        None => return,
    };

    let title = format!(" Chapter {}: {} ", config.chapter, config.title);
    let outer = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .padding(Padding::new(2, 2, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Prompt
            Constraint::Length(3),  // Premium input
            Constraint::Length(3),  // Policy count (ch2) or spacer
            Constraint::Length(2),  // Error
            Constraint::Min(3),    // Quick reference
            Constraint::Length(2), // Controls
        ])
        .split(inner);

    let prompt = Paragraph::new("Set your annual premium for this policy.")
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(prompt, chunks[0]);

    // Premium input
    let premium_style = if app.input_field == 0 {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(DIM)
    };
    let premium_block = Block::default()
        .title(" Annual Premium ")
        .borders(Borders::ALL)
        .border_style(if app.input_field == 0 {
            Style::default().fg(BLUE)
        } else {
            Style::default().fg(DIM)
        });
    let premium_text = if app.premium_input.is_empty() && app.input_field == 0 {
        Span::styled("enter amount...", Style::default().fg(DIM))
    } else {
        Span::styled(format!("$ {}", app.premium_input), premium_style)
    };
    let premium_p = Paragraph::new(Line::from(premium_text)).block(premium_block);
    frame.render_widget(premium_p, chunks[1]);

    // Set cursor position for premium input
    if app.input_field == 0 {
        frame.set_cursor_position((
            chunks[1].x + 4 + app.premium_input.len() as u16,
            chunks[1].y + 1,
        ));
    }

    // Policy count (chapter 2 only)
    if config.chapter == 2 {
        let count_style = if app.input_field == 1 {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(DIM)
        };
        let count_block = Block::default()
            .title(" Number of Policies ")
            .borders(Borders::ALL)
            .border_style(if app.input_field == 1 {
                Style::default().fg(BLUE)
            } else {
                Style::default().fg(DIM)
            });
        let count_p = Paragraph::new(Line::from(Span::styled(
            &app.policy_count_input,
            count_style,
        )))
        .block(count_block);
        frame.render_widget(count_p, chunks[2]);

        if app.input_field == 1 {
            frame.set_cursor_position((
                chunks[2].x + 2 + app.policy_count_input.len() as u16,
                chunks[2].y + 1,
            ));
        }
    }

    // Error
    if let Some(ref err) = app.input_error {
        let error = Paragraph::new(Span::styled(err, Style::default().fg(RED)));
        frame.render_widget(error, chunks[3]);
    }

    // Quick reference
    let mut ref_lines = vec![
        Line::from(Span::styled(
            "Quick Reference:",
            Style::default().fg(DIM).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  Property value: {}", fmt_money(config.property_value)),
            Style::default().fg(DIM),
        )),
    ];
    let perils_str: Vec<String> = config
        .perils
        .iter()
        .map(|p| format!("{} ({:.1}%)", p.name, p.frequency * 100.0))
        .collect();
    ref_lines.push(Line::from(Span::styled(
        format!("  Perils: {}", perils_str.join(", ")),
        Style::default().fg(DIM),
    )));
    ref_lines.push(Line::from(Span::styled(
        format!("  Expense ratio: {:.0}%", config.expense_ratio * 100.0),
        Style::default().fg(DIM),
    )));
    let reference = Paragraph::new(ref_lines);
    frame.render_widget(reference, chunks[4]);

    // Controls
    let mut ctrl = vec![
        Span::styled(" Enter ", Style::default().fg(BLUE)),
        Span::raw("Submit  "),
        Span::styled(" h ", Style::default().fg(YELLOW)),
        Span::raw("Hints  "),
    ];
    if config.chapter == 2 {
        ctrl.push(Span::styled(" Tab ", Style::default().fg(BLUE)));
        ctrl.push(Span::raw("Switch field  "));
    }
    ctrl.push(Span::styled(" Esc ", Style::default().fg(BLUE)));
    ctrl.push(Span::raw("Back"));
    let controls = Paragraph::new(Line::from(ctrl));
    frame.render_widget(controls, chunks[5]);
}

fn draw_simulating(frame: &mut Frame, app: &App, area: Rect) {
    let config = match &app.scenario {
        Some(c) => c,
        None => return,
    };

    let title = format!(" Chapter {}: {} \u{2014} Simulation ", config.chapter, config.title);
    let outer = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .padding(Padding::new(1, 1, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let visible_count = app.sim_current_year as usize;
    let visible_results = &app.sim_results[..visible_count.min(app.sim_results.len())];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // Table + chart
            Constraint::Length(3), // Surplus
            Constraint::Length(2), // Controls
        ])
        .split(inner);

    // Split main area into table and chart
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(chunks[0]);

    // Year results table
    let header = Row::new(vec![
        Cell::from("Year").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Premiums").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Claims").style(Style::default().add_modifier(Modifier::BOLD)),
        Cell::from("Net").style(Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let rows: Vec<Row> = visible_results
        .iter()
        .map(|r| {
            let net_style = if r.underwriting_income >= 0.0 {
                Style::default().fg(GREEN)
            } else {
                Style::default().fg(RED)
            };
            Row::new(vec![
                Cell::from(format!("{}", r.year)),
                Cell::from(fmt_money(r.premiums_earned)),
                Cell::from(fmt_money(r.claims_incurred))
                    .style(if r.claims_incurred > 0.0 {
                        Style::default().fg(RED)
                    } else {
                        Style::default()
                    }),
                Cell::from(fmt_money(r.underwriting_income)).style(net_style),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(6),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Year Results ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(DIM)),
    );
    frame.render_widget(table, main_chunks[0]);

    // Bar chart - claims per year
    if !visible_results.is_empty() {
        let max_val = visible_results
            .iter()
            .map(|r| r.claims_incurred as u64)
            .max()
            .unwrap_or(1)
            .max(1);

        let premium_line = if !visible_results.is_empty() {
            visible_results[0].premiums_earned as u64
        } else {
            0
        };

        let bars: Vec<Bar> = visible_results
            .iter()
            .map(|r| {
                let color = if r.claims_incurred > r.premiums_earned {
                    RED
                } else if r.claims_incurred > 0.0 {
                    YELLOW
                } else {
                    GREEN
                };
                Bar::default()
                    .value(r.claims_incurred as u64)
                    .label(format!("Y{}", r.year).into())
                    .style(Style::default().fg(color))
            })
            .collect();

        let chart_title = format!(
            " Claims vs Premium ({}) ",
            fmt_money(premium_line as f64)
        );
        let chart = BarChart::default()
            .block(
                Block::default()
                    .title(chart_title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(DIM)),
            )
            .data(BarGroup::default().bars(&bars))
            .bar_width(5)
            .bar_gap(1)
            .max(max_val);
        frame.render_widget(chart, main_chunks[1]);
    }

    // Surplus line
    let current_surplus = if let Some(last) = visible_results.last() {
        last.ending_surplus
    } else {
        app.sim_surplus
    };
    let surplus_style = if current_surplus >= app.sim_surplus {
        Style::default().fg(GREEN)
    } else if current_surplus > 0.0 {
        Style::default().fg(YELLOW)
    } else {
        Style::default().fg(RED)
    };
    let surplus = Paragraph::new(Line::from(vec![
        Span::raw("  Surplus: "),
        Span::styled(fmt_money(current_surplus), surplus_style),
        Span::styled(
            format!(
                "  ({} from start)",
                fmt_money_signed(current_surplus - app.sim_surplus)
            ),
            Style::default().fg(DIM),
        ),
    ]));
    frame.render_widget(surplus, chunks[1]);

    // Controls
    let controls = if app.sim_done {
        Paragraph::new(Line::from(vec![
            Span::styled(" Enter ", Style::default().fg(GREEN)),
            Span::raw("View Results  "),
            Span::styled(" q ", Style::default().fg(BLUE)),
            Span::raw("Quit"),
        ]))
    } else {
        Paragraph::new(Line::from(vec![
            Span::styled(" Space ", Style::default().fg(BLUE)),
            Span::raw("Skip  "),
            Span::styled(" q ", Style::default().fg(BLUE)),
            Span::raw("Quit"),
        ]))
    };
    frame.render_widget(controls, chunks[2]);
}

fn draw_results(frame: &mut Frame, app: &App, area: Rect) {
    let config = match &app.scenario {
        Some(c) => c,
        None => return,
    };

    let title = format!(" Chapter {}: {} \u{2014} Results ", config.chapter, config.title);
    let outer = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(BLUE))
        .padding(Padding::new(1, 1, 1, 0));
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Chart
            Constraint::Min(5),    // Feedback
            Constraint::Length(2), // Controls
        ])
        .split(inner);

    // Full bar chart
    if !app.sim_results.is_empty() {
        let max_val = app
            .sim_results
            .iter()
            .map(|r| r.claims_incurred.max(r.premiums_earned) as u64)
            .max()
            .unwrap_or(1)
            .max(1);

        let bars: Vec<Bar> = app
            .sim_results
            .iter()
            .map(|r| {
                let color = if r.claims_incurred > r.premiums_earned {
                    RED
                } else if r.claims_incurred > 0.0 {
                    YELLOW
                } else {
                    GREEN
                };
                Bar::default()
                    .value(r.claims_incurred as u64)
                    .label(format!("Y{}", r.year).into())
                    .style(Style::default().fg(color))
            })
            .collect();

        let chart = BarChart::default()
            .block(
                Block::default()
                    .title(" Claims by Year ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(DIM)),
            )
            .data(BarGroup::default().bars(&bars))
            .bar_width(5)
            .bar_gap(1)
            .max(max_val);
        frame.render_widget(chart, chunks[0]);
    }

    // Feedback
    if let Some(ref feedback) = app.feedback {
        let lines: Vec<Line> = feedback
            .lines
            .iter()
            .map(|(kind, text)| {
                let color = match kind {
                    FeedbackKind::Good => GREEN,
                    FeedbackKind::Warning => YELLOW,
                    FeedbackKind::Bad => RED,
                    FeedbackKind::Info => Color::White,
                };
                let prefix = match kind {
                    FeedbackKind::Good => "\u{2713} ",
                    FeedbackKind::Warning => "\u{26a0} ",
                    FeedbackKind::Bad => "\u{2717} ",
                    FeedbackKind::Info => "  ",
                };
                if text.is_empty() {
                    Line::from("")
                } else {
                    Line::from(Span::styled(
                        format!("{}{}", prefix, text),
                        Style::default().fg(color),
                    ))
                }
            })
            .collect();

        let passed_text = if feedback.passed {
            Line::from(Span::styled(
                "\n  Chapter complete! Press Enter to continue.",
                Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
            ))
        } else {
            Line::from(Span::styled(
                "\n  Not quite. Press r to retry or Enter to return to menu.",
                Style::default().fg(YELLOW),
            ))
        };

        let mut all_lines = lines;
        all_lines.push(passed_text);

        let feedback_p = Paragraph::new(all_lines)
            .block(
                Block::default()
                    .title(" Analysis ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(DIM)),
            )
            .wrap(Wrap { trim: false })
            .scroll((app.result_scroll, 0));
        frame.render_widget(feedback_p, chunks[1]);
    }

    // Controls
    let controls = Paragraph::new(Line::from(vec![
        Span::styled(" r ", Style::default().fg(YELLOW)),
        Span::raw("Retry  "),
        Span::styled(" Enter ", Style::default().fg(GREEN)),
        Span::raw("Continue  "),
        Span::styled(" \u{2191}\u{2193} ", Style::default().fg(BLUE)),
        Span::raw("Scroll  "),
        Span::styled(" q ", Style::default().fg(BLUE)),
        Span::raw("Quit"),
    ]));
    frame.render_widget(controls, chunks[2]);
}

fn draw_hints_popup(frame: &mut Frame, app: &App, area: Rect) {
    let config = match &app.scenario {
        Some(c) => c,
        None => return,
    };

    let popup_area = centered_rect(70, 60, area);
    frame.render_widget(Clear, popup_area);

    let mut lines: Vec<Line> = Vec::new();

    for (i, hint) in config.hints.iter().take(app.hints_revealed).enumerate() {
        lines.push(Line::from(Span::styled(
            format!("Hint {}:", i + 1),
            Style::default().fg(YELLOW).add_modifier(Modifier::BOLD),
        )));
        for line in hint.split('\n') {
            lines.push(Line::from(format!("  {}", line)));
        }
        lines.push(Line::from(""));
    }

    if app.hints_revealed < config.hints.len() {
        lines.push(Line::from(Span::styled(
            format!(
                "Press h to reveal next hint ({}/{} remaining)",
                config.hints.len() - app.hints_revealed,
                config.hints.len()
            ),
            Style::default().fg(DIM),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "All hints revealed.",
            Style::default().fg(DIM),
        )));
    }

    let popup = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Hints ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(YELLOW))
                .padding(Padding::new(1, 1, 1, 0)),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(popup, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}
