use crate::model::session::SessionDetail;
use rand::seq::IteratorRandom;

pub struct SessionRepository {
    sessions: Vec<SessionDetail>,
}

impl SessionRepository {
    pub fn new() -> Self {
        let initial_sessions = vec![
            SessionDetail {
                id: "rotate-your-head".to_string(),
                title: "Rotate your head".to_string(),
                description: "Slowly rotate your head".to_string(),
                advices: vec![
                    "You can perform this mobility exercise either sitting or standing.",
                    "Relax your shoulders and begin to slowly circle your head.",
                    "Start with a smaller range of motion. Decide for yourself how far you want to go into the stretch.",
                    "Very important: Perform the head circles very slowly and consciously, and avoid hasty, thoughtless movements."]
                    .iter().map(|s| s.to_string()).collect()
                ,
                duration_s: 60,
                active: false,
            },
            SessionDetail {
                id: "shoulder-circles".to_string(),
                title: "Shoulder Circles".to_string(),
                description: "A gentle shoulder mobility exercise that relieves tension and promotes relaxation.".to_string(),
                advices: vec![
                    "Begin with small, slow shoulder circles to ease into the movement.",
                    "Gradually increase the range of motion, making the circles larger.",
                    "Lift your shoulders up towards your ears during the movement for added stretch.",
                    "Breathe deeply and evenly to maximize relaxation and tension relief."]
                    .iter().map(|s| s.to_string()).collect()
                ,
                duration_s: 60,
                active: true,
            },
            SessionDetail {
                id: "seated-hip-mobility".to_string(),
                title: "Seated Hip Mobility".to_string(),
                description: "A seated exercise to improve hip flexibility and core engagement through controlled pelvic movements.".to_string(),
                advices: vec![
                    "Sit comfortably on a chair with your upper body upright and maintain a natural curve in your lower back.",
                    "Start with a neutral spine, keeping a slight arch in your lumbar region (lower back).",
                    "Engage your lower back by squeezing your glutes while simultaneously pulling your belly button inward.",
                    "Notice the tension in your core as the arch in your lower back flattens slightly.",
                    "Perform this movement in a steady, fluid rhythm, paying attention to your breathing."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 60,
                active: false,
            },
            SessionDetail {
                id: "chair-squat".to_string(),
                title: "Chair Squat".to_string(),
                description: "A effective bodyweight exercise to strengthen triceps, shoulders, and chest using a sturdy chair or bench.".to_string(),
                advices: vec![
                    "Lower your body by bending your elbows to a 90-degree angle while inhaling.",
                    "Push back up to the starting position, fully extending your arms while exhaling.",
                    "Maintain relaxed shoulders and a neutral neck position to avoid strain.",
                    "Perform slow, controlled movements for better muscle engagement and injury prevention.",
                ].iter().map(|s| s.to_string()).collect()
                ,
                duration_s: 60,
                active: false,
            },
            SessionDetail {
                id: "chin-tucks".to_string(),
                title: "Chin Tucks".to_string(),
                description: "A simple exercise to improve neck muscle endurance, relieve tension, and promote better posture.".to_string(),
                advices: vec![
                    "Stand in a comfortable, upright position with your shoulders relaxed",
                    "Keep your head straight, look forward.",
                    "Retract your chin backward without tilting your head, as if making a double chin.",
                    "Focus on relaxing your jaw muscles as you hold this position.",
                    "Slowly return to the starting position, allowing your neck muscles to relax.",
                    "You should feel a gentle stretch at the back of your neck and slight muscle engagement at the front.",
                    "This exercise may help relieve tension headaches, improve sitting posture, and reduce neck pain."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 60,
                active: false,
            }
        ];

        SessionRepository {
            sessions: initial_sessions,
        }
    }

    pub fn pick_random_session(&self) -> Option<&SessionDetail> {
        let mut rng = rand::thread_rng();
        self.sessions.iter().filter(|s| s.active).choose(&mut rng)
    }
}