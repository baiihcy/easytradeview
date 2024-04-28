use std::fmt::Display;

#[derive(Debug)]
pub struct Table {
    lens: Vec<usize>,
    table: Vec<Vec<String>>,
}

impl Table {
    pub fn new<T: ToString>(headers: &[T]) -> Table {
        let table: Vec<Vec<String>> = vec![headers.iter().map(|x| x.to_string()).collect()];
        let lens = table[0].iter().map(|x| x.len()).collect();
        Table { lens, table }
    }

    pub fn add_row<T: ToString>(&mut self, row: &[T]) {
        let row: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        for (i, x) in row.iter().enumerate() {
            if i >= self.lens.len() {
                self.lens.push(x.len());
            } else {
                self.lens[i] = self.lens[i].max(x.len());
            }
        }
        self.table.push(row);
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.table.iter() {
            for (j, len) in self.lens.iter().enumerate() {
                if j > 0 {
                    s += " | ";
                }
                s += &format!("{:<1$}", row[j], len);
            }
            s += "\n";
        }
        write!(f, "{}", s)
    }
}
