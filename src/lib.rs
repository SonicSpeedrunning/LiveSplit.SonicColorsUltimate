#![no_std]
#![feature(type_alias_impl_trait, const_async_blocks)]
#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::undocumented_unsafe_blocks,
    rust_2018_idioms
)]

use asr::{
    file_format::pe,
    future::{next_tick, retry},
    settings::Gui,
    signature::Signature,
    time::Duration,
    timer::{self, TimerState},
    watcher::Watcher,
    Address, Address64, Process,
};

asr::panic_handler!();
asr::async_main!(nightly);

const PROCESS_NAMES: &[&str] = &["SonicColorsUltimate.exe", "Sonic Colors - Ultimate.exe"];

async fn main() {
    let mut settings = Settings::register();

    loop {
        // Hook to the target process
        let process = retry(|| PROCESS_NAMES.iter().find_map(|&name| Process::attach(name))).await;

        process
            .until_closes(async {
                // Once the target has been found and attached to, set up some default watchers
                let mut watchers = Watchers::default();

                // Perform memory scanning to look for the addresses we need
                let addresses = Addresses::init(&process).await;

                loop {
                    // Splitting logic. Adapted from OG LiveSplit:
                    // Order of execution
                    // 1. update() will always be run first. There are no conditions on the execution of this action.
                    // 2. If the timer is currently either running or paused, then the isLoading, gameTime, and reset actions will be run.
                    // 3. If reset does not return true, then the split action will be run.
                    // 4. If the timer is currently not running (and not paused), then the start action will be run.
                    settings.update();
                    update_loop(&process, &addresses, &mut watchers);

                    let timer_state = timer::state();
                    if timer_state == TimerState::Running || timer_state == TimerState::Paused {
                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }

                        if let Some(game_time) = game_time(&watchers, &settings) {
                            timer::set_game_time(game_time)
                        }

                        if reset(&watchers, &settings) {
                            timer::reset()
                        } else if split(&watchers, &settings) {
                            timer::split()
                        }
                    }

                    if timer::state() == TimerState::NotRunning && start(&watchers, &settings) {
                        timer::start();
                        timer::pause_game_time();

                        if let Some(is_loading) = is_loading(&watchers, &settings) {
                            if is_loading {
                                timer::pause_game_time()
                            } else {
                                timer::resume_game_time()
                            }
                        }
                    }

                    next_tick().await;
                }
            })
            .await;
    }
}

#[derive(Default)]
struct Watchers {
    levelid: Watcher<Levels>,
    igt: Watcher<Duration>,
    goalringreached: Watcher<bool>,
    eggshuttle_totalstages: Watcher<u8>,
    eggshuttle_progressiveid: Watcher<u8>,
    runstart: Watcher<u8>,
    tr1rank: Watcher<i8>,
    accumulatedigt: Duration,
    currentgamemode: GameMode,
}

struct Addresses {
    base_address: Address,
}

impl Addresses {
    async fn init(process: &Process) -> Self {
        let main_module = {
            let main_module_base = retry(|| {
                PROCESS_NAMES
                    .iter()
                    .find_map(|&p| process.get_module_address(p).ok())
            })
            .await;
            let main_module_size =
                retry(|| pe::read_size_of_image(process, main_module_base)).await;
            (main_module_base, main_module_size as u64)
        };

        const SIG: Signature<5> = Signature::new("76 0C 48 8B 0D");
        let mut ptr = retry(|| SIG.scan_process_range(process, main_module)).await + 5;
        ptr = ptr + 0x4 + retry(|| process.read::<i32>(ptr)).await;

        Self { base_address: ptr }
    }
}

