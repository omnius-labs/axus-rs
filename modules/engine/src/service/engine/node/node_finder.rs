use std::sync::Arc;

use core_base::clock::SystemClock;
use futures::future::JoinAll;
use ring::rand::thread_rng;
use sqlx::types::chrono::Utc;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::{
    model::NodeRef,
    service::{
        session::{
            model::{Session, SessionType},
            SessionAccepter, SessionConnector,
        },
        util::VolatileHashSet,
    },
};

use super::{NodeRefFetcher, NodeRefRepo};

#[allow(dead_code)]
pub struct NodeFinder {
    session_connector: Arc<SessionConnector>,
    session_accepter: Arc<SessionAccepter>,
    node_fetcher: Arc<dyn NodeRefFetcher + Send + Sync>,
    system_clock: Arc<dyn SystemClock<Utc> + Send + Sync>,
    option: NodeFinderOptions,
    cancellation_token: CancellationToken,
    join_handles: Arc<Mutex<Option<JoinAll<tokio::task::JoinHandle<()>>>>>,
}

pub struct NodeFinderOptions {
    pub state_directory_path: String,
    pub max_connected_session_count: u32,
    pub max_accepted_session_count: u32,
}
#[allow(dead_code)]
struct SessionStatus {
    id: Vec<u8>,
    node_ref: NodeRef,
    session: Session,
}

impl NodeFinder {
    pub async fn new(
        session_connector: Arc<SessionConnector>,
        session_accepter: Arc<SessionAccepter>,
        node_ref_repo: Arc<NodeRefRepo>,
        node_fetcher: Arc<dyn NodeRefFetcher + Send + Sync>,
        system_clock: Arc<dyn SystemClock<Utc> + Send + Sync>,
        option: NodeFinderOptions,
    ) -> Self {
        let cancellation_token = CancellationToken::new();

        let result = Self {
            session_connector,
            session_accepter,
            node_fetcher,
            system_clock,
            option,
            cancellation_token,
            join_handles: Arc::new(Mutex::new(None)),
        };
        result.create_tasks().await;

        result
    }

    async fn create_tasks(&self) {
        todo!()
    }

    async fn internal_connect(
        session_connector: Arc<SessionConnector>,
        connected_node_refs: Arc<Mutex<VolatileHashSet<NodeRef>>>,
        node_ref_repo: Arc<NodeRefRepo>,
    ) -> anyhow::Result<()> {
        connected_node_refs.lock().await.refresh();

        let mut rng = thread_rng();

        match arr.choose(&mut rng) {
            Some(&number) => println!("Randomly selected number is {}", number),
            None => println!("The array is empty!"),
        }

        for v in node_ref_repo.get_node_refs().await? {
            if connected_node_refs.lock().await.contains(&v) {
                continue;
            }

            for a in v.addrs {
                let session = session_connector.connect(&a, &SessionType::NodeFinder).await?;
                let id = Self::handshake(&session).await?;
            }

            connected_node_refs.lock().await.insert(v);
        }

        Ok(())
    }

    async fn handshake(_session: &Session) -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}
