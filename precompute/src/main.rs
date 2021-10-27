use serde::Deserialize;
use statrs::function::gamma::{gamma, gamma_li};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::ops::{Add, Mul};
// use std::time::Instant;

fn sf(x: f64, k: f64) -> f64 {
    // 1 - cdf of chi2 distribution
    if x == 0.0 {
        return 1.0f64;
    }
    1.0 - gamma_li(k / 2.0, x / 2.0) / gamma(k / 2.0)
}

fn sum2<T>(data: &Vec<T>, shape: (usize, usize), axis: usize) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default,
{
    // Sum over one axis of a 2D matrix defined by the vectorized data and shape.
    if axis == 0 {
        let mut output = vec![T::default(); shape.1];
        for idx in 0usize..data.len() {
            output[idx / shape.0] = output[idx / shape.0] + data[idx];
        }
        output
    } else if axis == 1 {
        let mut output = vec![T::default(); shape.0];
        for idx in 0usize..data.len() {
            output[idx % shape.0] = output[idx % shape.0] + data[idx];
        }
        output
    } else {
        panic!("axis must be in [0, 1]");
    }
}

fn sum<T>(data: &Vec<T>) -> T
where
    T: Add<Output = T> + Copy + Default,
{
    // Sum the elements in a vector.
    let mut output = T::default();
    for idx in 0usize..data.len() {
        output = output + data[idx];
    }
    output
}

fn cross<T>(cols: &Vec<T>, rows: &Vec<T>) -> Vec<T>
where
    T: Mul<Output = T> + Copy + Default,
{
    // Compute the cross-product of two vectors. Return the vectorized result.
    let mut output = vec![T::default(); cols.len() * rows.len()];
    for idx_row in 0usize..rows.len() {
        for idx_col in 0usize..cols.len() {
            output[idx_row + rows.len() * idx_col] =
                rows[idx_row] * cols[idx_col];
        }
    }
    output
}

#[derive(Debug, Deserialize)]
struct Data {
    categories: Option<Vec<String>>,
    #[serde(rename(serialize = "enum", deserialize = "enum"))]
    _enum: Option<Vec<String>>,
    maximum: Option<usize>,
    minimum: Option<usize>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    _type: String,
}

fn parse_yaml(
    features_path: &str,
    col_idx: HashMap<&str, usize>,
) -> Vec<Option<usize>> {
    let s = fs::read_to_string(features_path).unwrap();
    let features: HashMap<String, HashMap<String, Data>> =
        serde_yaml::from_str(&s).unwrap();

    let mut num_options = vec![None; col_idx.len()];
    for (key, value) in &features["patient"] {
        let idx = match col_idx.get(&key.to_string() as &str) {
            Some(x) => *x,
            None => {
                continue; // ignore options for which there is no CSV column
            }
        };
        num_options[idx] = match &value._enum {
            Some(x) => Some(x.len()),
            None => match value._type.as_str() {
                "integer" => {
                    if !value.minimum.is_none() && !value.maximum.is_none() {
                        Some(
                            value.maximum.unwrap() - value.minimum.unwrap()
                                + 1usize,
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
    num_options
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut features_path = "features.yml".to_string();
    if &args[1] == "--features" {
        features_path = args.remove(2);
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let headers = rdr.headers().unwrap().clone();
    let num_cols = headers.len();

    // get column indices from CSV header
    let mut col_idx = HashMap::new();
    for (i, col) in headers.iter().enumerate() {
        col_idx.insert(col, i);
    }
    let num_options = parse_yaml(&features_path, col_idx);

    // add header row to output
    let mut column_names = Vec::new();
    column_names.push("".to_string());
    for col in headers.iter() {
        column_names.push(col.to_string());
    }

    // set up column storage things
    let mut columns = HashMap::new();
    for idx_col in 0usize..num_cols {
        let array = Vec::new();
        columns.insert(idx_col, array);
    }
    let mut col_val_maps = HashMap::new();
    for idx_col in 0usize..num_cols {
        col_val_maps.insert(idx_col, HashMap::new());
    }
    let mut col_num_vals: HashMap<usize, usize> = HashMap::new();
    for idx_col in 0usize..num_cols {
        col_num_vals.insert(idx_col, 0usize);
    }

    // mash data into columns
    for result in rdr.records() {
        let record = result.unwrap();
        for idx_col in 0usize..num_cols {
            let key = record.get(idx_col).unwrap().to_string();
            let num_vals = col_num_vals.get_mut(&idx_col).unwrap();
            let value_map = col_val_maps.get_mut(&idx_col).unwrap();
            if !value_map.contains_key(&key) {
                value_map.insert(key, *num_vals);
                *num_vals += 1;
                if !num_options[idx_col].is_none()
                    && num_vals > &mut num_options[idx_col].unwrap()
                {
                    panic!("There are more unique values for column {:?} than implied by its specification", column_names[idx_col + 1])
                }
            }
        }
        for idx_col in 0usize..num_cols {
            let key = record.get(idx_col).unwrap().to_string();
            let value_map = col_val_maps.get_mut(&idx_col).unwrap();
            let value = value_map.get(&key).unwrap();
            columns.get_mut(&idx_col).unwrap().push(*value);
        }
    }
    wtr.serialize(column_names).unwrap();

    // compute chi2 statistic and p-value for each pair of columns
    // let start = Instant::now();
    for idx_col in 0usize..num_cols {
        let first = columns.get(&idx_col).unwrap();

        // let it_start = Instant::now();
        let mut row = vec!["".to_string(); num_cols + 1];
        row[0] = headers.get(idx_col).unwrap().to_string();
        for jdx_col in (idx_col + 1)..num_cols {
            let second = columns.get(&jdx_col).unwrap();

            // columns for which there are not options get -1 everywhere
            let n = match num_options[idx_col] {
                Some(_n) => _n,
                None => {
                    row[jdx_col + 1] = (-1f64).to_string();
                    continue;
                }
            };
            let m = match num_options[jdx_col] {
                Some(_n) => _n,
                None => {
                    row[jdx_col + 1] = (-1f64).to_string();
                    continue;
                }
            };
            // construct observed count matrix
            let num_el = n * m;
            let mut observed = vec![f32::EPSILON; num_el];
            for idx_row in 0usize..first.len() {
                let key = first[idx_row] + n * second[idx_row];
                observed[key] += 1f32;
            }

            // construct expected count matrix
            let cols = sum2(&observed, (n, m), 0);
            let rows = sum2(&observed, (n, m), 1);
            let total = sum(&rows);
            let mut expected = cross(&cols, &rows);
            for idx in 0usize..expected.len() {
                expected[idx] /= total;
            }

            // compute chi2 statistic
            let mut terms = vec![0f32; num_el];
            for idx in 0usize..num_el {
                let diff = observed[idx] - expected[idx];
                terms[idx] = diff * diff / expected[idx];
            }
            let stat = sum(&terms);
            let dof = num_el - (n + m) + 2 - 1;

            // compute p-value
            let p_val = sf(stat as f64, dof as f64);
            row[jdx_col + 1] = p_val.to_string();
        }
        // let it_duration = it_start.elapsed();
        // println!("Time elapsed in expensive_function() is: {:?}", it_duration);

        wtr.serialize(row).unwrap();
    }
    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    wtr.flush().unwrap();
}
