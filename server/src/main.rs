use cachem::{ConnectionPool, EmptyMsg, Protocol};
use metrix_db::{FetchAllMetricInfosReq, FetchAllMetricInfosRes, FetchMetricFilter, FetchMetricsHistoryReq, FetchMetricsHistoryRes, FetchMetricsLastBulkReq, FetchMetricsLastBulkRes, FetchMetricsLastReq, FetchMetricsLastRes, InsertMetricsEntry, InsertMetricsReq, LookupMetricIdReq, LookupMetricIdRes};
use metrix_exporter::Metrix;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    morgan::Morgan::init(vec!["tracing".into()]);

    let metrix = Metrix::new(env!("CARGO_PKG_NAME").into(), "0.0.0.0:8889").await?;
    let pool = ConnectionPool::new("0.0.0.0:8888".into(), metrix.get_sender(), 25).await?;

    tokio::task::spawn(async { metrix.listen().await; });

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
            .and(warp::path!("api" / ..));

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
        let raw = history
            .clone()
            .and(warp::path!(Uuid / "raw"))
            .and(warp::get())
            .and_then(Self::fetch_raw);
        let history = last
            .or(last_bulk)
            .or(raw);

        let metric = root
            .clone()
            .and(warp::path!("metrics" / ..));
        let insert_metric = metric
            .clone()
            .and(warp::path!(Uuid))
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::insert_metric);
        let lookup_metric = metric
            .clone()
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::json())
            .and_then(Self::lookup_id);
        let metric = insert_metric
            .or(lookup_metric);

        let api = infos
            .or(history)
            .or(metric);

        warp::serve(api)
            .run(([0, 0, 0, 0], 8889))
            .await;
    }

    async fn fetch_raw(
        self: Self,
        id: Uuid,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let data = Protocol::request::<_, FetchMetricsHistoryRes>(
            &mut conn,
            FetchMetricsHistoryReq(FetchMetricFilter {
                id,
                ts_start: 0,
            })
        )
        .await
        .unwrap();

        Ok(warp::reply::json(&data.0))
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

    async fn insert_metric(
        self: Self,
        id: Uuid,
        val: u128,
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        if let Err(e) = Protocol::request::<_, EmptyMsg>(
            &mut conn,
            InsertMetricsReq(
                InsertMetricsEntry {
                    id,
                    value: val,
                }
            )
        )
        .await {
            log::error!("Error writing into database. Error: {:?}", e);
        }

        Ok(warp::reply::json(&""))
    }

    async fn lookup_id(
        self: Self,
        name: String
    ) -> Result<impl Reply, Rejection> {
        let mut conn = self.pool.acquire().await.unwrap();
        let id = Protocol::request::<_, LookupMetricIdRes>(
            &mut conn,
            LookupMetricIdReq(name),
        )
        .await
        .map(|x| x.0)
        .unwrap();

        Ok(warp::reply::json(&id.id))
    }
}
