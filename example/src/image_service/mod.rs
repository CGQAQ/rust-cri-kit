impl crate::cri::image_service_server::ImageService for FooImageService {}
pub struct FooImageService;

impl FooImageService {
    pub async fn image_status(
        &self,
        _request: tonic::Request<crate::cri::ImageStatusRequest>,
    ) -> Result<tonic::Response<crate::cri::ImageStatusResponse>, tonic::Status> {
        println!("FooImageService#image_status: It works, yay!!!!!!!!!!!!!!!");

        Ok(tonic::Response::new(
            crate::cri::ImageStatusResponse::default(),
        ))
    }
}
