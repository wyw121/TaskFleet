use crate::{models::Company, Database};
use anyhow::Result;

/// CompanyRepository: 负责所有公司相关的数据库操作
pub struct CompanyRepository {
    database: Database,
}

impl CompanyRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// 根据ID查询公司
    pub async fn find_by_id(&self, id: i64) -> Result<Option<Company>> {
        let company = sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.database.pool)
            .await?;
        Ok(company)
    }

    /// 根据公司名查询
    pub async fn find_by_name(&self, name: &str) -> Result<Option<Company>> {
        let company = sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE name = ?")
            .bind(name)
            .fetch_optional(&self.database.pool)
            .await?;
        Ok(company)
    }

    /// 获取所有公司列表
    pub async fn list_all(&self) -> Result<Vec<Company>> {
        let companies = sqlx::query_as::<_, Company>(
            "SELECT * FROM companies ORDER BY created_at DESC"
        )
        .fetch_all(&self.database.pool)
        .await?;
        Ok(companies)
    }

    /// 获取活跃公司列表
    pub async fn list_active(&self) -> Result<Vec<Company>> {
        let companies = sqlx::query_as::<_, Company>(
            "SELECT * FROM companies WHERE is_active = TRUE ORDER BY created_at DESC"
        )
        .fetch_all(&self.database.pool)
        .await?;
        Ok(companies)
    }

    /// 创建新公司
    pub async fn create(&self, company: Company) -> Result<Company> {
        let result = sqlx::query(
            r#"
            INSERT INTO companies (name, contact_email, contact_phone, max_employees, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&company.name)
        .bind(&company.contact_email)
        .bind(&company.contact_phone)
        .bind(company.max_employees)
        .bind(company.is_active)
        .bind(company.created_at)
        .bind(company.updated_at)
        .execute(&self.database.pool)
        .await?;
        
        let company_id = result.last_insert_rowid();
        let created_company = self.find_by_id(company_id).await?
            .ok_or_else(|| anyhow::anyhow!("创建公司后无法查询到"))?;
        Ok(created_company)
    }

    /// 更新公司信息
    pub async fn update(&self, company: Company) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE companies 
            SET name = ?, contact_email = ?, contact_phone = ?, max_employees = ?, 
                is_active = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&company.name)
        .bind(&company.contact_email)
        .bind(&company.contact_phone)
        .bind(company.max_employees)
        .bind(company.is_active)
        .bind(company.updated_at)
        .bind(company.id)
        .execute(&self.database.pool)
        .await?;
        Ok(())
    }

    /// 删除公司
    pub async fn delete(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM companies WHERE id = ?")
            .bind(id)
            .execute(&self.database.pool)
            .await?;
        Ok(())
    }

    /// 获取公司的员工数量
    pub async fn get_employee_count(&self, company_id: i64) -> Result<i32> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM users WHERE company_id = ?"
        )
        .bind(company_id)
        .fetch_one(&self.database.pool)
        .await?;
        Ok(count.0 as i32)
    }
}