#[derive(Gui)]
struct Settings {
    #[default = true]
    /// START: Auto start (Any%)
    start_anypercent: bool,
    #[default = true]
    /// START: Auto start (Sonic Simulator)
    start_sonic_simulator: bool,
    #[default = true]
    /// START: Auto start (Egg Shuttle)
    start_egg_shuttle: bool,
    #[default = true]
    /// RESET: Auto reset (Any%)
    reset_anypercent: bool,
    #[default = true]
    /// RESET: Auto reset (Egg Shuttle)
    reset_egg_shuttle: bool,
    #[default = true]
    /// Tropical Resort - Act 1
    tropical_resort_1: bool,
    #[default = true]
    /// Tropical Resort - Act 2
    tropical_resort_2: bool,
    #[default = true]
    /// Tropical Resort - Act 3
    tropical_resort_3: bool,
    #[default = true]
    /// Tropical Resort - Act 4
    tropical_resort_4: bool,
    #[default = true]
    /// Tropical Resort - Act 5
    tropical_resort_5: bool,
    #[default = true]
    /// Tropical Resort - Act 6
    tropical_resort_6: bool,
    #[default = true]
    /// Tropical Resort - BOSS
    tropical_resort_boss: bool,
    #[default = true]
    /// Sweet Mountain - Act 1
    sweet_mountain_1: bool,
    #[default = true]
    /// Sweet Mountain - Act 2
    sweet_mountain_2: bool,
    #[default = true]
    /// Sweet Mountain - Act 3
    sweet_mountain_3: bool,
    #[default = true]
    /// Sweet Mountain - Act 4
    sweet_mountain_4: bool,
    #[default = true]
    /// Sweet Mountain - Act 5
    sweet_mountain_5: bool,
    #[default = true]
    /// Sweet Mountain - Act 6
    sweet_mountain_6: bool,
    #[default = true]
    /// Sweet Mountain - BOSS
    sweet_mountain_boss: bool,
    #[default = true]
    /// Startlight Carnival - Act 1
    starlight_carnival_1: bool,
    #[default = true]
    /// Startlight Carnival - Act 2
    starlight_carnival_2: bool,
    #[default = true]
    /// Startlight Carnival - Act 3
    starlight_carnival_3: bool,
    #[default = true]
    /// Startlight Carnival - Act 4
    starlight_carnival_4: bool,
    #[default = true]
    /// Startlight Carnival - Act 5
    starlight_carnival_5: bool,
    #[default = true]
    /// Startlight Carnival - Act 6
    starlight_carnival_6: bool,
    #[default = true]
    /// Startlight Carnival - BOSS
    starlight_carnival_boss: bool,
    #[default = true]
    /// Planet Wisp - Act 1
    planet_wisp_1: bool,
    #[default = true]
    /// Planet Wisp - Act 2
    planet_wisp_2: bool,
    #[default = true]
    /// Planet Wisp - Act 3
    planet_wisp_3: bool,
    #[default = true]
    /// Planet Wisp - Act 4
    planet_wisp_4: bool,
    #[default = true]
    /// Planet Wisp - Act 5
    planet_wisp_5: bool,
    #[default = true]
    /// Planet Wisp - Act 6
    planet_wisp_6: bool,
    #[default = true]
    /// Planet Wisp - BOSS
    planet_wisp_boss: bool,
    #[default = true]
    /// Aquarium Park - Act 1
    aquarium_park_1: bool,
    #[default = true]
    /// Aquarium Park - Act 2
    aquarium_park_2: bool,
    #[default = true]
    /// Aquarium Park - Act 3
    aquarium_park_3: bool,
    #[default = true]
    /// Aquarium Park - Act 4
    aquarium_park_4: bool,
    #[default = true]
    /// Aquarium Park - Act 5
    aquarium_park_5: bool,
    #[default = true]
    /// Aquarium Park - Act 6
    aquarium_park_6: bool,
    #[default = true]
    /// Aquarium Park - BOSS
    aquarium_park_boss: bool,
    #[default = true]
    /// Asteroid Coaster - Act 1
    asteroid_coaster_1: bool,
    #[default = true]
    /// Asteroid Coaster - Act 2
    asteroid_coaster_2: bool,
    #[default = true]
    /// Asteroid Coaster - Act 3
    asteroid_coaster_3: bool,
    #[default = true]
    /// Asteroid Coaster - Act 4
    asteroid_coaster_4: bool,
    #[default = true]
    /// Asteroid Coaster - Act 5
    asteroid_coaster_5: bool,
    #[default = true]
    /// Asteroid Coaster - Act 6
    asteroid_coaster_6: bool,
    #[default = true]
    /// Asteroid Coaster - BOSS
    asteroid_coaster_boss: bool,
    #[default = true]
    /// Terminal Velocity - Act 1
    terminal_velocity_1: bool,
    #[default = true]
    /// Terminal Velocity - BOSS
    terminal_velocity_boss: bool,
    #[default = true]
    /// Terminal Velocity - Act 2
    terminal_velocity_2: bool,
    #[default = true]
    /// Sonic Simulator: 1 - 1
    sonic_simulator_1_1: bool,
    #[default = true]
    /// Sonic Simulator: 1 - 2
    sonic_simulator_1_2: bool,
    #[default = true]
    /// Sonic Simulator: 1 - 3
    sonic_simulator_1_3: bool,
    #[default = true]
    /// Sonic Simulator: 2 - 1
    sonic_simulator_2_1: bool,
    #[default = true]
    /// Sonic Simulator: 2 - 2
    sonic_simulator_2_2: bool,
    #[default = true]
    /// Sonic Simulator: 2 - 3
    sonic_simulator_2_3: bool,
    #[default = true]
    /// Sonic Simulator: 3 - 1
    sonic_simulator_3_1: bool,
    #[default = true]
    /// Sonic Simulator: 3 - 2
    sonic_simulator_3_2: bool,
    #[default = true]
    /// Sonic Simulator: 3 - 3
    sonic_simulator_3_3: bool,
    #[default = true]
    /// Sonic Simulator: 4 - 1
    sonic_simulator_4_1: bool,
    #[default = true]
    /// Sonic Simulator: 4 - 2
    sonic_simulator_4_2: bool,
    #[default = true]
    /// Sonic Simulator: 4 - 3
    sonic_simulator_4_3: bool,
    #[default = true]
    /// Sonic Simulator: 5 - 1
    sonic_simulator_5_1: bool,
    #[default = true]
    /// Sonic Simulator: 5 - 2
    sonic_simulator_5_2: bool,
    #[default = true]
    /// Sonic Simulator: 5 - 3
    sonic_simulator_5_3: bool,
    #[default = true]
    /// Sonic Simulator: 6 - 1
    sonic_simulator_6_1: bool,
    #[default = true]
    /// Sonic Simulator: 6 - 2
    sonic_simulator_6_2: bool,
    #[default = true]
    /// Sonic Simulator: 6 - 3
    sonic_simulator_6_3: bool,
    #[default = true]
    /// Sonic Simulator: 7 - 1
    sonic_simulator_7_1: bool,
    #[default = true]
    /// Sonic Simulator: 7 - 2
    sonic_simulator_7_2: bool,
    #[default = true]
    /// Sonic Simulator: 7 - 3
    sonic_simulator_7_3: bool,
}

