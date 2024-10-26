use rand::Rng;

// Utility
pub struct Utility;

// Intervals 
#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[derive(Clone)]
pub struct IntervalU32 {
    pub min: u32,
    pub max: u32,
}

impl Interval{
    pub fn new(min: f64, max: f64) -> Self {
        Interval {min, max}
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn adj_min(&self, x: f64) -> f64 {
        self.min + x
    }

    pub fn adj_max(&self, x: f64) -> f64 {
        self.max + x
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x:f64) -> f64 {
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        return x;
    }
}

impl IntervalU32{
    pub fn new(min: u32, max: u32) -> Self {
        IntervalU32 {min, max}
    }

    pub fn size(&self) -> u32 {
        self.max - self.min
    }

    pub fn adj_min(&self, x: u32) -> u32 {
        self.min + x
    }

    pub fn adj_max(&self, x: u32) -> u32 {
        self.max + x
    }

    pub fn contains(&self, x: u32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: u32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x:u32) -> u32 {
        if x < self.min {return self.min};
        if x > self.max {return self.max};
        return x;
    }
}    

impl Utility {
    pub fn random_float() -> f64 {
        rand::thread_rng().gen()
    }

    pub fn random_float_range(interval: Interval) -> f64 {
        if interval.min.is_infinite() || interval.max.is_infinite() {
            panic!("Cannot generate a random value for infinite intervals");
        }
        if interval.min >= interval.max {
            panic!("Invalid interval: min must be less than max");
        }
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen_range(interval.min..interval.max);
        y
    }

    pub fn random_int_range(interval_u32: IntervalU32) -> u32 {
        if interval_u32.min >= interval_u32.max {
            panic!("Invalid interval: min must be less than max");
        };
        let mut rng = rand::thread_rng();
        let y: u32 = rng.gen_range(interval_u32.min..interval_u32.max);
        y
    }
}