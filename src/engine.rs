use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peril {
    pub name: String,
    pub frequency: f64,
    pub severity_pct: f64,
}

#[derive(Clone, Debug)]
pub struct Policy {
    pub id: u32,
    pub property_value: f64,
    pub premium: f64,
    pub perils: Vec<Peril>,
}

#[derive(Clone, Debug)]
pub struct Claim {
    pub policy_id: u32,
    pub peril: String,
    pub amount: f64,
}

#[derive(Clone, Debug, Default)]
pub struct YearResult {
    pub year: u32,
    pub num_policies: u32,
    pub premiums_earned: f64,
    pub claims_incurred: f64,
    pub num_claims: u32,
    pub expenses: f64,
    pub underwriting_income: f64,
    pub ending_surplus: f64,
    pub loss_ratio: f64,
    pub combined_ratio: f64,
    pub claims: Vec<Claim>,
}

pub fn simulate_year(
    year: u32,
    policies: &[Policy],
    expense_ratio: f64,
    starting_surplus: f64,
    rng: &mut impl Rng,
) -> YearResult {
    let mut claims = Vec::new();
    let premiums_earned: f64 = policies.iter().map(|p| p.premium).sum();
    let expenses = premiums_earned * expense_ratio;

    for policy in policies {
        for peril in &policy.perils {
            if rng.gen::<f64>() < peril.frequency {
                let amount = policy.property_value * peril.severity_pct;
                claims.push(Claim {
                    policy_id: policy.id,
                    peril: peril.name.clone(),
                    amount,
                });
            }
        }
    }

    let claims_incurred: f64 = claims.iter().map(|c| c.amount).sum();
    let underwriting_income = premiums_earned - claims_incurred - expenses;
    let ending_surplus = starting_surplus + underwriting_income;
    let loss_ratio = if premiums_earned > 0.0 {
        claims_incurred / premiums_earned
    } else {
        0.0
    };
    let combined_ratio = if premiums_earned > 0.0 {
        (claims_incurred + expenses) / premiums_earned
    } else {
        0.0
    };

    YearResult {
        year,
        num_policies: policies.len() as u32,
        premiums_earned,
        claims_incurred,
        num_claims: claims.len() as u32,
        expenses,
        underwriting_income,
        ending_surplus,
        loss_ratio,
        combined_ratio,
        claims,
    }
}

pub fn expected_pure_premium(perils: &[Peril], property_value: f64) -> f64 {
    perils
        .iter()
        .map(|p| p.frequency * p.severity_pct * property_value)
        .sum()
}

pub fn expected_gross_premium(perils: &[Peril], property_value: f64, expense_ratio: f64) -> f64 {
    expected_pure_premium(perils, property_value) / (1.0 - expense_ratio)
}
