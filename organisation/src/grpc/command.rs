#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePeersetRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub peers: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "3")]
    pub initial_permission_graph:
        ::core::option::Option<PermissionGraph>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePeersetResponse {
    #[prost(string, tag = "1")]
    pub deployed_peerset_smart_contract_address:
        ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeersetCreatedRequest {
    #[prost(string, tag = "1")]
    pub deployed_peerset_smart_contract_address:
        ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeersetCreatedResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeChangeRequest {
    #[prost(string, tag = "1")]
    pub peerset_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub new_permission_graph:
        ::core::option::Option<PermissionGraph>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposeChangeResponse {
    #[prost(string, tag = "1")]
    pub proposed_change_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionGraph {
    #[prost(map = "string, message", tag = "1")]
    pub edges: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Edges,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edges {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<Node>,
    #[prost(message, repeated, tag = "2")]
    pub edges: ::prost::alloc::vec::Vec<Edge>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "NodeType", tag = "2")]
    pub r#type: i32,
    /// needed for entities managed by different peersets.
    #[prost(string, optional, tag = "3")]
    pub peerset_address:
        ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edge {
    #[prost(string, tag = "1")]
    pub destination_node_id: ::prost::alloc::string::String,
    /// todo: this should probably be something more sophisticated for now let's just make it a string.
    #[prost(string, tag = "2")]
    pub permission: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum NodeType {
    User = 0,
    Group = 1,
    Asset = 2,
}
impl NodeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NodeType::User => "USER",
            NodeType::Group => "GROUP",
            NodeType::Asset => "ASSET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(
        value: &str,
    ) -> ::core::option::Option<Self> {
        match value {
            "USER" => Some(Self::User),
            "GROUP" => Some(Self::Group),
            "ASSET" => Some(Self::Asset),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod organisation_dev_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrganisationDevClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrganisationDevClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(
            dst: D,
        ) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<
                tonic::transport::Endpoint,
            >,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?
                .connect()
                .await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrganisationDevClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner =
                tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrganisationDevClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<
                        tonic::body::BoxBody,
                    >>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrganisationDevClient::new(InterceptedService::new(
                inner,
                interceptor,
            ))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// / create s a new peerset with a set of peers.
        pub async fn create_peerset(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreatePeersetRequest,
            >,
        ) -> Result<
            tonic::Response<super::CreatePeersetResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!(
                        "Service was not ready: {}",
                        e.into()
                    ),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/command.OrganisationDev/CreatePeerset",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        /// notify peer that a new peerset has been created,
        pub async fn peerset_created(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PeersetCreatedRequest,
            >,
        ) -> Result<
            tonic::Response<super::PeersetCreatedResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!(
                        "Service was not ready: {}",
                        e.into()
                    ),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/command.OrganisationDev/PeersetCreated",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
        /// triggers process of proposing a change to the permission graph.
        pub async fn propose_change(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ProposeChangeRequest,
            >,
        ) -> Result<
            tonic::Response<super::ProposeChangeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!(
                        "Service was not ready: {}",
                        e.into()
                    ),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/command.OrganisationDev/ProposeChange",
            );
            self.inner
                .unary(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod organisation_dev_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrganisationDevServer.
    #[async_trait]
    pub trait OrganisationDev: Send + Sync + 'static {
        /// / create s a new peerset with a set of peers.
        async fn create_peerset(
            &self,
            request: tonic::Request<
                super::CreatePeersetRequest,
            >,
        ) -> Result<
            tonic::Response<super::CreatePeersetResponse>,
            tonic::Status,
        >;
        /// notify peer that a new peerset has been created,
        async fn peerset_created(
            &self,
            request: tonic::Request<
                super::PeersetCreatedRequest,
            >,
        ) -> Result<
            tonic::Response<super::PeersetCreatedResponse>,
            tonic::Status,
        >;
        /// triggers process of proposing a change to the permission graph.
        async fn propose_change(
            &self,
            request: tonic::Request<
                super::ProposeChangeRequest,
            >,
        ) -> Result<
            tonic::Response<super::ProposeChangeResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OrganisationDevServer<T: OrganisationDev> {
        inner: _Inner<T>,
        accept_compression_encodings:
            EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OrganisationDev> OrganisationDevServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(
                ),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(
                Self::new(inner),
                interceptor,
            )
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(
            mut self,
            encoding: CompressionEncoding,
        ) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
        for OrganisationDevServer<T>
    where
        T: OrganisationDev,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(
            &mut self,
            req: http::Request<B>,
        ) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/command.OrganisationDev/CreatePeerset" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePeersetSvc<T: OrganisationDev>(
                        pub Arc<T>,
                    );
                    impl<T: OrganisationDev>
                        tonic::server::UnaryService<
                            super::CreatePeersetRequest,
                        > for CreatePeersetSvc<T>
                    {
                        type Response =
                            super::CreatePeersetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreatePeersetRequest,
                            >,
                        ) -> Self::Future
                        {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .create_peerset(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePeersetSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/command.OrganisationDev/PeersetCreated" => {
                    #[allow(non_camel_case_types)]
                    struct PeersetCreatedSvc<T: OrganisationDev>(
                        pub Arc<T>,
                    );
                    impl<T: OrganisationDev>
                        tonic::server::UnaryService<
                            super::PeersetCreatedRequest,
                        > for PeersetCreatedSvc<T>
                    {
                        type Response =
                            super::PeersetCreatedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PeersetCreatedRequest,
                            >,
                        ) -> Self::Future
                        {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .peerset_created(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PeersetCreatedSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/command.OrganisationDev/ProposeChange" => {
                    #[allow(non_camel_case_types)]
                    struct ProposeChangeSvc<T: OrganisationDev>(
                        pub Arc<T>,
                    );
                    impl<T: OrganisationDev>
                        tonic::server::UnaryService<
                            super::ProposeChangeRequest,
                        > for ProposeChangeSvc<T>
                    {
                        type Response =
                            super::ProposeChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ProposeChangeRequest,
                            >,
                        ) -> Self::Future
                        {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .propose_change(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings =
                        self.accept_compression_encodings;
                    let send_compression_encodings =
                        self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposeChangeSvc(inner);
                        let codec =
                            tonic::codec::ProstCodec::default();
                        let mut grpc =
                            tonic::server::Grpc::new(codec)
                                .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header(
                            "content-type",
                            "application/grpc",
                        )
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: OrganisationDev> Clone for OrganisationDevServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self
                    .accept_compression_encodings,
                send_compression_encodings: self
                    .send_compression_encodings,
            }
        }
    }
    impl<T: OrganisationDev> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OrganisationDev> tonic::server::NamedService
        for OrganisationDevServer<T>
    {
        const NAME: &'static str = "command.OrganisationDev";
    }
}
