#[derive(Debug)]
pub struct Solver {
    pub year: u16,
    pub day: u8,
    pub level: u8,
    pub name: String,
    pub func: fn(&[&str]) -> String,
}

pub struct SolverInfo {
    pub year: u16,
    pub day: u8,
    pub level: u8,
    pub name: &'static str,
    pub func: fn(&[&str]) -> String,
}

inventory::collect!(SolverInfo);

pub fn discover_solvers(year: u16, day: u8) -> Vec<Solver> {
    inventory::iter::<SolverInfo>
        .into_iter()
        .filter(|info| info.year == year && info.day == day)
        .map(|info| Solver {
            year: info.year,
            day: info.day,
            level: info.level,
            name: info.name.to_string(),
            func: info.func,
        })
        .collect()
}

pub fn list_solvers() -> Result<(), Box<dyn std::error::Error>> {
    let mut solvers: Vec<_> = inventory::iter::<SolverInfo>
        .into_iter()
        .collect();
    
    solvers.sort_by(|a, b| {
        a.year.cmp(&b.year)
            .then(a.day.cmp(&b.day))
            .then(a.level.cmp(&b.level))
            .then(a.name.cmp(&b.name))
    });

    for solver in solvers {
        println!("{}-{:02}, {}, {}", 
            solver.year,
            solver.day,
            solver.level,
            solver.name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_sorting() {
        let solvers = vec![
            SolverInfo { year: 2024, day: 1, level: 2, name: "solve_b", func: |_| "".into() },
            SolverInfo { year: 2024, day: 1, level: 1, name: "solve_a", func: |_| "".into() },
            SolverInfo { year: 2023, day: 25, level: 1, name: "solve", func: |_| "".into() },
        ];

        let mut sorted = solvers;
        sorted.sort_by(|a, b| {
            a.year.cmp(&b.year)
                .then(a.day.cmp(&b.day))
                .then(a.level.cmp(&b.level))
                .then(a.name.cmp(&b.name))
        });

        assert_eq!(sorted[0].year, 2023);
        assert_eq!(sorted[1].year, 2024);
        assert_eq!(sorted[1].level, 1);
        assert_eq!(sorted[2].level, 2);
    }
}