pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // Check for any side of length zero or invalid triangle inequality.
        if sides.iter().any(|&s| s == 0) || 
           sides.iter().sum::<u64>() - sides.iter().max().unwrap() <= *sides.iter().max().unwrap() {
            None
        } else {
            Some(Triangle { sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[2] != self.sides[0]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2] || self.sides[2] == self.sides[0]
    }
}
