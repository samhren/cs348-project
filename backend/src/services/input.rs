use crate::errors::AppError;

const MAX_EMAIL_LENGTH: usize = 254;
const MAX_SEARCH_LENGTH: usize = 64;
const MAX_USERNAME_LENGTH: usize = 32;
const MIN_PASSWORD_LENGTH: usize = 8;
const MIN_USERNAME_LENGTH: usize = 3;

pub fn validate_username(username: &str) -> Result<String, AppError> {
    let username = username.trim();

    if username.len() < MIN_USERNAME_LENGTH || username.len() > MAX_USERNAME_LENGTH {
        return Err(AppError::BadRequest(
            "username must be between 3 and 32 characters".to_string(),
        ));
    }

    if !username
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
    {
        return Err(AppError::BadRequest(
            "username may only contain letters, numbers, underscores, and hyphens".to_string(),
        ));
    }

    Ok(username.to_string())
}

pub fn validate_email(email: &str) -> Result<String, AppError> {
    let email = email.trim().to_ascii_lowercase();

    if email.is_empty() || email.len() > MAX_EMAIL_LENGTH {
        return Err(AppError::BadRequest("email is invalid".to_string()));
    }

    if email.chars().any(|character| character.is_whitespace()) {
        return Err(AppError::BadRequest("email is invalid".to_string()));
    }

    let Some((local_part, domain_part)) = email.split_once('@') else {
        return Err(AppError::BadRequest("email is invalid".to_string()));
    };

    if local_part.is_empty()
        || domain_part.is_empty()
        || !domain_part.contains('.')
        || domain_part.starts_with('.')
        || domain_part.ends_with('.')
    {
        return Err(AppError::BadRequest("email is invalid".to_string()));
    }

    Ok(email)
}

pub fn validate_password(password: &str) -> Result<&str, AppError> {
    if password.len() < MIN_PASSWORD_LENGTH || password.len() > 128 {
        return Err(AppError::BadRequest(
            "password must be between 8 and 128 characters".to_string(),
        ));
    }

    if password.chars().any(|character| character.is_control()) {
        return Err(AppError::BadRequest(
            "password contains invalid characters".to_string(),
        ));
    }

    Ok(password)
}

pub fn validate_role(role: &str) -> Result<String, AppError> {
    let role = role.trim();

    match role {
        "admin" | "user" => Ok(role.to_string()),
        _ => Err(AppError::BadRequest("role is invalid".to_string())),
    }
}

pub fn validate_optional_role(role: &Option<String>) -> Result<Option<String>, AppError> {
    role.as_deref()
        .map(str::trim)
        .filter(|role| !role.is_empty())
        .map(validate_role)
        .transpose()
}

pub fn sanitize_optional_search_term(
    search_term: &Option<String>,
) -> Result<Option<String>, AppError> {
    let Some(search_term) = search_term.as_deref().map(str::trim) else {
        return Ok(None);
    };

    if search_term.is_empty() {
        return Ok(None);
    }

    if search_term.len() > MAX_SEARCH_LENGTH
        || search_term.chars().any(|character| character.is_control())
    {
        return Err(AppError::BadRequest("search term is invalid".to_string()));
    }

    Ok(Some(search_term.to_string()))
}
