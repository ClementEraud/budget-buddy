pub mod home;
pub mod login;

#[derive(Debug, PartialEq)]
pub enum View {
    Login,
    Home,
}
