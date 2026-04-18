use crate::engine::{self, Peril, YearResult};

use super::{Feedback, FeedbackKind, ScenarioConfig};

pub fn config() -> ScenarioConfig {
    ScenarioConfig {
        chapter: 2,
        title: "Growing the Book",
        subtitle: "Law of Large Numbers",
        brief: vec![
            "Pacific Shield survived its first year. Now it's time to grow.",
            "",
            "Your underwriting team has found a neighborhood of similar homes,",
            "all needing coverage. Same perils, same construction, same values.",
            "",
            "Your task: price the book and decide how many policies to write.",
            "",
            "More policies means more premium income — but also more exposure.",
            "The question is: does scale make you safer or more vulnerable?",
            "",
            "(Hint: this is the most important lesson in insurance.)",
        ],
        perils: vec![
            Peril {
                name: "Fire".into(),
                frequency: 0.02,
                severity_pct: 0.40,
            },
            Peril {
                name: "Water".into(),
                frequency: 0.05,
                severity_pct: 0.10,
            },
            Peril {
                name: "Theft".into(),
                frequency: 0.005,
                severity_pct: 0.05,
            },
        ],
        property_value: 250_000.0,
        expense_ratio: 0.30,
        num_years: 5,
        hints: vec![
            "Think about what happens to the average outcome when you\nflip a coin 10 times vs. 10,000 times.",
            "The Law of Large Numbers says the average of many independent\nevents converges to the expected value. More policies = more\npredictable results.",
            "With 1 policy, your loss ratio swings between 0% and 2000%+.\nWith 500 policies, it clusters tightly around the expected\nloss ratio.",
            "The ideal premium is the same as Chapter 1. The magic of\nLLN is that with enough policies, you can RELY on that\nexpected value.",
        ],
        max_market_premium: 8_000.0,
    }
}

pub fn feedback(results: &[YearResult], premium: f64, config: &ScenarioConfig) -> Feedback {
    let pure_premium = engine::expected_pure_premium(&config.perils, config.property_value);
    let gross_premium =
        engine::expected_gross_premium(&config.perils, config.property_value, config.expense_ratio);

    let mut lines = Vec::new();

    let avg_loss_ratio = if results.is_empty() {
        0.0
    } else {
        results.iter().map(|r| r.loss_ratio).sum::<f64>() / results.len() as f64
    };
    let loss_ratio_variance = if results.len() > 1 {
        let mean = avg_loss_ratio;
        results
            .iter()
            .map(|r| (r.loss_ratio - mean).powi(2))
            .sum::<f64>()
            / (results.len() - 1) as f64
    } else {
        0.0
    };
    let loss_ratio_std = loss_ratio_variance.sqrt();

    let num_policies = results.first().map(|r| r.num_policies).unwrap_or(0);
    let expected_lr = pure_premium / premium;

    lines.push((
        FeedbackKind::Info,
        format!(
            "Book size: {} policies. Simulated {} years.",
            num_policies,
            results.len()
        ),
    ));
    lines.push((
        FeedbackKind::Info,
        format!(
            "Average loss ratio: {:.1}% (expected: {:.1}%). Std dev: {:.1}%.",
            avg_loss_ratio * 100.0,
            expected_lr * 100.0,
            loss_ratio_std * 100.0
        ),
    ));

    let went_insolvent = results.iter().any(|r| r.ending_surplus < 0.0);
    let final_surplus = results.last().map(|r| r.ending_surplus).unwrap_or(0.0);

    let mut passed = true;

    if premium < gross_premium {
        lines.push((
            FeedbackKind::Bad,
            format!(
                "Premium ${:.0} is below break-even (${:.0}). Volume amplifies a bad price — you lose more with every policy you write.",
                premium, gross_premium
            ),
        ));
        passed = false;
    }

    if went_insolvent {
        lines.push((
            FeedbackKind::Bad,
            "The company went insolvent.".into(),
        ));
        passed = false;
    } else {
        lines.push((
            FeedbackKind::Good,
            format!("Final surplus: ${:.0}.", final_surplus),
        ));
    }

    lines.push((FeedbackKind::Info, String::new()));
    if num_policies >= 100 {
        lines.push((
            FeedbackKind::Good,
            "With a large book, the Law of Large Numbers works in your favor. Actual results track close to expected values.".into(),
        ));
        if loss_ratio_std < 0.10 {
            lines.push((
                FeedbackKind::Good,
                format!(
                    "Your loss ratio std dev of {:.1}% shows how predictable a large book is. This is the foundation of insurance.",
                    loss_ratio_std * 100.0
                ),
            ));
        }
    } else {
        lines.push((
            FeedbackKind::Warning,
            format!(
                "With only {} policies, results are volatile. Try running this chapter again with more policies to see the Law of Large Numbers in action.",
                num_policies
            ),
        ));
    }

    lines.push((FeedbackKind::Info, String::new()));
    lines.push((
        FeedbackKind::Info,
        "Key takeaway: Insurance works because of the Law of Large Numbers. A single policy is a gamble. A thousand policies is a business.".into(),
    ));

    Feedback { lines, passed }
}
