use crate::engine::{self, Peril, YearResult};

use super::{Feedback, FeedbackKind, ScenarioConfig};

pub fn config() -> ScenarioConfig {
    ScenarioConfig {
        chapter: 1,
        title: "Your First Policy",
        subtitle: "Expected Value",
        brief: vec![
            "Welcome to Pacific Shield Insurance.",
            "",
            "You've been hired as the company's first actuary. Your CEO has",
            "landed the company's very first prospective policyholder — a",
            "homeowner who needs property coverage.",
            "",
            "Your job: set the annual premium.",
            "",
            "Price it right, and the company profits. Price it too low, and",
            "claims eat you alive. Price it too high, and the homeowner walks.",
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
        num_years: 10,
        hints: vec![
            "Think about the expected cost of each peril separately.\nWhat's the average loss from fire in any given year?",
            "Expected loss per peril = probability x severity x property value.\nCalculate this for each peril and add them up.",
            "Expected losses:\n  Fire  = 0.02 x 0.40 x $250,000 = $2,000\n  Water = 0.05 x 0.10 x $250,000 = $1,250\n  Theft = 0.005 x 0.05 x $250,000 = $62.50\n  Total pure premium = $3,312.50",
            "The pure premium ($3,312.50) only covers expected claims.\n30% of your premium goes to expenses, so:\n  gross premium = pure_premium / (1 - 0.30)\n                = $3,312.50 / 0.70\n                = $4,732.14\nAdd a profit margin on top of that.",
        ],
        max_market_premium: 8_000.0,
    }
}

pub fn feedback(results: &[YearResult], premium: f64, config: &ScenarioConfig) -> Feedback {
    let pure_premium = engine::expected_pure_premium(&config.perils, config.property_value);
    let gross_premium =
        engine::expected_gross_premium(&config.perils, config.property_value, config.expense_ratio);

    let total_premiums: f64 = results.iter().map(|r| r.premiums_earned).sum();
    let total_claims: f64 = results.iter().map(|r| r.claims_incurred).sum();
    let total_expenses: f64 = results.iter().map(|r| r.expenses).sum();
    let total_income = total_premiums - total_claims - total_expenses;
    let avg_loss_ratio: f64 = if results.is_empty() {
        0.0
    } else {
        results.iter().map(|r| r.loss_ratio).sum::<f64>() / results.len() as f64
    };

    let final_surplus = results.last().map(|r| r.ending_surplus).unwrap_or(0.0);
    let went_insolvent = results.iter().any(|r| r.ending_surplus < 0.0);

    let mut lines = Vec::new();

    lines.push((
        FeedbackKind::Info,
        format!(
            "Over {} years: earned ${:.0} in premiums, paid ${:.0} in claims, ${:.0} in expenses.",
            results.len(),
            total_premiums,
            total_claims,
            total_expenses
        ),
    ));

    lines.push((
        FeedbackKind::Info,
        format!(
            "Net underwriting income: ${:.0}. Average loss ratio: {:.1}%.",
            total_income,
            avg_loss_ratio * 100.0
        ),
    ));

    let mut passed = true;

    if premium < pure_premium {
        lines.push((
            FeedbackKind::Bad,
            format!(
                "Your premium (${:.0}) was below the pure premium (${:.0}). You didn't even cover expected losses, let alone expenses.",
                premium, pure_premium
            ),
        ));
        passed = false;
    } else if premium < gross_premium {
        lines.push((
            FeedbackKind::Warning,
            format!(
                "Your premium (${:.0}) covered expected losses but not expenses. The break-even gross premium is ${:.0}.",
                premium, gross_premium
            ),
        ));
        passed = false;
    } else if premium > config.max_market_premium {
        lines.push((
            FeedbackKind::Warning,
            format!(
                "Your premium (${:.0}) is above the market rate of ${:.0}. In a competitive market, this homeowner shops elsewhere.",
                premium, config.max_market_premium
            ),
        ));
        passed = premium < config.max_market_premium * 1.5;
    } else {
        let margin = (premium - gross_premium) / premium * 100.0;
        lines.push((
            FeedbackKind::Good,
            format!(
                "Your premium of ${:.0} covers expected losses (${:.0}), expenses, and includes a {:.1}% profit margin. Well priced.",
                premium, pure_premium, margin
            ),
        ));
    }

    if went_insolvent {
        lines.push((
            FeedbackKind::Bad,
            "Your company went insolvent during the simulation. With only one policy, variance is extremely high — a lesson for Chapter 2.".into(),
        ));
    } else {
        lines.push((
            FeedbackKind::Info,
            format!("Final surplus: ${:.0}.", final_surplus),
        ));
    }

    lines.push((FeedbackKind::Info, String::new()));
    lines.push((
        FeedbackKind::Info,
        "Key takeaway: The pure premium is the floor — the minimum to cover expected claims. The gross premium adds expense loading. Any margin above that is profit.".into(),
    ));

    if results
        .iter()
        .any(|r| r.claims_incurred > r.premiums_earned * 2.0)
    {
        lines.push((
            FeedbackKind::Info,
            "Notice how some years had massive claims while others had none? With just one policy, outcomes are binary. Chapter 2 shows how volume tames this volatility.".into(),
        ));
    }

    Feedback { lines, passed }
}
