use crate::config::RegisterConfig;

pub async fn write_to_influx(db : influxDB, register_config: RegisterConfig, value : i64) -> Result<(), Box<dyn std::error::Error>> {
    use futures::prelude::*;
    use influxdb2::models::DataPoint;
    use influxdb2::Client;
    
    
    let client = Client::new(db.host, db.org, db.token);

    let points = vec![
        DataPoint::builder(&db.bucket)
            .field(register_config.name, value)
            .build()?,
    ];
    

    client.write(&db.bucket, stream::iter(points)).await?;

    Ok(())
}

pub struct influxDB
{
    pub host : String,
    pub org : String,
    pub token : String,
    pub bucket : String,
}