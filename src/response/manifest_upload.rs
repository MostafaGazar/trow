use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Header;

#[derive(Debug, Serialize)]
pub struct ManifestUpload {
    pub location: String,
    pub digest: String,
}

impl<'r> Responder<'r> for ManifestUpload {
    fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {

        let location = Header::new("Location", self.location);
        let digest = Header::new("Docker-Content-Digest", self.digest);
        Response::build()
            .status(Status::Created)
            .header(location)
            .header(digest)
            .ok()
    }
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use response::manifest_upload::ManifestUpload;
    use response::test_helper::test_route;

    #[test]
    fn accepted_ok() {
        
        let response = test_route(ManifestUpload{location: "location".to_owned(), digest: "digest".to_owned()});
        assert_eq!(response.status(), Status::Created);
    }
}
