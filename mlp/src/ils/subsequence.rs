use instance_reader::Instance;

#[derive(Default, Clone, Debug)]
pub struct Subsequence {
    t: u32,
    pub c: u32,
    w: u32,
    first: usize,
    last: usize,
}

impl Subsequence {
    fn concatenate(mut self, other: &Subsequence, instance: &Instance) -> Subsequence {
        let distance = instance.distance(self.last, other.first);
        self.c += other.w * (self.t + distance) + other.c;
        self.t += distance + other.t;
        self.w += other.w;
        self.last = other.last;

        self
    }

    fn create_single_node(i: usize, sequence: &[usize]) -> Self {
        Self {
            t: 0,
            c: 0,
            w: if i == 0 { 0 } else { 1 },
            first: sequence[i],
            last: sequence[i],
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubsequenceMatrix<'a> {
    matrix: Vec<Subsequence>,
    dimension: usize,
    instance: &'a Instance,
}

pub struct SubsequenceConcatenator<'a> {
    subsequence: Subsequence,
    matrix: &'a SubsequenceMatrix<'a>,
}

impl<'a> SubsequenceConcatenator<'a> {
    pub fn concatenate(mut self, i: usize, j: usize) -> SubsequenceConcatenator<'a> {
        self.subsequence = self
            .subsequence
            .concatenate(self.matrix.get(i, j), &self.matrix.instance);

        self
    }

    pub fn into_subsequence(self) -> Subsequence {
        self.subsequence
    }
}

impl<'a> SubsequenceMatrix<'a> {
    pub fn from(instance: &'a Instance) -> Self {
        let dimension = instance.dimension + 1;
        SubsequenceMatrix {
            matrix: vec![Default::default(); dimension * dimension],
            dimension,
            instance,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &Subsequence {
        &self.matrix[(i * self.dimension) + j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut Subsequence {
        &mut self.matrix[(i * self.dimension) + j]
    }

    pub fn concatenator_for(&'a self, i: usize, j: usize) -> SubsequenceConcatenator<'a> {
        SubsequenceConcatenator {
            subsequence: self.get(i, j).clone(),
            matrix: self,
        }
    }

    pub fn update(&mut self, sequence: &[usize], bounds: Option<(usize, usize)>) {
        let (begin, end) = bounds.unwrap_or((0, sequence.len() - 1));

        // Single node subsequences
        for i in begin..=end {
            *self.get_mut(i, i) = Subsequence::create_single_node(i, sequence);
        }

        // Direct subsequences
        for i in 0..=end {
            let first_col = std::cmp::max(begin, i + 1);
            for j in first_col..sequence.len() {
                *self.get_mut(i, j) = self
                    .concatenator_for(i, j - 1)
                    .concatenate(j, j)
                    .into_subsequence();
            }
        }

        // Reverse subsequences
        for i in (begin..sequence.len()).rev() {
            let last_col = std::cmp::min(end as isize, i as isize - 1);
            for j in (0..=last_col).rev() {
                let j = j as usize;
                *self.get_mut(i, j) = self
                    .concatenator_for(i, j + 1)
                    .concatenate(j, j)
                    .into_subsequence();
            }
        }
    }

    pub fn extract_solution_cost(&self) -> u32 {
        self.get(0, self.dimension - 1).c
    }
}