fn update_loop(proc: &Process, addresses: &Addresses, watchers: &mut Watchers) {
    let mut level: Levels = Levels::None;
    let mut igt = Duration::ZERO;
    let mut goal_ring = false;
    let mut eggshuttle_progressiveid: u8 = 0;
    let mut eggshuttle_totalstages: u8 = 0;
    let mut runstart: u8 = 0;
    let mut tr1rank: i8 = 0;

    if let Ok(addr_base) = proc.read::<Address64>(addresses.base_address) {
        if let Ok(addr_1) = proc.read::<Address64>(addr_base + 0x8) {
            if let Ok(addr_2) = proc.read::<Address64>(addr_1 + 0x38) {
                if let Ok(addr_3) = proc.read::<Address64>(addr_2 + 0x60) {
                    let level_id = proc.read::<[u8; 6]>(addr_3 + 0xE0);
                    level = match &level_id {
                        Ok(x) => match x {
                            b"stg110" => Levels::TropicalResortAct1,
                            b"stg130" => Levels::TropicalResortAct2,
                            b"stg120" => Levels::TropicalResortAct3,
                            b"stg140" => Levels::TropicalResortAct4,
                            b"stg150" => Levels::TropicalResortAct5,
                            b"stg160" => Levels::TropicalResortAct6,
                            b"stg190" => Levels::TropicalResortBoss,
                            b"stg210" => Levels::SweetMountainAct1,
                            b"stg230" => Levels::SweetMountainAct2,
                            b"stg220" => Levels::SweetMountainAct3,
                            b"stg260" => Levels::SweetMountainAct4,
                            b"stg240" => Levels::SweetMountainAct5,
                            b"stg250" => Levels::SweetMountainAct6,
                            b"stg290" => Levels::SweetMountainBoss,
                            b"stg310" => Levels::StarlightCarnivalAct1,
                            b"stg330" => Levels::StarlightCarnivalAct2,
                            b"stg340" => Levels::StarlightCarnivalAct3,
                            b"stg350" => Levels::StarlightCarnivalAct4,
                            b"stg320" => Levels::StarlightCarnivalAct5,
                            b"stg360" => Levels::StarlightCarnivalAct6,
                            b"stg390" => Levels::StarlightCarnivalBoss,
                            b"stg410" => Levels::PlanetWispAct1,
                            b"stg440" => Levels::PlanetWispAct2,
                            b"stg450" => Levels::PlanetWispAct3,
                            b"stg430" => Levels::PlanetWispAct4,
                            b"stg460" => Levels::PlanetWispAct5,
                            b"stg420" => Levels::PlanetWispAct6,
                            b"stg490" => Levels::PlanetWispBoss,
                            b"stg510" => Levels::AquariumParkAct1,
                            b"stg540" => Levels::AquariumParkAct2,
                            b"stg550" => Levels::AquariumParkAct3,
                            b"stg530" => Levels::AquariumParkAct4,
                            b"stg560" => Levels::AquariumParkAct5,
                            b"stg520" => Levels::AquariumParkAct6,
                            b"stg590" => Levels::AquariumParkBoss,
                            b"stg610" => Levels::AsteroidCoasterAct1,
                            b"stg630" => Levels::AsteroidCoasterAct2,
                            b"stg640" => Levels::AsteroidCoasterAct3,
                            b"stg650" => Levels::AsteroidCoasterAct4,
                            b"stg660" => Levels::AsteroidCoasterAct5,
                            b"stg620" => Levels::AsteroidCoasterAct6,
                            b"stg690" => Levels::AsteroidCoasterBoss,
                            b"stg710" => Levels::TerminalVelocityAct1,
                            b"stg790" => Levels::TerminalVelocityBoss,
                            b"stg720" => Levels::TerminalVelocityAct2,
                            b"stgD10" => Levels::SonicSimulatorAct1_1,
                            b"stgB20" => Levels::SonicSimulatorAct1_2,
                            b"stgE50" => Levels::SonicSimulatorAct1_3,
                            b"stgD20" => Levels::SonicSimulatorAct2_1,
                            b"stgB30" => Levels::SonicSimulatorAct2_2,
                            b"stgF30" => Levels::SonicSimulatorAct2_3,
                            b"stgG10" => Levels::SonicSimulatorAct3_1,
                            b"stgG30" => Levels::SonicSimulatorAct3_2,
                            b"stgA10" => Levels::SonicSimulatorAct3_3,
                            b"stgD30" => Levels::SonicSimulatorAct4_1,
                            b"stgG20" => Levels::SonicSimulatorAct4_2,
                            b"stgC50" => Levels::SonicSimulatorAct4_3,
                            b"stgE30" => Levels::SonicSimulatorAct5_1,
                            b"stgB10" => Levels::SonicSimulatorAct5_2,
                            b"stgE40" => Levels::SonicSimulatorAct5_3,
                            b"stgG40" => Levels::SonicSimulatorAct6_1,
                            b"stgC40" => Levels::SonicSimulatorAct6_2,
                            b"stgF40" => Levels::SonicSimulatorAct6_3,
                            b"stgA30" => Levels::SonicSimulatorAct7_1,
                            b"stgE20" => Levels::SonicSimulatorAct7_2,
                            b"stgC10" => Levels::SonicSimulatorAct7_3,
                            _ => Levels::None,
                        },
                        _ => Levels::None,
                    };

                    if level != Levels::None {
                        igt = match proc.read::<f32>(addr_3 + 0x270) {
                            Ok(x) => Duration::milliseconds((x * 100.0) as i64 * 10),
                            _ => Duration::ZERO,
                        };

                        goal_ring = match proc.read::<u8>(addr_3 + 0x110) {
                            Ok(x) => (x & (1 << 5)) != 0,
                            _ => false,
                        };
                    }
                }

                if let Ok(addr_3) = proc.read::<Address64>(addr_2 + 0x68) {
                    if let Ok(addr_4) = proc.read::<Address64>(addr_3 + 0x110) {
                        if let Ok(sh) = proc.read::<u8>(addr_4) {
                            eggshuttle_totalstages = sh;

                            if let Ok(x) = proc.read::<u8>(addr_4 + 0xB8) {
                                eggshuttle_progressiveid = x;
                            }
                        }
                    }
                }
            }

            if let Ok(addr_2) = proc.read::<Address64>(addr_1 + 0x8) {
                if let Ok(addr_3) = proc.read::<Address64>(addr_2 + 0x10) {
                    if let Ok(addr_4) = proc.read::<Address64>(addr_3 + 0x60) {
                        if let Ok(x) = proc.read::<u8>(addr_4 + 0x120) {
                            runstart = x;

                            if let Ok(y) = proc.read::<i8>(addr_4 + 0x1CC) {
                                tr1rank = y;
                            }
                        }
                    }
                }
            }
        }
    }

    watchers.levelid.update_infallible(level);
    watchers.igt.update_infallible(igt);
    watchers.goalringreached.update_infallible(goal_ring);
    watchers
        .eggshuttle_progressiveid
        .update_infallible(eggshuttle_progressiveid);
    watchers
        .eggshuttle_totalstages
        .update_infallible(eggshuttle_totalstages);
    watchers.runstart.update_infallible(runstart);
    watchers.tr1rank.update_infallible(tr1rank);

    if timer::state() == TimerState::NotRunning {
        if watchers.accumulatedigt != Duration::ZERO {
            watchers.accumulatedigt = Duration::ZERO
        }

        if let Some(eggshuttlecount) = &watchers.eggshuttle_totalstages.pair {
            watchers.currentgamemode =
                match eggshuttlecount.current > 0 && eggshuttlecount.current <= 45 {
                    true => GameMode::EggShuttle,
                    _ => GameMode::AnyPercent,
                };
        }
    }

    if let Some(igtpair) = &watchers.igt.pair {
        if igtpair.old != Duration::ZERO && igtpair.current == Duration::ZERO {
            watchers.accumulatedigt += igtpair.old
        }
    }
}

