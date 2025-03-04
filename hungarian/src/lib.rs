use core::panic;

use libc::c_int;

#[repr(C)]
struct HungarianProblem {
    num_rows: c_int,
    num_cols: c_int,
    cost: *mut *mut c_int,
    assignment: *mut *mut c_int,
}

unsafe extern "C" {
    fn hungarian_init(
        p: *mut HungarianProblem,
        cost_matrix: *mut *mut c_int,
        rows: c_int,
        cols: c_int,
        mode: c_int,
    ) -> c_int;

    fn hungarian_free(p: *mut HungarianProblem);

    fn hungarian_solve(p: *mut HungarianProblem) -> c_int;
}

pub enum HungarianMode {
    MaximizeUtil,
    MinimizeCost,
}

pub struct Hungarian {
    problem: Box<HungarianProblem>,
}

#[derive(Debug)]
pub struct HungarianResult {
    pub assigment: Vec<Vec<i32>>,
    pub cost: i32,
}

impl Hungarian {
    pub fn new(cost_matrix: &[Vec<i32>], mode: HungarianMode) -> Self {
        let rows = cost_matrix.len() as c_int;
        if rows <= 0 {
            panic!("Matrix is empty");
        }

        let cols = cost_matrix[0].len() as c_int;
        for row in cost_matrix.iter() {
            if row.len() as c_int != cols {
                panic!("Matrix does't have the same number of collumns for each row");
            }
        }
        if cols <= 0 {
            panic!("Matrix is empty");
        }

        let mut cost_matrix_ptrs: Vec<*mut c_int> = cost_matrix
            .iter()
            .map(|row| row.as_ptr() as *mut c_int)
            .collect();

        let mut problem = Box::new(HungarianProblem {
            num_rows: 0,
            num_cols: 0,
            cost: std::ptr::null_mut(),
            assignment: std::ptr::null_mut(),
        });

        let mode: c_int = match mode {
            HungarianMode::MinimizeCost => 0,
            HungarianMode::MaximizeUtil => 1,
        };

        let _ = unsafe {
            hungarian_init(
                &mut *problem,
                cost_matrix_ptrs.as_mut_ptr(),
                rows,
                cols,
                mode,
            )
        };

        Hungarian { problem }
    }

    pub fn solve(&mut self) -> HungarianResult {
        let cost = unsafe { hungarian_solve(&mut *self.problem) };

        let rows = self.problem.num_rows as usize;
        let cols = self.problem.num_cols as usize;
        let mut assigment: Vec<Vec<i32>> = Vec::with_capacity(rows);

        let assignment_ptr = self.problem.assignment;

        for row in 0..rows {
            let row_ptr = unsafe { *assignment_ptr.add(row) };

            let row_vec: Vec<i32> = unsafe { Vec::from_raw_parts(row_ptr, cols, cols) };

            assigment.push(row_vec);
        }

        // Free assignment pointer
        unsafe { Vec::from_raw_parts(self.problem.assignment, rows, rows) };
        // And set it to null
        self.problem.assignment = std::ptr::null_mut();

        HungarianResult { assigment, cost }
    }
}

impl Drop for Hungarian {
    fn drop(&mut self) {
        unsafe { hungarian_free(&mut *self.problem) }
    }
}
