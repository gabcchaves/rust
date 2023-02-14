use std::convert::{TryInto};
use std::ops::{Add, Div};

fn comp_median<T>(v: Vec<T>) -> Result<f64, String>
where
    T: TryInto<f64> + Copy,
    f64: Div<Output = f64> + Add<Output = f64>,
{
    let median: f64;

    if v.len() % 2 == 0 {
        let mi = v.len() / 2 as usize;
        let a = v[mi-1].try_into().map_err(|_| "Error converting to f64.")?;
        let b = v[mi].try_into().map_err(|_| "Error converting to f64.")?;
        if ((a + b) / 2.0) == ((a + b) / 2.0).trunc() {
            median = ((a + b) / 2.0).trunc();
        } else {
            median = (a + b) / 2.0;
        }
    } else {
        median = v[v.len() / 2 as usize].try_into().map_err(|_| "Error converting to f64.")?;
    }

    Ok(median)
}

fn main() {
    assert_eq!(comp_median(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap(), 4.5);
}
