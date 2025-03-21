pub trait DragonModulePermissions {
    fn all_permissions(&self) -> &'static [&'static str] {
        &[]
    }
}
