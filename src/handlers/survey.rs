use axum::{Json, extract::Extension};
use hyper::StatusCode;
use mongodb::{Client, bson::doc};
use mongodb::options::AggregateOptions;
use futures::stream::TryStreamExt;
use std::sync::Arc;
use std::env;
use crate::models::investorInfo::InvestorInfo;
use crate::models::survey::{SurveyResponse, SurveyResult};
use crate::handlers::send_email::send_email_oauth2;
use crate::handlers::email::{generate_oauth_url, AUTH_CODE};

pub async fn handle_survey(
    Json(payload): Json<SurveyResponse>,
    Extension(mongo_client): Extension<Arc<Client>>,
) -> Result<Json<SurveyResult>, StatusCode> {
    println!("Received survey payload: {:?}", payload);
    let fixed_industry = "Software";
    println!("Starting survey handling with fixed industry: {}", fixed_industry);

    // Поиск инвесторов в MongoDB
    let db = mongo_client.database("YoctoDB");
    let collection = db.collection::<InvestorInfo>("Yocto-Investors");

    // Параметры агрегации
    let aggregate_pipeline = vec![
        doc! { "$match": { "Preferred Sectors": { "$exists": true }, "Email": { "$exists": true } } },
        doc! { "$match": { "Preferred Sectors": fixed_industry } },
        doc! { "$limit": 100 }
    ];
    let aggregate_options = AggregateOptions::builder().build();

    // Выполнение агрегации
    println!("Executing MongoDB aggregation...");
    let mut cursor = collection.aggregate(aggregate_pipeline, aggregate_options).await.unwrap();
    
    let mut matching_investors: Vec<InvestorInfo> = Vec::new();

    while let Some(doc) = cursor.try_next().await.unwrap() {
        let investor: InvestorInfo = mongodb::bson::from_document(doc).unwrap();
        matching_investors.push(investor);
    }

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let redirect_url = env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    let (auth_url, _csrf_token) = generate_oauth_url(&client_id, &redirect_url);
    println!("Please go to this URL and authorize the application: {}", auth_url);

    let auth_code = {
        let auth_code = AUTH_CODE.lock().unwrap();
        auth_code.clone().unwrap_or_else(|| {
            println!("No authorization code found. Please authorize the application first.");
            std::process::exit(1);
        })
    };

    let email = "vladeliseykin2101@gmail.com";
    let subject = "Test";
    let body = "Test";
    let from_email = "veliseykin2000@gmail.com";

    match send_email_oauth2(email, subject, body, from_email, &auth_code, &client_id, &client_secret, &redirect_url).await {
        Ok(_) => println!("Email sent to {}", email),
        Err(e) => println!("Failed to send email to {}: {:?}", email, e),
    }

    Ok(Json(SurveyResult {
        message: "Survey submitted successfully".to_string(),
    }))
}