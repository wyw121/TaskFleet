use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};

/// 对密码进行哈希加密
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| anyhow!("密码加密失败: {}", e))
}

/// 验证密码是否匹配
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(|e| anyhow!("密码验证失败: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "test_password_123";
        let hashed = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }
}
