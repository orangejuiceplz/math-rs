use super::MathSession;
use crate::math::solver::solve_math_problem;
use dashmap::DashMap;
use serenity::all::{ChannelId, Message, UserId};
use std::sync::Arc;
use tracing::{debug, info, warn};

#[derive(Debug)]
pub struct SessionManager {
    sessions: Arc<DashMap<UserId, MathSession>>,
    session_timeout_minutes: i64,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            session_timeout_minutes: 30, // 30 minute timeout
        }
    }

    pub fn create_session(
        &self,
        user_id: UserId,
        channel_id: ChannelId,
    ) -> String {
        // checks if user already has an active session
        if let Some(existing_session) = self.sessions.get(&user_id) {
            if !existing_session.is_expired(self.session_timeout_minutes) {
                return format!(
                    "You already have an active math session! Send me a math problem to solve."
                );
            } else {
                // delete expired session
                self.sessions.remove(&user_id);
            }
        }

        let session = MathSession::new(user_id, channel_id);
        let session_id = session.id;
        
        self.sessions.insert(user_id, session);
        
        info!("Created new math session {} for user {}", session_id, user_id);
        
        format!(
            " **math session started!** \n\n\
            bot to help you solve math problems step by step!\n\n\
            **Current features:**\n\
            • basic arithmetic (2 + 3 * 4)\n\
            • algebraic equations (solve 2x + 5 = 11)\n\
            • calculus (derivative of x^2)\n\
            • more to come\n\n\
            type your math problem and i'll solve it for you! \n\n\
            *session will expire after {} minutes of inactivity.*",
            self.session_timeout_minutes
        )
    }

    pub async fn handle_message(&self, msg: &Message) -> Option<String> {
        let user_id = msg.author.id;
        
        // checks if user has an active session
        if let Some(mut session) = self.sessions.get_mut(&user_id) {
            if session.is_expired(self.session_timeout_minutes) {
                // delete expired session
                drop(session);
                self.sessions.remove(&user_id);
                return Some("Your math session has expired. Use `/mathbot start session` to start a new one.".to_string());
            }

            // add message to conversation history
            session.add_message(msg.content.clone());
            debug!("Processing math problem: {}", msg.content);

            // process the math problem
            match solve_math_problem(&msg.content).await {
                Ok(solution) => Some(solution),
                Err(err) => {
                    warn!("Error solving math problem: {}", err);
                    Some(format!(
                        "❌ **Error solving math problem:**\n```\n{}\n```\n\n\
                        Please check your input and try again. Make sure your math expression is properly formatted!",
                        err
                    ))
                }
            }
        } else {
            // no active session
            None
        }
    }

    pub fn end_session(&self, user_id: UserId) -> String {
        if let Some((_, session)) = self.sessions.remove(&user_id) {
            info!("ended math session {} for user {}", session.id, user_id);
            "session ended".to_string()
        } else {
            "you don't have an active math session to end.".to_string()
        }
    }

    pub fn get_session_info(&self, user_id: UserId) -> String {
        if let Some(session) = self.sessions.get(&user_id) {
            if session.is_expired(self.session_timeout_minutes) {
                // Remove expired session
                drop(session);
                self.sessions.remove(&user_id);
                return "Your session has expired. Use `/mathbot start session` to start a new one.".to_string();
            }

            format!(
                "**Session Info:**\n\
                • session ID: `{}`\n\
                • created: {}\n\
                • last activity: {}\n\
                • messages: {}\n\
                • tatus: active ✅",
                session.id,
                session.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                session.last_activity.format("%Y-%m-%d %H:%M:%S UTC"),
                session.conversation_history.len()
            )
        } else {
            "you don't have an active math session. use `/mathbot start session` to create one.".to_string()
        }
    }

    // cleanup expired sessions (could be called periodically)
    pub fn cleanup_expired_sessions(&self) -> usize {
        let mut removed_count = 0;
        
        // collect user IDs with expired sessions
        let expired_users: Vec<UserId> = self
            .sessions
            .iter()
            .filter_map(|entry| {
                let (user_id, session) = entry.pair();
                if session.is_expired(self.session_timeout_minutes) {
                    Some(*user_id)
                } else {
                    None
                }
            })
            .collect();

        // remove expired sessions
        for user_id in expired_users {
            if self.sessions.remove(&user_id).is_some() {
                removed_count += 1;
            }
        }

        if removed_count > 0 {
            info!("Cleaned up {} expired math sessions", removed_count);
        }

        removed_count
    }

    pub fn get_active_session_count(&self) -> usize {
        self.cleanup_expired_sessions();
        self.sessions.len()
    }
}