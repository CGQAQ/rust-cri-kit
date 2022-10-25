use crate::cri;
use crate::cri::runtime_service_server::RuntimeService;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::{codegen::futures_core::Stream, Status};

#[derive(Clone)]
pub struct FooRuntimeService;

impl crate::cri::runtime_service_server::RuntimeService for FooRuntimeService {
    type GetContainerEventsStream = GetContainerEventsStream_;
}

impl FooRuntimeService {
    pub async fn get_container_events(
        &self,
        _request: tonic::Request<cri::GetEventsRequest>,
    ) -> Result<
        tonic::Response<
            <crate::runtime_service::FooRuntimeService as RuntimeService>::GetContainerEventsStream,
        >,
        tonic::Status,
    > {
        println!("FooRuntimeService#get_container_events: It works, yay!!!!!!!!!!!!!!!");

        Ok(tonic::Response::new(GetContainerEventsStream_::new(
            cri::ContainerEventResponse::default(),
        )))
    }
}

pub struct GetContainerEventsStream_ {
    response: cri::ContainerEventResponse,
}

impl GetContainerEventsStream_ {
    #[allow(dead_code)]
    pub fn new(response: cri::ContainerEventResponse) -> Self {
        GetContainerEventsStream_ { response }
    }
}

impl Stream for GetContainerEventsStream_ {
    type Item = Result<cri::ContainerEventResponse, Status>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(Ok(self.response.clone())))
    }
}
