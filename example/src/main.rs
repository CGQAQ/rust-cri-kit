mod cri;
mod image_service;
mod runtime_service;
use crate::cri::image_service_server::ImageService;
use crate::cri::runtime_service_server::RuntimeService;

async fn call_image_status(
    image_service: &impl ImageService,
) -> Result<(), Box<dyn std::error::Error>> {
    image_service
        .image_status(tonic::Request::new(
            crate::cri::ImageStatusRequest::default(),
        ))
        .await?;
    Ok(())
}

async fn call_get_container_events(
    runtime_service: &impl RuntimeService,
) -> Result<(), Box<dyn std::error::Error>> {
    runtime_service
        .get_container_events(tonic::Request::new(Default::default()))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i = image_service::FooImageService;
    call_image_status(&i).await?;

    let r = runtime_service::FooRuntimeService;
    call_get_container_events(&r).await?;

    Ok(())
}