fn start(watchers: &Watchers, settings: &Settings) -> bool {
    let Some(level_id) = &watchers.levelid.pair else {
        return false;
    };
    let Some(igt) = &watchers.igt.pair else {
        return false;
    };
    let Some(runstart) = &watchers.runstart.pair else {
        return false;
    };
    let Some(tr1rank) = &watchers.tr1rank.pair else {
        return false;
    };

    match watchers.currentgamemode {
        GameMode::EggShuttle => {
            settings.start_egg_shuttle
                && level_id.current == Levels::TropicalResortAct1
                && (level_id.old == Levels::None
                    || (igt.old > asr::time::Duration::ZERO
                        && igt.current == asr::time::Duration::ZERO))
        }
        _ => {
            (settings.start_anypercent
                && tr1rank.current == -1
                && runstart.old == 35
                && runstart.current == 110)
                || (settings.start_sonic_simulator
                    && level_id.current == Levels::SonicSimulatorAct1_1
                    && level_id.old == Levels::None)
        }
    }
}

fn split(watchers: &Watchers, settings: &Settings) -> bool {
    let Some(levelid) = &watchers.levelid.pair else {
        return false;
    };

    let cont = match levelid.old {
        Levels::TropicalResortAct1 => settings.tropical_resort_1,
        Levels::TropicalResortAct2 => settings.tropical_resort_2,
        Levels::TropicalResortAct3 => settings.tropical_resort_3,
        Levels::TropicalResortAct4 => settings.tropical_resort_4,
        Levels::TropicalResortAct5 => settings.tropical_resort_5,
        Levels::TropicalResortAct6 => settings.tropical_resort_6,
        Levels::TropicalResortBoss => settings.tropical_resort_boss,
        Levels::SweetMountainAct1 => settings.sweet_mountain_1,
        Levels::SweetMountainAct2 => settings.sweet_mountain_2,
        Levels::SweetMountainAct3 => settings.sweet_mountain_3,
        Levels::SweetMountainAct4 => settings.sweet_mountain_4,
        Levels::SweetMountainAct5 => settings.sweet_mountain_5,
        Levels::SweetMountainAct6 => settings.sweet_mountain_6,
        Levels::SweetMountainBoss => settings.sweet_mountain_boss,
        Levels::StarlightCarnivalAct1 => settings.starlight_carnival_1,
        Levels::StarlightCarnivalAct2 => settings.starlight_carnival_2,
        Levels::StarlightCarnivalAct3 => settings.starlight_carnival_3,
        Levels::StarlightCarnivalAct4 => settings.starlight_carnival_4,
        Levels::StarlightCarnivalAct5 => settings.starlight_carnival_5,
        Levels::StarlightCarnivalAct6 => settings.starlight_carnival_6,
        Levels::StarlightCarnivalBoss => settings.starlight_carnival_boss,
        Levels::PlanetWispAct1 => settings.planet_wisp_1,
        Levels::PlanetWispAct2 => settings.planet_wisp_2,
        Levels::PlanetWispAct3 => settings.planet_wisp_3,
        Levels::PlanetWispAct4 => settings.planet_wisp_4,
        Levels::PlanetWispAct5 => settings.planet_wisp_5,
        Levels::PlanetWispAct6 => settings.planet_wisp_6,
        Levels::PlanetWispBoss => settings.planet_wisp_boss,
        Levels::AquariumParkAct1 => settings.aquarium_park_1,
        Levels::AquariumParkAct2 => settings.aquarium_park_2,
        Levels::AquariumParkAct3 => settings.aquarium_park_3,
        Levels::AquariumParkAct4 => settings.aquarium_park_4,
        Levels::AquariumParkAct5 => settings.aquarium_park_5,
        Levels::AquariumParkAct6 => settings.aquarium_park_6,
        Levels::AquariumParkBoss => settings.aquarium_park_boss,
        Levels::AsteroidCoasterAct1 => settings.asteroid_coaster_1,
        Levels::AsteroidCoasterAct2 => settings.asteroid_coaster_2,
        Levels::AsteroidCoasterAct3 => settings.asteroid_coaster_3,
        Levels::AsteroidCoasterAct4 => settings.asteroid_coaster_4,
        Levels::AsteroidCoasterAct5 => settings.asteroid_coaster_5,
        Levels::AsteroidCoasterAct6 => settings.asteroid_coaster_6,
        Levels::AsteroidCoasterBoss => settings.asteroid_coaster_boss,
        Levels::TerminalVelocityAct1 => settings.terminal_velocity_1,
        Levels::TerminalVelocityBoss => settings.terminal_velocity_boss,
        Levels::TerminalVelocityAct2 => settings.terminal_velocity_2,
        Levels::SonicSimulatorAct1_1 => settings.sonic_simulator_1_1,
        Levels::SonicSimulatorAct1_2 => settings.sonic_simulator_1_2,
        Levels::SonicSimulatorAct1_3 => settings.sonic_simulator_1_3,
        Levels::SonicSimulatorAct2_1 => settings.sonic_simulator_2_1,
        Levels::SonicSimulatorAct2_2 => settings.sonic_simulator_2_2,
        Levels::SonicSimulatorAct2_3 => settings.sonic_simulator_2_3,
        Levels::SonicSimulatorAct3_1 => settings.sonic_simulator_3_1,
        Levels::SonicSimulatorAct3_2 => settings.sonic_simulator_3_2,
        Levels::SonicSimulatorAct3_3 => settings.sonic_simulator_3_3,
        Levels::SonicSimulatorAct4_1 => settings.sonic_simulator_4_1,
        Levels::SonicSimulatorAct4_2 => settings.sonic_simulator_4_2,
        Levels::SonicSimulatorAct4_3 => settings.sonic_simulator_4_3,
        Levels::SonicSimulatorAct5_1 => settings.sonic_simulator_5_1,
        Levels::SonicSimulatorAct5_2 => settings.sonic_simulator_5_2,
        Levels::SonicSimulatorAct5_3 => settings.sonic_simulator_5_3,
        Levels::SonicSimulatorAct6_1 => settings.sonic_simulator_6_1,
        Levels::SonicSimulatorAct6_2 => settings.sonic_simulator_6_2,
        Levels::SonicSimulatorAct6_3 => settings.sonic_simulator_6_3,
        Levels::SonicSimulatorAct7_1 => settings.sonic_simulator_7_1,
        Levels::SonicSimulatorAct7_2 => settings.sonic_simulator_7_2,
        Levels::SonicSimulatorAct7_3 => settings.sonic_simulator_7_3,
        _ => false,
    };

    if !cont {
        return false;
    }

    let Some(goalringreached) = &watchers.goalringreached.pair else {
        return false;
    };

    if watchers.currentgamemode == GameMode::EggShuttle {
        let Some(progressiveid) = &watchers.eggshuttle_progressiveid.pair else {
            return false;
        };
        let Some(totalstages) = &watchers.eggshuttle_totalstages.pair else {
            return false;
        };

        if progressiveid.old == totalstages.current - 1 {
            goalringreached.current && !goalringreached.old
        } else {
            progressiveid.current == progressiveid.old + 1
        }
    } else if levelid.old == Levels::TerminalVelocityAct2 {
        goalringreached.current && !goalringreached.old
    } else {
        !goalringreached.current && goalringreached.old
    }
}

