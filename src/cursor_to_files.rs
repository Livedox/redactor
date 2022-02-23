use mongodb::Cursor;
use futures::StreamExt;

use crate::db::File;

pub async fn cursor_to_files(mut cursor: Cursor<File>) -> Vec<File> {
    let mut files:Vec<File> = vec![];

    while let Some(doc) = cursor.next().await {
        files.push(doc.expect("Error in cursor_to_files"));
    }

    files
}