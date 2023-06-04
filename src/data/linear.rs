#[derive(Debug, Clone)]
pub enum Number {
    F64(f64),
    I64(i64),    
}

#[derive(Debug, Clone)]
pub struct SLinear {
    series: Vec<Number>
}

impl SLinear {
    pub fn new(series: Vec<Number>) -> Self {
        Self { series }
    }

    pub fn get(&self) -> Vec<Number> {
        self.series.clone()
    }

    pub fn type_number(&self) -> String {
        let mut string_type = "I64".to_string();
        for n in self.get() {            
            if let Number::F64(_i) = n {
                string_type = "F64".to_string();
                break;
            }
        };        
        string_type
    }
}


