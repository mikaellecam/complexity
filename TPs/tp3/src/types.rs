// Ce fichier définit les structures de données utilisées dans le jeu

/// Représente les quatre opérations arithmétiques de base
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Représente une étape dans le calcul
#[derive(Debug, Clone)]
pub struct CalculationStep {
    pub left: i32,
    pub right: i32,
    pub operation: Operation,
    pub result: i32,
}

/// Représente un chemin de solution complet
#[derive(Debug, Clone)]
pub struct Solution {
    pub steps: Vec<CalculationStep>,
    pub target: i32,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "×"),
            Operation::Divide => write!(f, "÷"),
        }
    }
}

impl std::fmt::Display for CalculationStep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {} = {}", self.left, self.operation, self.right, self.result)
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Solution pour atteindre {}:", self.target)?;
        for (i, step) in self.steps.iter().enumerate() {
            writeln!(f, "Étape {}: {}", i + 1, step)?;
        }
        Ok(())
    }
}