use axum::http::HeaderMap;
use axum::{extract::State, Json};
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::database::sessions::{Entity as Sessions, self};

use crate::utils::jwt::create_token;


pub async fn create_unique_session(
    State(database): State<DatabaseConnection>,
) -> Json<String> {

    loop {
        if let Ok(unique_session) = create_token() {
            // new uniqueID was generated okay

            if let Ok(session) = Sessions::find()
                .filter(sessions::Column::UniqueId.eq(&unique_session))
                .one(&database)
                .await
                .map_err(|_| "Failed to query") {
                    // Query was fine

                    if session.is_some() {
                        // this uniqueID is already in database, need to generate a new one
                        continue;
                    } else {
                        // the uniqueID is clear, you can use it (submit to db and return)
                        let mut new_session = sessions::ActiveModel::new();
                        new_session.unique_id = Set(unique_session.clone());
                        
                        match new_session.save(&database).await {
                            Err(_) => { continue; },
                            Ok(_) => { }
                        };

                        return Json(unique_session);
                    }
                } else {
                    // The query was ruined for some reason
                }
        } else {
            // uniqueID wasn't generated properly
            continue;
        }
    }
}


pub async fn validate_unique_session(
    State(database): State<DatabaseConnection>,
    header: HeaderMap
) -> Json<String> {

    if let Some(given_session) = header.get("UniqueID") {
        let st_session = given_session.to_str().unwrap().to_string();

        if let Ok(session) = Sessions::find()
            .filter(sessions::Column::UniqueId.eq(&st_session))
            .one(&database)
            .await
            .map_err(|_| "Failed to query") {
                // Query was fine

                if session.is_some() {
                    return Json(st_session);
                } else {
                    // the uniqueID is clear, you can use it (submit to db and return)
                    return create_unique_session(State(database.clone())).await;
                }
            } else {
                return create_unique_session(State(database.clone())).await;
            }
    } else {
        return create_unique_session(State(database.clone())).await;
    }
}