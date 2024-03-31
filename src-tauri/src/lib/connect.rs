use std::{
    error::Error,
    path::Path,
    time::Instant,
};

use duckdb::{
    Connection,
    Result
};

fn conn_db(table: String, file: String, sep: String) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();

    let file_path = Path::new(&file);
    let file_name = match file_path.file_name() {
        Some(name) => match name.to_str() {
            Some(name_str) => name_str.split('.').collect::<Vec<&str>>(),
            None => vec![],
        },
        None => vec![],
    };
    let parent_path = file_path.parent()
        .map(|parent| parent.to_string_lossy())
        .unwrap_or_else(|| "Default Path".to_string().into());

    let db_path = format!("{parent_path}/{}.duckdb", file_name[0]);
    // let current_time = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    // let output = format!("{parent_path}/{}_{current_time}.csv", file_name[0]);

    let conn = Connection::open(&db_path)?;

    let idata = format!("CREATE TABLE IF NOT EXISTS {table} AS SELECT * FROM read_csv('{file}', all_varchar=true, sep='{sep}');");
    // 
    
    conn.execute_batch(&idata)?;

    let end = Instant::now();
    let elapsed = end - start;
    let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());

    Ok(elapsed_str)
}

fn excute_query(db_file: String, sep: String, sql: String) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();
    
    let file_path = Path::new(&db_file);
    let file_name = match file_path.file_name() {
        Some(name) => match name.to_str() {
            Some(name_str) => name_str.split('.').collect::<Vec<&str>>(),
            None => vec![],
        },
        None => vec![],
    };
    let parent_path = file_path.parent()
        .map(|parent| parent.to_string_lossy())
        .unwrap_or_else(|| "Default Path".to_string().into());
    let current_time = chrono::Local::now().format("%Y-%m-%d-%H%M%S");
    let output = format!("{parent_path}/{}_query_{current_time}.csv", file_name[0]);

    let conn = Connection::open(db_file)?;
    let edata = format!("COPY ({sql}) TO '{output}' (DELIMITER '{sep}');");
    conn.execute_batch(&edata)?;

    let end = Instant::now();
    let elapsed = end - start;
    let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());

    Ok(elapsed_str)
}

#[tauri::command]
pub async fn connect(table: String, file: String, sep: String, window: tauri::Window) -> String {
    let elapsed_time = match async { conn_db(table, file, sep) }.await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("connect error: {err}");
            window.emit("connect_err", &err.to_string()).unwrap();
            err.to_string()
        }
    };

    elapsed_time
}

#[tauri::command]
pub async fn query(file: String, sep: String, sql: String, window: tauri::Window) -> String {
    let elapsed_time = match async { excute_query(file, sep, sql) }.await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("query error: {err}");
            window.emit("query_err", &err.to_string()).unwrap();
            err.to_string()
        }
    };

    elapsed_time
}