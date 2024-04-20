// i32のリストとその平均を保持する構造体
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

// フィールドは外部からはAPI経由でしか操作できない様に実装
impl AverageCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
}

// GUIに描画を行うtrait
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    // Note: ここではトレイト境界の代わりに、トレイトオブジェクトを使用する様に実装しているため、
    //       Boxの中のDrawがそれぞれ別の型でもOK
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
