#![deny(clippy::all)]
#![deny(clippy::pedantic)]
mod pbs;

use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use pbs::DatastoreUsuage;
use prometheus::{proto::MetricFamily, Encoder, Gauge, Opts, TextEncoder};

use prometheus::{labels, opts, Registry};

fn create_gauge(r: &Registry, opts: Opts, value: i64) {
    let gauge = Gauge::with_opts(opts).unwrap();
    r.register(Box::new(gauge.clone())).unwrap();
    #[allow(clippy::cast_precision_loss)]
    gauge.set(value as f64);
}

fn create_dataset_metrics(r: &Registry, dataset_usuage: &DatastoreUsuage) {
    create_gauge(
        r,
        opts!(
            "pbs_datastore_available_bytes",
            "Number of available bytes in the datastore",
            labels!("datastore" => &dataset_usuage.store)
        ),
        dataset_usuage.avail,
    );
    create_gauge(
        r,
        opts!(
            "pbs_datastore_total_bytes",
            "Number of total bytes in the datastore",
            labels!("datastore" => &dataset_usuage.store)
        ),
        dataset_usuage.total,
    );
    create_gauge(
        r,
        opts!(
            "pbs_datastore_used_bytes",
            "Number of used bytes in the datastore",
            labels!("datastore" => &dataset_usuage.store)
        ),
        dataset_usuage.used,
    );
    create_gauge(
        r,
        opts!(
            "pbs_datastore_estimated_full",
            "Time and Date when the datastore will be estimated full as a Epoch Unix Timestamp.",
            labels!("datastore" => &dataset_usuage.store)
        ),
        dataset_usuage.estimated_full_date,
    );
}

async fn collect_metrics() -> Vec<MetricFamily> {
    let r = Registry::new();
    let client = pbs::create_client();
    let status_response = pbs::get_status_response(client).await.unwrap();
    for dataset in status_response.data {
        create_dataset_metrics(&r, &dataset);
    }
    r.gather()
}

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = collect_metrics().await;
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 9898).into();
    println!("Listening on http://{}", addr);

    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    if let Err(err) = serve_future.await {
        eprintln!("server error: {}", err);
    }
}
