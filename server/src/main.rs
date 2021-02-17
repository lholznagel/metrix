use cachem::{ConnectionPool, Protocol};
use metrix_db::{FetchAllMetricInfosReq, FetchAllMetricInfosRes, FetchMetricsLastBulkReq, FetchMetricsLastBulkRes, FetchMetricsLastReq, FetchMetricsLastRes};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["tracing".into()]);

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
            .map(move || _self.clone());

        let infos = root
            .clone()
            .and(warp::path!("infos" / ..));
        let all_infos = infos
            .clone()
            .and(warp::path::end())
            .and(warp::get())
            .and_then(Self::fetch_all_metric_infos);
        let infos = all_infos;

        let history = root
            .clone()
            .and(warp::path!("history" / ..));
        let last = history
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::get())
            .and_then(Self::fetch_last);
        let last_bulk = history
            .clone()
            .and(warp::path!("bulk"))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::fetch_last_bulk);
        let history = last
            .or(last_bulk);

        let api = infos
            .or(history);

        warp::serve(api)
            .run(([0, 0, 0, 0], 8889))
            .await;
    }

    async fn fetch_all_metric_infos(
        self: Self,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, FetchAllMetricInfosRes>(
            &mut conn,
            FetchAllMetricInfosReq::default()
        )
        .await
        .unwrap();

        Ok(warp::reply::json(&data.0))
    }

    async fn fetch_last(
        self: Self,
        id: Uuid,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, FetchMetricsLastRes>(
            &mut conn,
            FetchMetricsLastReq(id)
        )
        .await
        .unwrap();

        Ok(warp::reply::json(&data.0))
    }

    async fn fetch_last_bulk(
        self: Self,
        ids: Vec<Uuid>,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, FetchMetricsLastBulkRes>(
            &mut conn,
            FetchMetricsLastBulkReq(ids)
        )
        .await
        .unwrap();

        Ok(warp::reply::json(&data.0))
    }
}
