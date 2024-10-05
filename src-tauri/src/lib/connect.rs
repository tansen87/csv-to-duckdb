use std::{error::Error, path::Path, time::Instant};

use duckdb::{Connection, Result, polars::prelude::*};
use indexmap::IndexMap;
use tauri::Emitter;

fn csv2json(file: String, sep: String) -> Result<String, Box<dyn Error>> {
    let sep = if sep == "\\t" {
        b'\t'
    } else {
        sep.clone().into_bytes()[0]
    };

    let df = CsvReader::from_path(file)?
        .with_separator(sep)
        .with_n_rows(Some(100))
        .infer_schema(Some(0))
        .truncate_ragged_lines(true)
        .has_header(true)
        .with_n_threads(Some(4))
        .finish()?;
    let column_names = df.get_column_names();
    let mut height = Vec::new();
    if df.height() <= 100 {
        height.push(df.height())
    } else {
        height.push(5);
    }

    let buffer = (0..height[0])
        .into_iter()
        .map(|i| {
            let row = df
                .get_row(i)
                .expect(&*format!(
                    "Could not access row {}, please try again.",
                    i + 2
                ))
                .0;

            let object = column_names
                .iter()
                .zip(row.iter())
                .map(|(column, data)| (column.to_string(), data.get_str().unwrap_or("").to_owned()))
                .collect::<IndexMap<String, String>>();
            serde_json::to_string(&object).expect("Unable to serialize the result.")
        })
        .collect::<Vec<String>>();
    let result = if height[0] > 1 {
        format!("[{}]", buffer.join(","))
    } else {
        buffer
            .get(0)
            .expect("Unable to get value from buffer.")
            .clone()
    };

    Ok(result)
}

fn csv_to_db(
    table: String,
    file: String,
    sep: String,
    quote: String,
    output_path: String,
) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();

    let conn = Connection::open(output_path)?;
    let idata = format!(
        "
        CREATE TABLE {table}
        AS SELECT *
        FROM read_csv('{file}', all_varchar=true, sep='{sep}', quote='{quote}');"
    );
    conn.execute_batch(&idata)?;

    let end = Instant::now();
    let elapsed = end - start;
    let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());

    Ok(elapsed_str)
}

fn excute_query(db_file: String, sep: String, sql: String) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();

    let file_path = Path::new(&db_file);
    let file_name = Path::new(&db_file).file_name().unwrap().to_str().unwrap();
    let parent_path = file_path
        .parent()
        .map(|parent| parent.to_string_lossy())
        .unwrap();
    let current_time = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let output = format!("{parent_path}/{}_query_{current_time}.csv", file_name);

    let conn = Connection::open(db_file)?;
    let edata = format!("COPY ({sql}) TO '{output}' (DELIMITER '{sep}');");
    conn.execute_batch(&edata)?;

    let end = Instant::now();
    let elapsed = end - start;
    let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());

    Ok(elapsed_str)
}

#[tauri::command]
pub async fn view(file: String, sep: String, window: tauri::Window) -> String {
    let result = match async { csv2json(file, sep) }.await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("csv2json error: {err}");
            window.emit("csv2json_err", &err.to_string()).unwrap();
            err.to_string()
        }
    };

    result
}

#[tauri::command]
pub async fn csv2db(
    table: String,
    file: String,
    sep: String,
    quote: String,
    output_path: String,
) -> String {
    let elapsed_time = match async { csv_to_db(table, file, sep, quote, output_path) }.await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("connect error: {err}");
            err.to_string()
        }
    };

    elapsed_time
}

#[tauri::command]
pub async fn query(file: String, sep: String, sql: String) -> String {
    let elapsed_time = match async { excute_query(file, sep, sql) }.await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("query error: {err}");
            err.to_string()
        }
    };

    elapsed_time
}
