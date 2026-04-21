//! Admin dashboard pages — protected area for managing the store.

mod dashboard;
mod products;
mod users;

pub use dashboard::AdminDashboard;
pub use products::AdminProducts;
pub use users::AdminUsers;
