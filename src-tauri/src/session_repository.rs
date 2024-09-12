use crate::model::session::SessionDetail;
use rand::seq::SliceRandom;

pub struct SessionRepository {
    sessions: Vec<SessionDetail>,
}

impl SessionRepository {
    pub fn new() -> Self {
        let initial_sessions = vec![
            SessionDetail {
                title: "Morgenyoga".to_string(),
                subtitle: "Sanfter Start in den Tag".to_string(),
                duration_s: 30,
            },
            SessionDetail {
                title: "HIIT-Training".to_string(),
                subtitle: "Hochintensives Intervalltraining".to_string(),
                duration_s: 60,
            },
            SessionDetail {
                title: "Meditation".to_string(),
                subtitle: "Achtsamkeit und innere Ruhe".to_string(),
                duration_s: 120,
            },
            SessionDetail {
                title: "Pilates".to_string(),
                subtitle: "Körperkontrolle und Flexibilität".to_string(),
                duration_s: 90,
            },
            SessionDetail {
                title: "Krafttraining".to_string(),
                subtitle: "Muskelaufbau und Stärkung".to_string(),
                duration_s: 10,
            },
            SessionDetail {
                title: "Lauftraining".to_string(),
                subtitle: "Ausdauer und Cardio".to_string(),
                duration_s: 40,
            },
            SessionDetail {
                title: "Stretching".to_string(),
                subtitle: "Dehnen und Entspannen".to_string(),
                duration_s: 10,
            },
            SessionDetail {
                title: "Rückenfit".to_string(),
                subtitle: "Stärkung der Rückenmuskulatur".to_string(),
                duration_s: 50,
            },
            SessionDetail {
                title: "Zumba".to_string(),
                subtitle: "Tanzen und Fitness".to_string(),
                duration_s: 10,
            },
            SessionDetail {
                title: "Entspannungsübungen".to_string(),
                subtitle: "Progressive Muskelentspannung".to_string(),
                duration_s: 45,
            },
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