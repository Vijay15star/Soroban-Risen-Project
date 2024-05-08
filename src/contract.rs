use soroban_sdk::{Env, Symbol, Binary, vec, Address, IntoVal};


struct CalculationResult {
    result: i32, 
}

pub struct CalculatorContract;

#[contractimpl]
impl CalculatorContract {
    pub fn add(&self, e: Env, a: i32, b: i32) -> CalculationResult {
        CalculationResult { result: a + b }
    }

    pub fn subtract(&self, e: Env, a: i32, b: i32) -> CalculationResult {
        CalculationResult { result: a - b }
    }

    pub fn multiply(&self, e: Env, a: i32, b: i32) -> CalculationResult {
        CalculationResult { result: a * b }
    }

    pub fn divide(&self, e: Env, a: i32, b: i32) -> Result<CalculationResult, String> {
        if b == 0 {
            return Err("Division by zero".to_string());
        }
        Ok(CalculationResult { result: a / b })
    }
}
