use std::fmt;

#[derive(Debug)]
pub struct UtilityError {
    problem: String,
    sub_problems: Vec<UtilityError>,
    row_number: Option<usize>
}

impl UtilityError {
    pub fn new(problem: String, row_number: Option<usize>) -> UtilityError{
        UtilityError { problem: problem, sub_problems: Vec::new(), row_number: row_number }
    }
    
    pub fn with_sub_problems(problem: String, sub_problems: Vec<UtilityError>, row_number: Option<usize>) -> UtilityError{
        UtilityError { problem: problem, sub_problems: sub_problems, row_number: row_number }
    }
    
    fn write_with_depth(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        let result = write!(f, "{}", "  ".repeat(depth));
        
        if result.is_err() {
            return Err(result.err().unwrap())
        }
        
        let result = match &(self).row_number {
            Some(rn) => write!(f, "Error (row {}): {}", rn, &(self).problem),
            None => write!(f, "Error: {}", &(self).problem)
        };
        
        if result.is_err() {
            return Err(result.err().unwrap())
        }
        
        for prob in &(self).sub_problems {
            let result = write!(f, "\n");
            
            if result.is_err() {
                return Err(result.err().unwrap())
            }
            
            let result = prob.write_with_depth(f, depth + 1);

            if result.is_err() {
                return Err(result.err().unwrap())
            }
        }

        Ok(())
    }
}

impl fmt::Display for UtilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_with_depth(f, 0)
    }
}

pub fn collect_results(results: Vec<Result<(), UtilityError>>) -> Result<(), UtilityError>{
    let mut errors: Vec<UtilityError> = Vec::new();
    for result in results {
        if result.is_err() {
            errors.push(result.err().unwrap());
        }
    }
    
    match errors.len() == 0 {
        true => Ok(()),
        false => Err(UtilityError::with_sub_problems(String::from("One or more errors detected"), errors, None))
    }
}