#![no_std]
use asr::{signature::Signature, timer, timer::TimerState, watcher::Watcher, Address, Process, time::Duration};

#[cfg(all(not(test), target_arch = "wasm32"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

static AUTOSPLITTER: spinning_top::Spinlock<State> = spinning_top::const_spinlock(State {
    game: None,
    settings: None,
    watchers: Watchers {
        levelid: Watcher::new(),
        igt: Watcher::new(),
        goalringreached: Watcher::new(),
        eggshuttle_totalstages: Watcher::new(),
        eggshuttle_progressiveid: Watcher::new(),
        runstart: Watcher::new(),
        tr1rank: Watcher::new(),
        accumulatedigt: Duration::ZERO,
        currentgamemode: GameMode::AnyPercent,
    },
});

struct State {
    game: Option<ProcessInfo>,
    settings: Option<Settings>,
    watchers: Watchers,
}

struct ProcessInfo {
    game: Process,
    main_module_base: Address,
    main_module_size: u64,
    addresses: Option<MemoryPtr>,
}

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

struct MemoryPtr {
    base_address: Address,
}


#[derive(asr::Settings)]
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

impl ProcessInfo {
    fn attach_process() -> Option<Self> {
        const PROCESS_NAMES: [&str; 2] = ["SonicColorsUltimate.exe", "Sonic Colors - Ultimate.exe"];
        let mut proc: Option<Process> = None;
        let mut proc_name: Option<&str> = None;
    
        for name in PROCESS_NAMES {
            proc = Process::attach(name);
            if proc.is_some() {
                proc_name = Some(name);
                break
            }
        }
    
        let game = proc?;
        let main_module_base = game.get_module_address(proc_name?).ok()?;
        let main_module_size = game.get_module_size(proc_name?).ok()?;

        Some(Self {
            game,
            main_module_base,
            main_module_size,
            addresses: None,
        })
    }

    fn look_for_addresses(&mut self) -> Option<MemoryPtr> {
        const SIG: Signature<5> = Signature::new("76 0C 48 8B 0D");
        let game = &self.game;

        let mut ptr = SIG.scan_process_range(game, self.main_module_base, self.main_module_size)?.0 + 5;
        ptr += 0x4 + game.read::<u32>(Address(ptr)).ok()? as u64;

        Some(MemoryPtr {
            base_address: Address(ptr),
        })
    }
}

impl State {
    fn init(&mut self) -> bool {        
        if self.game.is_none() {
            self.game = ProcessInfo::attach_process()
        }

        let Some(game) = &mut self.game else {
            return false
        };

        if !game.game.is_open() {
            self.game = None;
            return false
        }

        if game.addresses.is_none() {
            game.addresses = game.look_for_addresses()
        }

        game.addresses.is_some()   
    }

