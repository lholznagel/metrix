use cachem::{ConnectionPool, Protocol};
use metrix_db::{FetchMetricLatestFilter, FetchMetricsLatestReq, FetchMetricsLatestRes};
use metrix_exporter::Metrix;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec![]);

    let mut metrix = Metrix::new("0.0.0.0:8888").await.unwrap();
    metrix.register(vec!["my::cool::metric"]).await.unwrap();
    let sender = metrix.get_sender();
    tokio::task::spawn(async move {
        metrix.listen().await
    });
    // sender.send("my::cool::metric", 1u128).await;

    let pool = ConnectionPool::new("0.0.0.0:8888".into(), 10).await?;

    ApiServer::new(pool).serve().await;

    Ok(())
}

#[derive(Clone)]
struct ApiServer {
    pool: ConnectionPool
}

impl ApiServer {
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn serve(&self) {
        let _self = self.clone();

        let root = warp::any()
            .map(move || _self.clone())
            .and(warp::path!("api" / "v1" / Uuid))
            .and(warp::get())
            .and_then(Self::fetch_by_id);

        warp::serve(root)
            .run(([0, 0, 0, 0], 8889))
            .await;
    }

    async fn fetch_by_id(
        self: Self,
        id: Uuid,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, FetchMetricsLatestRes>(
            &mut conn,
            FetchMetricsLatestReq(
                FetchMetricLatestFilter {
                    id
                }
            )
        )
        .await
        .unwrap();

        Ok(warp::reply::json(&data.0))
    }
}
