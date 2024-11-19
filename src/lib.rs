#[macro_export]
#[rustfmt::skip]
macro_rules! the_crate { {} => { #[allow(dead_code)] mod emoji {
pub mod to_mindustry {
    pub mod named {
        include!{concat!(env!("OUT_DIR"), "/to_mindustry_named")}
    }
}

/// get discord emojis by name
pub mod named {
    include!{concat!(env!("OUT_DIR"), "/to_discord_named")}
}

/// map mindustry emojis
pub mod mindustry {
    use super::named;
    /// mindustry liquid, i.e gallium, water, oil
    pub use mindus::fluid::Type as Fluid;
    /// mindustry item, i.e copper, lead, silicon, phase fabric
    pub use mindus::item::Type as Item;
    include!{concat!(env!("OUT_DIR"), "/to_discord")}

    /// Returns the emoji of a [`Fluid`]
    pub const fn fluid(f: Fluid) -> &'static str {
        match f {
            Fluid::Arkycite => named::ARKYCITE,
            Fluid::Cryofluid => named::CRYOFLUID,
            Fluid::Cyanogen => named::CYANOGEN,
            Fluid::Gallium => named::GALLIUM,
            Fluid::Hydrogen => named::HYDROGEN,
            Fluid::Neoplasm => named::NEOPLASM,
            Fluid::Nitrogen => named::NITROGEN,
            Fluid::Oil => named::OIL,
            Fluid::Ozone => named::OZONE,
            Fluid::Slag => named::SLAG,
            Fluid::Water => named::WATER,
        }
    }

    /// Returns the emoji of a [`Item`]
    pub const fn item(i: Item) -> &'static str {
        match i {
            Item::Beryllium => named::BERYLLIUM,
            Item::BlastCompound => named::BLASTCOMPOUND,
            Item::Carbide => named::CARBIDE,
            Item::Coal => named::COAL,
            Item::Copper => named::COPPER,
            Item::DormantCyst => named::DORMANTCYST,
            Item::FissileMatter => named::FISSILEMATTER,
            Item::Graphite => named::GRAPHITE,
            Item::Lead => named::LEAD,
            Item::Metaglass => named::METAGLASS,
            Item::Oxide => named::OXIDE,
            Item::PhaseFabric => named::PHASEFABRIC,
            Item::Plastanium => named::PLASTANIUM,
            Item::Pyratite => named::PYRATITE,
            Item::Sand => named::SAND,
            Item::Scrap => named::SCRAP,
            Item::Silicon => named::SILICON,
            Item::SporePod => named::SPOREPOD,
            Item::SurgeAlloy => named::SURGEALLOY,
            Item::Thorium => named::THORIUM,
            Item::Titanium => named::TITANIUM,
            Item::Tungsten => named::TUNGSTEN,
        }
    }

    /// "fixes" mindustry emojis with discord emojis
    /// ```
    /// assert_eq!(
    ///   emoji::mindustry::to_discord("the  will output many /s"),
    ///   "the <:spore_press:1164832597411647508> will output many <:sporepod:1144220601205149777>/s"
    /// );
    /// ```
    pub fn to_discord(s: &str) -> String {
        let mut o = String::with_capacity(s.len());
        for ch in s.chars() {
            if let Some(mtch) = TO_DISCORD.get(&ch) {
                o.push_str(mtch);
            } else {
                o.push(ch);
            }
        }
        o
    }
}}};
}

#[cfg(feature = "build")]
mod build {
    use std::{
        collections::HashMap,
        fs::File,
        path::PathBuf,
        sync::{Arc, OnceLock},
    };

    use serenity::all::{Context, ShardManager};

    pub fn load() {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(_load())
    }

    async fn _load() {
        static SHARD_MNGR: OnceLock<Arc<ShardManager>> = OnceLock::new();
        let tok = std::env::var("TOKEN")
            .unwrap_or_else(|_| std::fs::read_to_string("token").expect("wher token"));
        let f = poise::Framework::builder()
            .options(poise::FrameworkOptions::default())
            .setup(|c, _ready, _: &poise::Framework<(), anyhow::Error>| {
                Box::pin(async move {
                    build_files(c).await;
                    ShardManager::shutdown_all(SHARD_MNGR.get().unwrap()).await;
                    Ok(())
                })
            })
            .build();

        let mut c =
            serenity::all::ClientBuilder::new(tok, serenity::all::GatewayIntents::non_privileged())
                .framework(f)
                .await
                .unwrap();
        SHARD_MNGR.set(c.shard_manager.clone()).unwrap();
        c.start().await.unwrap()
    }

