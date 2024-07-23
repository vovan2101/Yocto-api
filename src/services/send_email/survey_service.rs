use hyper::StatusCode;
use mongodb::{Client, bson::doc};
use mongodb::options::AggregateOptions;
use futures::stream::TryStreamExt;
use std::sync::Arc;
use crate::models::investor_info::InvestorInfo;
use crate::models::survey::SurveyResponse;

pub async fn process_survey(
    payload: SurveyResponse,
    mongo_client: Arc<Client>,
) -> Result<Vec<InvestorInfo>, StatusCode> {
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

    Ok(matching_investors)
}
