#![allow(dead_code)]
use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
    pg::{self, Decode},
};

// The environment variable set in `spin.toml` that points to the
// address of the Pg server that the component will write to
const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Clone)]
struct NumericRow {
    intid: i32,
    rsmallserial: i16,
    rsmallint: i16,
    rint2: i16,
    rserial: i32,
    rint: i32,
    rint4: i32,
    rbigserial: i64,
    rbigint: i64,
    rint8: i64,
    rreal: f32,
    rdouble: f64,
}

#[derive(Debug, Clone)]
struct CharacterRow {
    rchar: String,
    rvarchar: String,
    rtext: String,
}

#[derive(Debug, Clone)]
struct GeneralRow<'a> {
    rbool: bool,
    obool: Option<bool>,
    rbytea: &'a str,
}

#[http_component]
fn process(req: Request) -> Result<Response> {
    match req.uri().path() {
        "/test_character_types" => test_character_types(req),
        "/test_numeric_types" => test_numeric_types(req),
        "/test_general_types" => test_general_types(req),
        "/pg_backend_pid" => pg_backend_pid(req),
        _ => Ok(http::Response::builder()
            .status(404)
            .body(Some("Not found".into()))?),
    }
}

fn test_numeric_types(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let create_table_sql = r#"
        CREATE TEMPORARY TABLE test_numeric_types (
            intid integer GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
            rsmallserial smallserial NOT NULL,
            rsmallint smallint NOT NULL,
            rint2 int2 NOT NULL,
            rserial serial NOT NULL,
            rint int NOT NULL,
            rint4 int4 NOT NULL,
            rbigserial bigserial NOT NULL,
            rbigint bigint NOT NULL,
            rint8 int8 NOT NULL,
            rreal real NOT NULL,
            rdouble double precision NOT NULL
         );
    "#;

    pg::execute(&address, create_table_sql, &[])?;

    let insert_sql = r#"
        INSERT INTO test_numeric_types
            (rsmallint, rint2, rint, rint4, rbigint, rint8, rreal, rdouble)
        VALUES
            (0, 0, 0, 0, 0, 0, 0, 0);
    "#;

    pg::execute(&address, insert_sql, &[])?;

    let sql = r#"
        SELECT
            intid,
            rsmallserial,
            rsmallint,
            rint2,
            rserial,
            rint,
            rint4,
            rbigserial,
            rbigint,
            rint8,
            rreal,
            rdouble
        FROM test_numeric_types;
    "#;

    let rowset = pg::query(&address, sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let intid = i32::decode(&row[0])?;
        let rsmallserial = i16::decode(&row[1])?;
        let rsmallint = i16::decode(&row[2])?;
        let rint2 = i16::decode(&row[3])?;
        let rserial = i32::decode(&row[4])?;
        let rint = i32::decode(&row[5])?;
        let rint4 = i32::decode(&row[6])?;
        let rbigserial = i64::decode(&row[7])?;
        let rbigint = i64::decode(&row[8])?;
        let rint8 = i64::decode(&row[9])?;
        let rreal = f32::decode(&row[10])?;
        let rdouble = f64::decode(&row[11])?;

        let row = NumericRow {
            intid,
            rsmallserial,
            rsmallint,
            rint2,
            rserial,
            rint,
            rint4,
            rbigserial,
            rbigint,
            rint8,
            rreal,
            rdouble,
        };

        response_lines.push(format!("row: {:#?}", row));
    }

    let response = format!(
        "Found {} rows(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn test_character_types(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let create_table_sql = r#"
        CREATE TEMPORARY TABLE test_character_types (
            rvarchar varchar(40) NOT NULL,
            rtext text NOT NULL,
            rchar char(10) NOT NULL
         );
    "#;

    pg::execute(&address, create_table_sql, &[])?;

    let insert_sql = r#"
        INSERT INTO test_character_types
            (rvarchar, rtext, rchar)
        VALUES
            ('rvarchar', 'rtext', 'rchar');
    "#;

    pg::execute(&address, insert_sql, &[])?;

    let sql = r#"
        SELECT
            rvarchar, rtext, rchar
        FROM test_character_types;
    "#;

    let rowset = pg::query(&address, sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let rvarchar = String::decode(&row[0])?;
        let rtext = String::decode(&row[1])?;
        let rchar = String::decode(&row[2])?;

        let row = CharacterRow {
            rvarchar,
            rtext,
            rchar,
        };

        response_lines.push(format!("row: {:#?}", row));
    }

    let response = format!(
        "Found {} rows(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn test_general_types(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let create_table_sql = r#"
        CREATE TEMPORARY TABLE test_general_types (
            rbool bool NOT NULL,
            obool bool,
            rbytea bytea NOT NULL
         );
    "#;

    pg::execute(&address, create_table_sql, &[])?;

    let insert_sql = r#"
        INSERT INTO test_general_types
            (rbool, rbytea)
        VALUES
            (TRUE, '\176'::bytea);
    "#;

    pg::execute(&address, insert_sql, &[])?;

    let sql = r#"
        SELECT
            rbool, obool, rbytea
        FROM test_general_types;
    "#;

    let rowset = pg::query(&address, sql, &[])?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let rbool = bool::decode(&row[0])?;
        let obool = Option::<bool>::decode(&row[1])?;
        let rbytea = Vec::<u8>::decode(&row[2])?;

        let row = GeneralRow {
            rbool,
            obool,
            rbytea: std::str::from_utf8(&rbytea).unwrap(),
        };

        response_lines.push(format!("row: {:#?}", row));
    }

    let response = format!(
        "Found {} rows(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn pg_backend_pid(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;
    let sql = "SELECT pg_backend_pid()";

    let get_pid = || {
        let rowset = pg::query(&address, sql, &[])?;

        let row = &rowset.rows[0];
        i32::decode(&row[0])
    };

    assert_eq!(get_pid()?, get_pid()?);

    let response = format!("pg_backend_pid: {}\n", get_pid()?);

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn format_col(column: &pg::Column) -> String {
    format!("{}:{:?}", column.name, column.data_type)
}