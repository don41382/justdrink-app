use crate::model::session::{Exercise, SessionId};
use rand::seq::IteratorRandom;
use std::collections::HashSet;

pub struct SessionRepository {
    exercises: Vec<Exercise>,
    picked_sessions: HashSet<usize>, // Track indices of picked sessions
}

impl SessionRepository {
    // Initialize the repository with predefined sessions and an empty set of picked sessions
    pub fn new() -> Self {
        let initial_sessions = vec![
            Exercise {
                id: SessionId("rotate-your-head".to_string()),
                title: "Rotate your head".to_string(),
                description: "Slowly rotate your head to release tension and improve neck mobility.".to_string(),
                advices: vec![
                    "You can perform this mobility exercise either sitting or standing.",
                    "Relax your shoulders and begin to slowly circle your head.",
                    "Start with a smaller range of motion. Decide for yourself how far you want to go into the stretch.",
                    "Important: Perform the head circles very slowly and consciously, and avoid hasty, thoughtless movements."]
                    .iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
            Exercise {
                id: SessionId("shoulder-circle".to_string()),
                title: "Shoulder Circles".to_string(),
                description: "A gentle shoulder mobility exercise that relieves tension and promotes relaxation.".to_string(),
                advices: vec![
                    "Begin with small, slow shoulder circles to ease into the movement.",
                    "Gradually increase the range of motion, making the circles larger.",
                    "Lift your shoulders up towards your ears during the movement for added stretch.",
                    "Breathe deeply and evenly to maximize relaxation and tension relief.",
                    "Change your rotation direction to balance your movement.",
                ]
                    .iter().map(|s| s.to_string()).collect(),
                duration_s: 40,
                active: true,
            },
            Exercise {
                id: SessionId("pelvic-tilt".to_string()),
                title: "Pelvic Tilt".to_string(),
                description: "A seated exercise to improve hip flexibility and core engagement through controlled pelvic movements.".to_string(),
                advices: vec![
                    "Sit comfortably on a chair with your upper body upright and maintain a natural curve in your lower back.",
                    "Start with a neutral spine, keeping a slight arch in your lumbar region (lower back).",
                    "Engage your lower back by squeezing your glutes while simultaneously pulling your belly button inward.",
                    "Notice the tension in your core as the arch in your lower back flattens slightly.",
                    "Perform this movement in a steady, fluid rhythm, paying attention to your breathing."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 20,
                active: true,
            },
            Exercise {
                id: SessionId("chair-squat".to_string()),
                title: "Chair Squat".to_string(),
                description: "An effective bodyweight exercise to strengthen triceps, shoulders, and chest using a sturdy chair or bench.".to_string(),
                advices: vec![
                    "Lower your body by bending your elbows to a 90-degree angle while inhaling.",
                    "Push back up to the starting position, fully extending your arms while exhaling.",
                    "Maintain relaxed shoulders and a neutral neck position to avoid strain.",
                    "Perform slow, controlled movements for better muscle engagement and injury prevention.",
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
            Exercise {
                id: SessionId("chin-tuck".to_string()),
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
                duration_s: 20,
                active: true,
            },
            Exercise {
                id: SessionId("w-and-y".to_string()),
                title: "W and Y".to_string(),
                description: "A postural strengthening exercise to improve shoulder stability and upper back mobility.".to_string(),
                advices: vec![
                    "Start in a standing or prone position with your back straight and core engaged.",
                    "For the W movement: Bend your elbows to 90 degrees and bring your arms back to form a 'W' shape, squeezing your shoulder blades together.",
                    "Hold the W position for 2-3 seconds before slowly returning to the starting position.",
                    "For the Y movement: Extend your arms straight overhead into a 'Y' shape, keeping your elbows slightly bent.",
                    "Lift your arms in the Y position with control, focusing on engaging your upper back muscles.",
                    "Avoid shrugging your shoulders or arching your back during the exercise.",
                    "Perform the movements slowly and consciously, ensuring proper form and breathing."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 40,
                active: true,
            },
            Exercise {
                id: SessionId("standing-calf-raise".to_string()),
                title: "Standing Calf Raise".to_string(),
                description: "A simple exercise to strengthen the calf muscles and improve balance.".to_string(),
                advices: vec![
                    "Stand tall with your feet shoulder-width apart, keeping your posture upright.",
                    "Place your hands on your hips or a sturdy surface for balance, if needed.",
                    "Slowly lift your heels off the ground, rising onto the balls of your feet.",
                    "Pause at the top of the movement for 1-2 seconds, feeling the contraction in your calves.",
                    "Lower your heels back to the ground in a controlled manner.",
                    "Focus on engaging your calf muscles and maintaining smooth, steady movements.",
                    "Avoid locking your knees or rocking your body during the exercise."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
            Exercise {
                id: SessionId("seated-knee-extension".to_string()),
                title: "Seated Knee Extension".to_string(),
                description: "An exercise to strengthen the quadriceps and improve knee mobility.".to_string(),
                advices: vec![
                    "Sit on a sturdy chair with your back straight and feet flat on the ground.",
                    "Keep your knees bent at a 90-degree angle and your hands resting on your thighs or the sides of the chair.",
                    "Slowly straighten one leg, lifting your foot until your knee is fully extended.",
                    "Hold the extended position for 2-3 seconds, focusing on squeezing your quadriceps.",
                    "Lower your foot back to the starting position in a controlled manner.",
                    "Repeat on the other leg, alternating legs for the desired number of repetitions.",
                    "Avoid locking your knee or using jerky movements during the exercise."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
            Exercise {
                id: SessionId("seated-glute-stretch-pencil".to_string()),
                title: "Seated Glute Stretch".to_string(),
                description: "A seated stretch to target the glutes and improve hip flexibility.".to_string(),
                advices: vec![
                    "Sit on a sturdy chair with your back straight and feet flat on the floor.",
                    "Cross one ankle over the opposite knee, creating a figure-four shape with your legs.",
                    "Gently press down on the raised knee to deepen the stretch, keeping your back straight.",
                    "Lean slightly forward at the hips, maintaining a neutral spine to increase the intensity of the stretch.",
                    "Hold the stretch for 20-30 seconds, focusing on breathing deeply and relaxing your muscles.",
                    "Switch sides and repeat with the other leg.",
                    "Avoid forcing the stretch or rounding your back during the movement."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 40,
                active: true,
            },
            // revisit
            Exercise {
                id: SessionId("left-down-right-head".to_string()),
                title: "Left Down Right Head Exercise (Seated)".to_string(),
                description: "A seated neck stretch to relieve tension and improve flexibility in the neck and shoulders.".to_string(),
                advices: vec![
                    "Sit on a sturdy chair with your back straight and shoulders relaxed.",
                    "Slowly tilt your head toward your left shoulder, aiming to bring your left ear closer to your left shoulder.",
                    "Pause briefly, then gently roll your head down toward your chest, feeling the stretch in the back of your neck.",
                    "Continue the motion by rolling your head toward your right shoulder, bringing your right ear closer to your right shoulder.",
                    "Move slowly and smoothly in a semicircular pattern, avoiding any jerky or forced movements.",
                    "Repeat the motion for the desired number of repetitions, maintaining deep and steady breaths.",
                    "Avoid shrugging your shoulders or tilting your torso during the exercise."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
            Exercise {
                id: SessionId("head-bend-to-the-side".to_string()),
                title: "Head Bend to the Side".to_string(),
                description: "A seated stretch to relieve neck tension and improve lateral neck flexibility.".to_string(),
                advices: vec![
                    "Sit on a sturdy chair with your back straight and shoulders relaxed.",
                    "Gently tilt your head toward your left shoulder, aiming to bring your left ear closer to your left shoulder.",
                    "Keep your shoulders level and avoid lifting the opposite shoulder during the stretch.",
                    "For a deeper stretch, you can place your left hand gently on the side of your head, applying light pressure.",
                    "Hold the stretch for 15-20 seconds, breathing deeply and relaxing your muscles.",
                    "Return your head to the neutral position and repeat on the other side.",
                    "Avoid rotating your head or straining during the movement; focus on a gentle and controlled stretch."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 40,
                active: true,
            },
            Exercise {
                id: SessionId("elbow-to-knee".to_string()),
                title: "Elbow to Knee".to_string(),
                description: "A dynamic seated exercise to engage the core and improve coordination.".to_string(),
                advices: vec![
                    "Sit on a sturdy chair with your back straight and feet flat on the floor.",
                    "Place your hands behind your head, keeping your elbows wide.",
                    "Lift your left knee toward your chest while simultaneously twisting your torso to bring your right elbow toward the lifted knee.",
                    "Focus on engaging your core muscles as you perform the twisting motion.",
                    "Return to the starting position in a controlled manner and repeat on the other side.",
                    "Alternate sides for the desired number of repetitions, maintaining steady and controlled movements.",
                    "Avoid slouching or using momentum; keep the movement deliberate and focus on muscle engagement."
                ].iter().map(|s| s.to_string()).collect(),
                duration_s: 30,
                active: true,
            },
        ];

        SessionRepository {
            exercises: initial_sessions,
            picked_sessions: HashSet::new(), // Initialize with an empty set
        }
    }

    // This method ensures that all sessions are picked once before repeating
    pub fn pick_random_session(&mut self) -> Option<&Exercise> {
        // If all sessions have been picked, reset the picked list
        if self.picked_sessions.len() == self.exercises.len() {
            self.picked_sessions.clear();
        }

        // Filter active sessions that haven't been picked
        let mut rng = rand::thread_rng();
        let available_sessions: Vec<(usize, &Exercise)> = self
            .exercises
            .iter()
            .enumerate()
            .filter(|(i, s)| s.active && !self.picked_sessions.contains(i))
            .collect();

        // Randomly choose one of the available sessions
        if let Some((index, session)) = available_sessions.into_iter().choose(&mut rng) {
            // Add the index of the picked session to the picked_sessions set
            self.picked_sessions.insert(index);
            Some(session)
        } else {
            None
        }
    }
}
