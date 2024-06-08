#[derive(Debug, Serialize, Deserialize)]
pub struct SandwichMongo {
    _id: ObjectId,
    name: String,
    ingredients: Vec<String>,
    sandwich_type: SandwichType,
}

impl From<Sandwich> for SandwichMongo {
    fn from(sandwich: Sandwich) -> Self {
        let object_id = match sandwich.id().value() {
            Some(id) => ObjectId::from_str(id).unwrap(),
            None => ObjectId::new(),
        };

        let sand_mongo = SandwichMongo {
            _id: object_id,
            name: sandwich.name().value().to_string(),
            ingredients: sandwich.ingredients().value().clone(),
            sandwich_type: sandwich.sandwich_type().clone(),
        };

        sand_mongo
    }
}

impl TryInto<Sandwich> for SandwichMongo {
    type Error = String;

    fn try_into(self) -> Result<Sandwich, Self::Error> {
        Sandwich::new(
            self._id.to_string(),
            self.name,
            self.ingredients,
            self.sandwich_type,
        )
    }
}

#[derive(Clone)]
pub struct SandwichMongoRepository {
    database: String,
    collection: String,
    conn_uri: String,
}

impl SandwichMongoRepository {
    async fn open_connection(&self) -> Client {
        let c = Client::with_uri_str(&self.conn_uri);
        c.await
            .expect("Error while opening the connection with MongoDB")
    }

    async fn get_collection(&self) -> Collection<SandwichMongo> {
        let client = self.open_connection().await;
        client.database(&self.database).collection(&self.collection)
    }
}

#[async_trait]
impl Repository<Sandwich> for SandwichMongoRepository {
    /// new constructor function
    fn new(config: &PersistenceConfig) -> Result<Self, String>
    where
        Self: Sized,
    {
        config.validate()?;
        let config = config.clone();
        let conn_uri = create_connection_uri(&config);

        Ok(SandwichMongoRepository {
            database: config.database,
            collection: config.schema_collection,
            conn_uri,
        })
    }

    async fn create(&self, sandwich: Sandwich) -> Result<Sandwich, RepoCreateError> {
        let sand_mongo = SandwichMongo::from(sandwich.clone());

        let recipes_coll = self.get_collection().await;

        let result = recipes_coll.insert_one(sand_mongo, None).await;
        let inserted_id = match result {
            Ok(e) => e.inserted_id.as_object_id().unwrap(),
            Err(e) => return Err(RepoCreateError::Unknown(e.to_string())),
        };

        let created = Sandwich::new(
            inserted_id.to_string(),
            sandwich.name().value().to_string(),
            sandwich.ingredients().value().clone(),
            sandwich.sandwich_type().clone(),
        )
        .unwrap();
        Ok(created)
    }

    async fn find_one(&self, sandwich: FindSandwich) -> Result<Sandwich, RepoSelectError> {
        let recipes_coll = self.get_collection().await;

        let document = self.compose_document_from_sandwich(sandwich).unwrap();
        let result: Result<Option<SandwichMongo>, Error> =
            recipes_coll.find_one(document, None).await;

        let found = match result {
            Ok(s) => match s {
                Some(s) => s,
                None => return Err(RepoSelectError::NotFound),
            },
            Err(_) => return Err(RepoSelectError::Unknown(String::from("unknown error"))),
        };

        let sandwich: Result<Sandwich, String> = found.try_into();
        Ok(sandwich.unwrap())
    }

    fn compose_document_from_sandwich(&self, sandwich: FindSandwich) -> Result<Document, Error> {
        if sandwich.id.is_some() {
            let id = sandwich.id.as_ref().unwrap();
            Ok(doc! {
                "_id": bson::oid::ObjectId::parse_str(id).unwrap()
            })
        } else {
            let mut doc = doc! {};

            if !sandwich.name.is_empty() {
                doc.insert("name", sandwich.name);
            }

            if !sandwich.ingredients.is_empty() {
                doc.insert(
                    "ingredients",
                    doc! {
                        "$all": sandwich.ingredients
                    },
                );
            }

            Ok(doc)
        }
    }

    async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<Sandwich>, RepoFindAllError> {
        let recipes_coll = self.get_collection().await;

        let document = self.compose_document_from_sandwich(sandwich).unwrap();
        let res = recipes_coll.find(document, None).await;

        let mut cursor = match res {
            Ok(c) => c,
            Err(_) => return Ok(vec![]),
        };

        let mut sand_vec: Vec<Sandwich> = Vec::new();

        while cursor
            .advance()
            .await
            .map_err(|_| RepoFindAllError::Unknown(String::from("Cursor iteration error")))?
        {
            let sand = match cursor.deserialize_current() {
                Ok(s) => match s.try_into() {
                    Ok(s) => s,
                    Err(s) => return Err(RepoFindAllError::Unknown(String::from(s))),
                },
                Err(_) => {
                    return Err(RepoFindAllError::Unknown(String::from(
                        "Error while deserializing",
                    )))
                }
            };
            sand_vec.push(sand);
        }

        Ok(sand_vec)
    }

    async fn update(&self, sandwich: Sandwich) -> Result<Sandwich, RepoUpdateError> {
        let sand_mongo = SandwichMongo::from(sandwich.clone());

        let recipes_coll = self.get_collection().await;

        let res = recipes_coll
            .update_one(
                doc! {
                    "_id": sand_mongo._id
                },
                doc! {
                    "$set": {
                        "name": sand_mongo.name,
                        "ingredients": sand_mongo.ingredients
                    }
                },
                None,
            )
            .await;

        return match res {
            Ok(ur) => {
                if ur.matched_count > 0 {
                    Ok(sandwich.to_owned())
                } else {
                    Err(RepoUpdateError::NotFound)
                }
            }
            Err(_) => Err(RepoUpdateError::Unknown(String::from(
                "An error occurred while updating the document",
            ))),
        };
    }

    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
        let object_id = match ObjectId::from_str(id) {
            Ok(id) => id,
            Err(e) => return Err(RepoDeleteError::InvalidData(e.to_string())),
        };

        let recipes_coll = self.get_collection().await;

        let res = recipes_coll
            .delete_one(
                doc! {
                    "_id": object_id
                },
                None,
            )
            .await;

        match res {
            Ok(dr) => {
                if dr.deleted_count == 1 {
                    Ok(())
                } else {
                    Err(RepoDeleteError::NotFound)
                }
            }
            Err(_) => Err(RepoDeleteError::Unknown(String::from(
                "An error occurred during the deletion",
            ))),
        }
    }
}

fn create_connection_uri(config: &PersistenceConfig) -> String {
    format!(
        "mongodb://{}:{}@{}/{}",
        config.user,
        config.password,
        match config.port {
            None => config.host.to_string(),
            Some(port) => config.host.clone() + ":" + &port.to_string(),
        },
        config.auth_db
    )
}
