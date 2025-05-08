struct WorkplaceCatalogPath(String);

trait SetName {
    fn set_path(self, path_name: Option<Self>) -> Self;
}

impl SetName for WorkplaceCatalogPath {
    fn set_path(self, path_name: Option<Self>) -> Self {
        path_name.unwrap_or(Self(".".to_string())) // Use provided path or default "."
    }
}
