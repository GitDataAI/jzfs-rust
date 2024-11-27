use uuid::Uuid;

pub struct BranchModel{
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub name: String,
    pub head: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    // TODO branch object
}