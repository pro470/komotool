use bevy_app::{App, Last, Plugin, PreUpdate, Update};
use bevy_core::NonSendMarker;
use bevy_ecs::reflect::ReflectResource;
use bevy_ecs::schedule::{IntoSystemConfigs, Schedules};
use bevy_ecs::system::{Local, NonSend, Res, ResMut, Resource};
use bevy_reflect::Reflect;
use bevy_state::condition::in_state;
use bevy_time::{Fixed, Time, Timer, TimerMode};
use bevy_utils::{Duration, Instant};
use komotool_utils::GlobalLoadingState;
use komotool_utils::startup_schedule::UpdateStartup;

/// Adds framepacing and framelimiting functionality to your [`App`]
#[derive(Default)]
pub struct KomotoolFramepacePlugin;

impl Plugin for KomotoolFramepacePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FramepaceSettings>()
            .init_resource::<FramepaceSettings>()
            .register_type::<IdleFramePaceState>()
            .init_resource::<IdleFramePaceState>()
            .init_resource::<FrameTimer>()
            .init_resource::<FramePaceStats>()
            .init_resource::<FPS>()
            .add_systems(
                UpdateStartup,
                insert_komotool_framepace_systems.run_if(in_state(GlobalLoadingState::CleanupDone)),
            )
            .add_systems(Update, count_frames);
    }
}

/// Framepacing plugin configuration
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct FramepaceSettings {
    /// Configures the framerate limiting strategy when active.
    pub limiter: Limiter,
    /// Configures the framerate limiting strategy when idle.
    pub idle_limiter: Limiter,
    /// The duration of inactivity before switching to the idle limiter.
    pub idle_threshold: Duration,
    /// Fixed time limiter
    pub fixed_time_limiter: Duration,
    /// Fixed time idle limiter
    pub fixed_time_idle_limiter: Duration,
}

impl Default for FramepaceSettings {
    fn default() -> Self {
        Self {
            limiter: Limiter::Manual(Duration::from_secs_f64(1.0 / 60.0)),
            // Default idle limiter to 4 FPS
            idle_limiter: Limiter::from_framerate(30.0),
            // Default idle threshold to 60 seconds
            idle_threshold: Duration::from_secs(60),
            // Fixed time limiters
            fixed_time_limiter: Duration::from_secs_f64(1.0 / 30.0),
            // Fixed time idle limiters
            fixed_time_idle_limiter: Duration::from_secs_f64(1.0 / 4.0),
        }
    }
}

/// Tracks the last time there was activity to determine idle state for frame pacing.
#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct IdleFramePaceState {
    /// The timestamp of the last registered activity.
    pub last_activity: Instant,
}

impl Default for IdleFramePaceState {
    fn default() -> Self {
        Self {
            last_activity: Instant::now(),
        }
    }
}

/// Configures the framelimiting technique for the app
#[derive(Debug, Clone, Reflect)]
pub enum Limiter {
    /// Set a fixed manual frametime limit
    Manual(Duration),
    /// Disables frame limiting
    Off,
}

impl Default for Limiter {
    fn default() -> Self {
        Limiter::Manual(Duration::from_secs_f64(1.0 / 60.0))
    }
}

impl Limiter {
    /// Returns `true` if the [`Limiter`] is enabled
    pub fn is_enabled(&self) -> bool {
        !matches!(self, Limiter::Off)
    }

    /// Constructs a new [`Limiter`] from the provided `framerate`
    pub fn from_framerate(framerate: f64) -> Self {
        Limiter::Manual(Duration::from_secs_f64(1.0 / framerate))
    }
}

/// Tracks timing information between frames
#[derive(Debug, Default, Resource, Reflect)]
pub struct FrameTimer {
    last_frame: Option<Instant>,
}

/// Sleeps until it's time to start the next frame
pub fn framerate_limiter(
    mut timer: ResMut<FrameTimer>,
    settings: Res<FramepaceSettings>,
    mut stats: ResMut<FramePaceStats>,
    idle_state: Res<IdleFramePaceState>,
    mut fixedtime: ResMut<Time<Fixed>>,
    _main_thread_marker: Option<NonSend<NonSendMarker>>,
) {
    let now = Instant::now();

    if let Some(last_frame) = timer.last_frame {
        let frame_time = now - last_frame;

        // Determine which limiter to use
        let time_since_last_activity = now.duration_since(idle_state.last_activity);
        let is_idle = time_since_last_activity > settings.idle_threshold;

        // Calculate the target timestep based on current state
        let target_timestep = if is_idle {
            settings.fixed_time_idle_limiter
        } else {
            settings.fixed_time_limiter
        };

        // Only update the timestep if it's different from the current one
        if fixedtime.timestep() != target_timestep {
            fixedtime.set_timestep(target_timestep);
        }

        // Select the appropriate limiter for frame pacing
        let active_limiter = if is_idle {
            &settings.idle_limiter
        } else {
            &settings.limiter
        };

        // Apply frame limiting
        if let Limiter::Manual(target_duration) = active_limiter {
            if let Some(sleep_duration) = target_duration.checked_sub(frame_time) {
                spin_sleep::sleep(sleep_duration);
            }
        }

        stats.frametime = frame_time;
        stats.oversleep = now.elapsed().saturating_sub(frame_time);
    }
    timer.last_frame = Some(Instant::now());
}

/// Updates frame timer at start of frame
pub fn update_frame_timer(mut timer: ResMut<FrameTimer>) {
    if timer.last_frame.is_none() {
        timer.last_frame = Some(Instant::now());
    }
}

/// Holds frame time measurements for diagnostics
#[derive(Debug, Default, Resource, Reflect)]
pub struct FramePaceStats {
    pub frametime: Duration,
    pub oversleep: Duration,
}

pub fn insert_komotool_framepace_systems(mut schedules: ResMut<Schedules>) {
    println!("Adding framepace systems");
    schedules.add_systems(Last, framerate_limiter);
    schedules.add_systems(PreUpdate, update_frame_timer);
    println!("Framepace systems added");
}

// Resource to store the current FPS value
#[derive(Resource, Default, Reflect)]
pub struct FPS {
    pub value: u32,
}

// Resource to store our frame count information
#[derive(Default, Reflect)]
struct FrameCountState {
    frames: u32,
    timer: Timer,
}

// System that counts frames and updates FPS resource every second
fn count_frames(time: Res<Time>, mut state: Local<FrameCountState>, mut fps: ResMut<FPS>) {
    // If this is the first run, initialize the timer
    if state.timer.duration().is_zero() {
        state.timer = Timer::from_seconds(1.0, TimerMode::Repeating);
    }

    // Increment the frame counter
    state.frames += 1;

    // Tick the timer and check if a second has passed
    if state.timer.tick(time.delta()).just_finished() {
        // Print the number of frames in the last second
        println!("Frames in the last second: {}", state.frames);

        // Only update the FPS resource if the value is different
        if fps.value != state.frames {
            fps.value = state.frames;
        }

        // Reset the frame counter for the next second
        state.frames = 0;
    }
}