    fn update(&mut self) {
        let Some(game) = &self.game else { return };
        let Some(addresses) = &game.addresses else { return };
        let proc = &game.game;

        let mut level: Levels = Levels::None;
        let mut igt = Duration::ZERO;
        let mut goal_ring = false;
        let mut eggshuttle_progressiveid: u8 = 0;
        let mut eggshuttle_totalstages: u8 = 0;
        let mut runstart: u8 = 0;
        let mut tr1rank: i8 = 0;

        if let Ok(addr_base) = proc.read::<u64>(addresses.base_address) {
            if let Ok(addr_1) = proc.read::<u64>(Address(addr_base + 0x8)) {
                if let Ok(addr_2) = proc.read::<u64>(Address(addr_1 + 0x38)) {
                    if let Ok(addr_3) = proc.read::<u64>(Address(addr_2 + 0x60)) {
                        let level_id = proc.read::<[u8; 6]>(Address(addr_3 + 0xE0));
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
                            igt = match proc.read::<f32>(Address(addr_3 + 0x270)) {
                                Ok(x) => Duration::milliseconds((x * 100.0) as i64 * 10),
                                _ => Duration::ZERO,
                            };

                            goal_ring = match proc.read::<u8>(Address(addr_3 + 0x110)) {
                                Ok(x) => (x & (1 << 5)) != 0,
                                _ => false,
                            };
                        }
                    }

                    if let Ok(addr_3) = proc.read::<u64>(Address(addr_2 + 0x68)) {
                        if let Ok(addr_4) = proc.read::<u64>(Address(addr_3 + 0x110)) {
                            if let Ok(sh) = proc.read::<u8>(Address(addr_4)) {
                                eggshuttle_totalstages = sh;

                                if let Ok(x) = proc.read::<u8>(Address(addr_4 + 0xB8)) {
                                    eggshuttle_progressiveid = x;
                                }
                            }
                        }
                    }
                }

                if let Ok(addr_2) = proc.read::<u64>(Address(addr_1 + 0x8)) {
                    if let Ok(addr_3) = proc.read::<u64>(Address(addr_2 + 0x10)) {
                        if let Ok(addr_4) = proc.read::<u64>(Address(addr_3 + 0x60)) {
                            if let Ok(x) = proc.read::<u8>(Address(addr_4 + 0x120)) {
                                runstart = x;

                                if let Ok(y) = proc.read::<i8>(Address(addr_4 + 0x1CC)) {
                                    tr1rank = y;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.watchers.levelid.update(Some(level));
        self.watchers.igt.update(Some(igt));
        self.watchers.goalringreached.update(Some(goal_ring));
        self.watchers.eggshuttle_progressiveid.update(Some(eggshuttle_progressiveid));
        self.watchers.eggshuttle_totalstages.update(Some(eggshuttle_totalstages));
        self.watchers.runstart.update(Some(runstart));
        self.watchers.tr1rank.update(Some(tr1rank));


        if timer::state() == TimerState::NotRunning {
            if self.watchers.accumulatedigt != Duration::ZERO {
                self.watchers.accumulatedigt = Duration::ZERO
            }

            if let Some(eggshuttlecount) = &self.watchers.eggshuttle_totalstages.pair {
                self.watchers.currentgamemode = match eggshuttlecount.current > 0 && eggshuttlecount.current <= 45 {
                    true => GameMode::EggShuttle,
                    _ => GameMode::AnyPercent,
                };
            }
        }

        if let Some(igtpair) = &self.watchers.igt.pair {
            if igtpair.old != Duration::ZERO && igtpair.current == Duration::ZERO {
                self.watchers.accumulatedigt += igtpair.old
            }
        }
    }

    fn start(&mut self) -> bool {
        let Some(settings) = &self.settings else { return false };
        let Some(level_id) = &self.watchers.levelid.pair else { return false };
        let Some(igt) = &self.watchers.igt.pair else { return false };
        let Some(runstart) = &self.watchers.runstart.pair else { return false };
        let Some(tr1rank) = &self.watchers.tr1rank.pair else { return false };

        match self.watchers.currentgamemode {
            GameMode::EggShuttle => settings.start_egg_shuttle && level_id.current == Levels::TropicalResortAct1 && (level_id.old == Levels::None || (igt.old > asr::time::Duration::ZERO && igt.current == asr::time::Duration::ZERO)),
            _ => (settings.start_anypercent && tr1rank.current == -1 && runstart.old == 35 && runstart.current == 110)
                || (settings.start_sonic_simulator && level_id.current == Levels::SonicSimulatorAct1_1 && level_id.old == Levels::None),
        }   
    }

    fn split(&mut self) -> bool {
        let Some(settings) = &self.settings else { return false };
        let Some(levelid) = &self.watchers.levelid.pair else { return false };

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
            return false
        }

        let Some(goalringreached) = &self.watchers.goalringreached.pair else { return false };

        if self.watchers.currentgamemode == GameMode::EggShuttle {
            let Some(progressiveid) = &self.watchers.eggshuttle_progressiveid.pair else { return false };
            let Some(totalstages) = &self.watchers.eggshuttle_totalstages.pair else { return false };

            if progressiveid.old == totalstages.current - 1 {
                goalringreached.current && !goalringreached.old
            } else {
                progressiveid.current == progressiveid.old + 1
            }
        } else {
            if levelid.old == Levels::TerminalVelocityAct2 {
                goalringreached.current && !goalringreached.old
            } else {
                !goalringreached.current && goalringreached.old
            }
        }
    }

    fn reset(&mut self) -> bool {
        let Some(settings) = &self.settings else { return false };

        if self.watchers.currentgamemode == GameMode::EggShuttle {
            let Some(igt) = &self.watchers.igt.pair else { return false };
            let Some(goal_ring) = &self.watchers.goalringreached.pair else { return false };
            settings.reset_egg_shuttle && igt.old != Duration::ZERO && igt.current == Duration::ZERO && !goal_ring.old
        } else {
            let Some(runstart) = &self.watchers.runstart.pair else { return false };
            settings.reset_anypercent && runstart.old == 110 && runstart.current == 35
        }
    }

    fn is_loading(&mut self) -> Option<bool> {
        Some(true)
    }

    fn game_time(&mut self) -> Option<Duration> {
        let Some(igt) = &self.watchers.igt.pair else { return Some(Duration::ZERO) };
        Some(igt.current + self.watchers.accumulatedigt)
    }
}

#[no_mangle]
pub extern "C" fn update() {
    // Get access to the spinlock
    let autosplitter = &mut AUTOSPLITTER.lock();
    
    // Sets up the settings
    autosplitter.settings.get_or_insert_with(Settings::register);

    // Main autosplitter logic, essentially refactored from the OG LivaSplit autosplitting component.
    // First of all, the autosplitter needs to check if we managed to attach to the target process,
    // otherwise there's no need to proceed further.
    if !autosplitter.init() {
        return
    }

    // The main update logic is launched with this
    autosplitter.update();

    // Splitting logic. Adapted from OG LiveSplit:
    // Order of execution
    // 1. update() [this is launched above] will always be run first. There are no conditions on the execution of this action.
    // 2. If the timer is currently either running or paused, then the isLoading, gameTime, and reset actions will be run.
    // 3. If reset does not return true, then the split action will be run.
    // 4. If the timer is currently not running (and not paused), then the start action will be run.
    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused {
        if let Some(is_loading) = autosplitter.is_loading() {
            if is_loading {
                timer::pause_game_time()
            } else {
                timer::resume_game_time()
            }
        }

        if let Some(game_time) = autosplitter.game_time() {
            timer::set_game_time(game_time)
        }

        if autosplitter.reset() {
            timer::reset()
        } else if autosplitter.split() {
            timer::split()
        }
    } 

    if timer::state() == TimerState::NotRunning {
        if autosplitter.start() {
            timer::start();

            if let Some(is_loading) = autosplitter.is_loading() {
                if is_loading {
                    timer::pause_game_time()
                } else {
                    timer::resume_game_time()
                }
            }
        }
    }     
}

#[derive(Clone, Copy, PartialEq)]
enum GameMode {
    AnyPercent,
    EggShuttle
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