fn reset(watchers: &Watchers, settings: &Settings) -> bool {
    if watchers.currentgamemode == GameMode::EggShuttle {
        let Some(igt) = &watchers.igt.pair else {
            return false;
        };
        let Some(goal_ring) = &watchers.goalringreached.pair else {
            return false;
        };
        settings.reset_egg_shuttle
            && igt.old != Duration::ZERO
            && igt.current == Duration::ZERO
            && !goal_ring.old
    } else {
        let Some(runstart) = &watchers.runstart.pair else {
            return false;
        };
        settings.reset_anypercent && runstart.old == 110 && runstart.current == 35
    }
}

fn is_loading(_watchers: &Watchers, _settings: &Settings) -> Option<bool> {
    Some(true)
}

fn game_time(watchers: &Watchers, _settings: &Settings) -> Option<Duration> {
    Some(watchers.igt.pair?.current + watchers.accumulatedigt)
}

#[derive(Clone, Copy, PartialEq, Default)]
enum GameMode {
    #[default]
    AnyPercent,
    EggShuttle,
}

#[derive(Clone, Copy, PartialEq)]
enum Levels {
    TropicalResortAct1,
    TropicalResortAct2,
    TropicalResortAct3,
    TropicalResortAct4,
    TropicalResortAct5,
    TropicalResortAct6,
    TropicalResortBoss,
    SweetMountainAct1,
    SweetMountainAct2,
    SweetMountainAct3,
    SweetMountainAct4,
    SweetMountainAct5,
    SweetMountainAct6,
    SweetMountainBoss,
    StarlightCarnivalAct1,
    StarlightCarnivalAct2,
    StarlightCarnivalAct3,
    StarlightCarnivalAct4,
    StarlightCarnivalAct5,
    StarlightCarnivalAct6,
    StarlightCarnivalBoss,
    PlanetWispAct1,
    PlanetWispAct2,
    PlanetWispAct3,
    PlanetWispAct4,
    PlanetWispAct5,
    PlanetWispAct6,
    PlanetWispBoss,
    AquariumParkAct1,
    AquariumParkAct2,
    AquariumParkAct3,
    AquariumParkAct4,
    AquariumParkAct5,
    AquariumParkAct6,
    AquariumParkBoss,
    AsteroidCoasterAct1,
    AsteroidCoasterAct2,
    AsteroidCoasterAct3,
    AsteroidCoasterAct4,
    AsteroidCoasterAct5,
    AsteroidCoasterAct6,
    AsteroidCoasterBoss,
    TerminalVelocityAct1,
    TerminalVelocityBoss,
    TerminalVelocityAct2,
    SonicSimulatorAct1_1,
    SonicSimulatorAct1_2,
    SonicSimulatorAct1_3,
    SonicSimulatorAct2_1,
    SonicSimulatorAct2_2,
    SonicSimulatorAct2_3,
    SonicSimulatorAct3_1,
    SonicSimulatorAct3_2,
    SonicSimulatorAct3_3,
    SonicSimulatorAct4_1,
    SonicSimulatorAct4_2,
    SonicSimulatorAct4_3,
    SonicSimulatorAct5_1,
    SonicSimulatorAct5_2,
    SonicSimulatorAct5_3,
    SonicSimulatorAct6_1,
    SonicSimulatorAct6_2,
    SonicSimulatorAct6_3,
    SonicSimulatorAct7_1,
    SonicSimulatorAct7_2,
    SonicSimulatorAct7_3,
    None,
}
