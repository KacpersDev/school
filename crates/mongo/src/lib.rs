use mongodb::{Client, Collection, bson::{Document, doc}};

pub struct Student {
    pub name: String,
    pub class: String,
    pub credit: u32,
    pub notes: Vec<&'static str>
}

pub struct Teacher {
    pub name: String,
    pub class: String,
    pub subject: String,
}

pub struct Class {
    pub name: String,
    pub students: Vec<&'static str>,
    pub time_table: Vec<&'static str>,
    pub absence: Vec<Absence>,
}

pub struct Absence {
    pub class: String,
    pub students: Vec<&'static str>,
}   

pub enum EntityType {
    STUDENT(Student),
    TEACHER(Teacher)
}

pub async fn create_entity(entity: EntityType) -> mongodb::error::Result<()> {

    let client = Client::with_uri_str("uri").await?;
    let database = client.database("school");

    match entity {
        EntityType::STUDENT(student) => {
            let student_collection: Collection<Document> = database.collection("students");

            let insert_document = doc! {
                "name": student.name,
                "class": student.class,
                "credit": student.credit,
                "notes": student.notes,
            };

            student_collection.insert_one(insert_document).await?;
        }
        EntityType::TEACHER(teacher) => {
            let teacher_collection: Collection<Document> = database.collection("teachers");

            let insert_document = doc! {
                "name": teacher.name,
                "class": teacher.class,
                "subject": teacher.subject,
            };

            teacher_collection.insert_one(insert_document).await?;
        }
    }

    Ok(())
}

pub async fn create_class(class: Class) -> mongodb::error::Result<()> {

    let client = Client::with_uri_str("uri").await?;
    let database = client.database("classes");
    let classes_collection: Collection<Document> = database.collection("classes");

    let insert_document = doc! {
        "name": class.name,
        "students": class.students,
        "timetable": class.time_table,
    };

    classes_collection.insert_one(insert_document).await?;

    Ok(())
}

pub async fn create_absence(absence: Absence) -> mongodb::error::Result<()> {

    let client = Client::with_uri_str("uri").await?;
    let database = client.database("name");
    let absence_collection: Collection<Document> = database.collection("absences");

    let insert_document = doc! {
        "class": absence.class,
        "students": absence.students,
    };

    absence_collection.insert_one(insert_document).await?;

    Ok(())
}