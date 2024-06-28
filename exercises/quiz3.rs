// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

pub trait ReportCard {
    fn print(&self) -> String;
}

pub struct NumericalReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

pub struct AlphabeticalReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard for NumericalReportCard {
    fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

impl ReportCard for AlphabeticalReportCard {
    fn print(&self) -> String {
        let grade = match &self.grade {
            1.0 => "F-",
            1.1..=1.5 => "F",
            1.6..=2.0 => "F+",
            2.1..=2.5 => "D-",
            2.6..=3.0 => "D",
            3.1..=3.5 => "C",
            3.6..=4.0 => "B",
            4.1..=4.5 => "A-",
            4.6..=5.0 => "A",
            5.1..=5.5 => "A+",
            _ => "Invalid Grade"
        };

        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = NumericalReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = AlphabeticalReportCard {
            grade: 5.5,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
