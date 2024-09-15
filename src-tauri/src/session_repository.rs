use crate::model::session::SessionDetail;
use rand::seq::SliceRandom;

pub struct SessionRepository {
    sessions: Vec<SessionDetail>,
}

impl SessionRepository {
    pub fn new() -> Self {
        let initial_sessions = vec![
            SessionDetail {
                title: "Rotate your head".to_string(),
                subtitle: "You can perform this mobility exercise either sitting or standing. Relax your shoulders and begin to slowly circle your head. Start with a smaller range of motion. Decide for yourself how far you want to go into the stretch. Very important: Perform the head circles very slowly and consciously, and avoid hasty, thoughtless movements.".to_string(),
                duration_s: 10,
            }
        ];

        SessionRepository {
            sessions: initial_sessions,
        }
    }

    pub fn pick_random_session(&self) -> Option<&SessionDetail> {
        let mut rng = rand::thread_rng();
        self.sessions.choose(&mut rng)
    }
}
