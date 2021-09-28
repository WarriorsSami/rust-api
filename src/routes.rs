use crate::db::Conn as DbConn;
use super::models::{Book, NewBook};
use serde_json::Value;
use rocket_contrib::json::Json;
use serde::__private::ptr::null;

#[get("/books", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let books = Book::all(&conn);
    let status = if books.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": books,
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    let status = if Book::insert(new_book.into_inner(), &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": Book::all(&conn).first(),
    }))
}

#[get("/books/<id>", format = "application/json")]
pub fn show_by_id(conn: DbConn, id: i32) -> Json<Value> {
    let book = Book::show(id, &conn);
    let status = if book.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": book.get(0),
    }))
}

#[put("books/<id>", format = "application/json", data = "<book>")]
pub fn update(conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status = if Book::update_by_id(id, &conn, book.into_inner()) {
        200
    } else {
        400
    };

    let result = if status == 200 {
        Book::show(id, &conn).get(0)
    } else {
        null
    };

    Json(json!({
        "status": status,
        "result": result,
    }))
}

#[delete("books/<id>", format = "application/json")]
pub fn delete(conn: DbConn, id: i32) -> Json<Value> {
    let status = if Book::delete_by_id(id, &conn) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("books/authors/<author>", format = "application/json")]
pub fn show_by_author(conn: DbConn, author: String) -> Json<Value> {
    let books = Book::all_by_author(author, &conn);
    let status = if books.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": books,
    }))
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Unavailable resource",
    }))
}