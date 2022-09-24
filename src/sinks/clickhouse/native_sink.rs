use bytes::{BufMut, Bytes, BytesMut};
use futures::{FutureExt, SinkExt, stream::BoxStream};
use http::{Request, StatusCode, Uri};
use hyper::Body;
use snafu::ResultExt;
use clickhouse_rs::{Block, Pool};

use super::ClickhouseConfig;
use crate::{
    codecs::Transformer,
    config::SinkContext,
    event::Event,
    sinks::{
        util::{
            retries::{RetryAction, RetryLogic},
            Buffer, TowerRequestConfig, StreamSink,
        },
        Healthcheck, HealthcheckError, UriParseSnafu, VectorSink,
    },
    tls::TlsSettings,
};

async fn build_native_sink(cfg: &ClickhouseConfig, cx: SinkContext) -> crate::Result<(VectorSink, Healthcheck)> {
    let batch = cfg.batch.into_batch_settings()?;
    let tls_settings = TlsSettings::from_options(&cfg.tls)?;
    let pool = Pool::new("");
    unimplemented!()
}

async fn healthcheck(pool: Pool) -> crate::Result<()> {
    let mut client = pool.get_handle().await?;
    client.ping().await
}

struct NativeClickhouseSink {
    pool: Pool,
    columns: Vec<String>,
}

impl NativeClickhouseSink {
    async fn run_inner(self: Box<Self>, input: BoxStream<'_, Event>) -> Result<(), ()> {
        unimplemented!()
    }
}

impl StreamSink<Event> for NativeClickhouseSink {
    async fn run(self: Box<Self>, input: stream::BoxStream<'_, T>) -> Result<(), ()> {
        self.run_inner(input).await
    }
}