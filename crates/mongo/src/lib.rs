use mongodb::{Client, Collection, bson::{Document, doc}};

pub struct Student {
    pub name: String,
    pub class: String,
    pub credit: u32,
}

pub async fn create_student(student: Student) -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
    let database = client.database("school");
    let student_collection: Collection<Document> = database.collection("students");

    let insert_document = doc! {
        "name": student.name,
        "class": student.class,
        "credit": student.credit,
    };

    let result = student_collection.insert_one(insert_document).await?;
    println!("result id {}", result.inserted_id);

    Ok(())
}
