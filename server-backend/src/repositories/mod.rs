// Repository 层：负责所有数据库操作
// Service 层通过 Repository 访问数据，而不是直接操作 Database

pub mod user_repository;
// TaskFleet核心仓库（待添加）:
// pub mod task_repository;
// pub mod project_repository;

pub use user_repository::UserRepository;
