pub mod chapter1;
pub mod chapter2;

use crate::engine::{Peril, Policy, YearResult};

pub struct ScenarioConfig {
    pub chapter: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub brief: Vec<&'static str>,
    pub perils: Vec<Peril>,
    pub property_value: f64,
    pub expense_ratio: f64,
    pub num_years: u32,
    pub hints: Vec<&'static str>,
    pub max_market_premium: f64,
}

pub struct Feedback {
    pub lines: Vec<(FeedbackKind, String)>,
    pub passed: bool,
}

#[derive(Clone, Copy)]
pub enum FeedbackKind {
    Good,
    Warning,
    Bad,
    Info,
}

pub fn get_scenario(chapter: usize) -> Option<ScenarioConfig> {
    match chapter {
        1 => Some(chapter1::config()),
        2 => Some(chapter2::config()),
        _ => None,
    }
}

pub fn create_policies(config: &ScenarioConfig, premium: f64, count: u32) -> Vec<Policy> {
    (0..count)
        .map(|id| Policy {
            id,
            property_value: config.property_value,
            premium,
            perils: config.perils.clone(),
        })
        .collect()
}

pub fn get_feedback(
    chapter: usize,
    results: &[YearResult],
    premium: f64,
    config: &ScenarioConfig,
) -> Feedback {
    match chapter {
        1 => chapter1::feedback(results, premium, config),
        2 => chapter2::feedback(results, premium, config),
        _ => Feedback {
            lines: vec![],
            passed: false,
        },
    }
}

pub struct ChapterInfo {
    pub number: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub fn chapter_list() -> Vec<ChapterInfo> {
    vec![
        ChapterInfo {
            number: 1,
            title: "Your First Policy",
            subtitle: "Expected Value",
        },
        ChapterInfo {
            number: 2,
            title: "Growing the Book",
            subtitle: "Law of Large Numbers",
        },
        ChapterInfo {
            number: 3,
            title: "Fitting the Curve",
            subtitle: "Loss Distributions",
        },
        ChapterInfo {
            number: 4,
            title: "Building the Rate Manual",
            subtitle: "Ratemaking",
        },
        ChapterInfo {
            number: 5,
            title: "The Long Tail",
            subtitle: "Reserving & IBNR",
        },
        ChapterInfo {
            number: 6,
            title: "Sharing the Risk",
            subtitle: "Reinsurance",
        },
        ChapterInfo {
            number: 7,
            title: "Storm Season",
            subtitle: "Catastrophe Modeling",
        },
        ChapterInfo {
            number: 8,
            title: "The Bottom Line",
            subtitle: "Combined Ratio",
        },
        ChapterInfo {
            number: 9,
            title: "Capital Allocation",
            subtitle: "Surplus Management",
        },
        ChapterInfo {
            number: 10,
            title: "The Full Picture",
            subtitle: "Stochastic Modeling",
        },
    ]
}