    async fn build_files(c: &Context) {
        use std::io::Write;
        let emojis = c.get_application_emojis().await.unwrap();
        let of =
            |f| File::create(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join(f)).unwrap();

        let mut f = of("to_discord");
        _ = write!(
            f,
            r#"
        /// to discord emojis
        pub static TO_DISCORD: phf::Map<char, &str> = phf::phf_map! {{"#
        );
        let to_mindustry = LIST.iter().copied().collect::<HashMap<_, _>>();
        for (str, char) in emojis
            .iter()
            .filter_map(|e| to_mindustry.get(&*e.name).map(|x| (e.to_string(), x)))
        {
            _ = write!(f, r#"'{char}' => "{str}","#);
        }
        _ = write!(f, "}};");

        let mut f = of("to_mindustry_named");
        for (str, char) in emojis
            .iter()
            .filter_map(|e| to_mindustry.get(&*e.name).map(|x| (&e.name, x)))
        {
            let str = str.to_owned().to_uppercase();
            _ = write!(
                f,
                r#"#[doc = "{str} => {char}"] pub const {str}: &str = "{char}";"#
            );
        }

        let mut f = of("to_discord_named");
        for (k, v) in emojis.iter().map(|e| (e.name.to_string(), e.to_string())) {
            let k = k.to_owned().to_uppercase();
            _ = write!(f, r#"#[doc = "{v}"] pub const {k}: &str = "{v}";"#);
        }
    }

    #[rustfmt::skip]
    const LIST: &[(&str, char)] = &[("ship_assembler",''),("tungsten_wall",''),("blast_door",''),("ship_refabricator",''),("reinforced_liquid_container",''),("tank_assembler",''),("tank_fabricator",''),("tank_refabricator",''),("prime_refabricator",''),("malign",''),("mech_refabricator",''),("lustre",''),("reinforced_liquid_junction",''),("reinforced_bridge_conduit",''),("titan",''),("reinforced_liquid_tank",''),("reinforced_surge_wall_large",''),("reinforced_conduit",''),("smite",''),("reinforced_surge_wall",''),("basic_assembler_module",''),("tungsten_wall_large",''),("reinforced_pump",''),("shielded_wall",''),("ship_fabricator",''),("beryllium_wall_large",''),("carbide_wall_large",''),("breach",''),("sublimate",''),("beryllium_wall",''),("mech_fabricator",''),("reinforced_liquid_router",''),("mech_assembler",''),("afflict",''),("disperse",''),("scathe",''),("carbide_wall",''),("cleroi",''),("locus",''),("omura",''),("risso",''),("manifold",''),("collaris",''),("emanate",''),("anthicus",''),("spiroct",''),("tecta",''),("quell",''),("mace",''),("incite",''),("latum",''),("anthicus_missile",''),("oxynoe",''),("obviate",''),("alpha",''),("disrupt_missile",''),("renale",''),("assembly_drone",''),("vanquish",''),("avert",''),("precept",''),("scathe_missile",''),("quell_missile",''),("beta",''),("merui",''),("evoke",''),("aegires",''),("stell",''),("disrupt",''),("elude",''),("navanax",''),("conquer",''),("large_payload_mass_driver",''),("deconstructor",''),("reinforced_payload_conveyor",''),("constructor",''),("small_deconstructor",''),("reinforced_payload_router",''),("large_constructor",''),("ductbridge",''),("turbine_condenser",''),("core_citadel",''),("slag_heater",''),("shockwave_tower",''),("beam_link",''),("reinforced_vault",''),("phase_synthesizer",''),("cliff_crusher",''),("heat_source",''),("electric_heater",''),("beam_tower",''),("unit_cargo_loader",''),("slag_incinerator",''),("unit_repair_tower",''),("beam_node",''),("heat_router",''),("reinforced_container",''),("atmospheric_concentrator",''),("carbide_crucible",''),("chemical_combustion_chamber",''),("build_tower",''),("impact_drill",''),("pyrolysis_generator",''),("core_bastion",''),("eruption_drill",''),("large_plasma_bore",''),("plasma_bore",''),("neoplasia_reactor",''),("core_acropolis",''),("silicon_arcfurnace",''),("large_shield_projector",''),("flux_reactor",''),("regen_projector",''),("heat_redirector",''),("electrolyzer",''),("cyanogen_synthesizer",''),("surge_crucible",''),("oxidation_chamber",''),("radar",''),("phase_heater",''),("vent_condenser",''),("additive_reconstructor",''),("air_factory",''),("antumbra",''),("arc",''),("arkyid",''),("armored_conveyor",''),("armored_duct",''),("atrax",''),("battery",''),("battery_large",''),("blast_drill",''),("blast_mixer",''),("bridge_conduit",''),("bridge_conveyor",''),("bryde",''),("canvas",''),("coal_centrifuge",''),("combustion_generator",''),("conduit",''),("container",''),("conveyor",''),("copper_wall",''),("copper_wall_large",''),("core_foundation",''),("core_nucleus",''),("core_shard",''),("corvus",''),("crawler",''),("cryofluid_mixer",''),("cultivator",''),("cyclone",''),("cyerce",''),("dagger",''),("differential_generator",''),("diode",''),("disassembler",''),("distributor",''),("door",''),("door_large",''),("duct",''),("duo",''),("eclipse",''),("exponential_reconstructor",''),("flare",''),("force_projector",''),("foreshadow",''),("fortress",''),("fuse",''),("gamma",''),("graphite_press",''),("ground_factory",''),("hail",''),("horizon",''),("hyper_processor",''),("illuminator",''),("impact_reactor",''),("impulse_pump",''),("incinerator",''),("interplanetary_accelerator",''),("inverted_sorter",''),("item_source",''),("item_void",''),("junction",''),("kiln",''),("lancer",''),("large_logic_display",''),("laser_drill",''),("launch_pad",''),("liquid_container",''),("liquid_junction",''),("liquid_router",''),("liquid_source",''),("liquid_tank",''),("liquid_void",''),("logic_display",''),("logic_processor",''),("mass_driver",''),("mechanical_drill",''),("mechanical_pump",''),("mega",''),("meltdown",''),("melter",''),("memory_bank",''),("memory_cell",''),("mender",''),("mend_projector",''),("message",''),("micro_processor",''),("minke",''),("mono",''),("multiplicative_reconstructor",''),("multi_press",''),("naval_factory",''),("nova",''),("oct",''),("oil_extractor",''),("overdrive_dome",''),("overdrive_projector",''),("overflow_gate",''),("parallax",''),("payload_conveyor",''),("payload_loader",''),("payload_mass_driver",''),("payload_router",''),("payload_source",''),("payload_unloader",''),("payload_void",''),("phase_conduit",''),("phase_conveyor",''),("phase_wall",''),("phase_wall_large",''),("phase_weaver",''),("plastanium_compressor",''),("plastanium_conveyor",''),("plastanium_wall",''),("plastanium_wall_large",''),("plated_conduit",''),("pneumatic_drill",''),("poly",''),("power_node",''),("power_node_large",''),("pulsar",''),("pulse_conduit",''),("pulverizer",''),("pyratite_mixer",''),("quad",''),("quasar",''),("reign",''),("retusa",''),("ripple",''),("rotary_pump",''),("router",''),("rtg_generator",''),("salvo",''),("scatter",''),("scepter",''),("scorch",''),("scrap_wall",''),("scrap_wall_gigantic",''),("scrap_wall_huge",''),("scrap_wall_large",''),("segment",''),("sei",''),("separator",''),("shield_projector",''),("silicon_crucible",''),("silicon_smelter",''),("solar_panel",''),("solar_panel_large",''),("sorter",''),("spectre",''),("spore_press",''),("steam_generator",''),("surge_conveyor",''),("surge_smelter",''),("surge_tower",''),("surge_wall",''),("surge_wall_large",''),("swarmer",''),("switch",''),("tetrative_reconstructor",''),("thermal_generator",''),("thorium_reactor",''),("thorium_wall",''),("thorium_wall_large",''),("thruster",''),("titanium_conveyor",''),("titanium_wall",''),("titanium_wall_large",''),("toxopid",''),("tsunami",''),("underflow_gate",''),("unloader",''),("vault",''),("vela",''),("water_extractor",''),("wave",''),("world_cell",''),("world_message",''),("world_processor",''),("zenith",''),("add",''),("admin",''),("android",''),("attack",''),("book",''),("book_open",''),("box",''),("cancel",''),("chart_bar",''),("chat",''),("command_attack",''),("command_rally",''),("copy",''),("crafting",''),("defense",''),("diagonal",''),("discord",''),("distribution",''),("download",''),("down_open",''),("edit",''),("editor",''),("editor",''),("effect",''),("eraser",''),("exit",''),("export",''),("eye",''),("eye_off",''),("file",''),("file_image",''),("file_text",''),("file_text",''),("fill",''),("filter",''),("filters",''),("flip_x",''),("flip_y",''),("folder",''),("github",''),("github_square",''),("google_play",''),("grid",''),("hammer",''),("home",''),("host",''),("down",''),("image",''),("info",''),("info_circle",''),("itchio",''),("layers",''),("left",''),("left_open",''),("line",''),("link",''),("liquid",''),("list",''),("lock",''),("lock_open",''),("logic",''),("map",''),("move",''),("none",''),("ok",''),("paste",''),("pause",''),("pencil",''),("pick",''),("plane",''),("planet",''),("play",''),("players",''),("power",''),("power",''),("production",''),("pvp",''),("reddit",''),("redo",''),("refresh",''),("resize",''),("right",''),("right_open",''),("right_open",''),("rotate",''),("save",''),("settings",''),("spray",''),("star",''),("steam",''),("survival",''),("terminal",''),("trash",''),("tree",''),("trello",''),("turret",''),("undo",''),("units",''),("up",''),("upload",''),("up_open",''),("warning",'⚠'),("waves",''),("wrench",''),("zoom",''),("arkycite",''),("cryofluid",''),("cyanogen",''),("gallium",''),("hydrogen",''),("neoplasm",''),("nitrogen",''),("oil",''),("ozone",''),("slag",''),("water",''),("beryllium",''),("blastcompound",''),("carbide",''),("coal",''),("copper",''),("dormantcyst",''),("fissilematter",''),("graphite",''),("lead",''),("metaglass",''),("oxide",''),("phasefabric",''),("plastanium",''),("pyratite",''),("sand",''),("scrap",''),("silicon",''),("sporepod",''),("surgealloy",''),("thorium",''),("titanium",''),("tungsten",'')];
}
#[cfg(feature = "build")]
pub use build::load;
