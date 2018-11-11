use model::package::Package;

pub struct Log {
    pub date: String,
    pub time: String,
    pub command: String,
    pub requester: String,
    pub automatic_action: bool,
    pub installed: Vec<Package>,
    pub removed: Vec<Package>,
    pub purged: Vec<Package>,
    pub upgraded: Vec<Package>
}
