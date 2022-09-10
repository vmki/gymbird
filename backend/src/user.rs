pub type UUID = String;

pub struct UserProfile {
    name: String,
    username: String,
    user_id: UUID,
}

pub struct UserAccount {
    email: String,
    password: String,
    user_id: UUID,
}

pub struct User {
    user_profile: UserProfile,
    user_account: UserAccount,
    id: UUID,
}
