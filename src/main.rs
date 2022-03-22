#[derive(Debug, PartialEq, Eq, Clone)]
enum Attribute {
    Strength,
    Intelligence,
    Willpower,
    Agility,
    Speed,
    Endurance,
    Personality,
    Luck,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DerivedAttribute {
    Health,
    Magicka,
    Fatigue,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Effect {
    Chameleon,
    CureDisease,
    CureParalysis,
    CurePoison,
    DetectLife,
    Dispel,
    Feather,
    FireShield,
    FortifyAttribute(Attribute),
    FortifyDerivedAttribute(DerivedAttribute),
    FrostShield,
    Invisibility,
    Light,
    NightEye,
    ReflectDamage,
    ReflectSpell,
    ResistDisease,
    ResistFire,
    ResistFrost,
    ResistParalysis,
    ResistPoison,
    ResistShock,
    RestoreAttribute(Attribute),
    RestoreDerivedAttribute(DerivedAttribute),
    Shield,
    ShockShield,
    WaterBreathing,
    WaterWalking,
    Burden,
    DamageAttribute(Attribute),
    DamageDerivedAttribute(DerivedAttribute),
    DrainAttribute(Attribute),
    DrainDerivedAttribute(DerivedAttribute),
    FireDamage,
    FrostDamage,
    Paralyze,
    ShockDamage,
    Silence,
    WeaknessToFire,
}

#[derive(Debug, Clone)]
struct Ingredient {
    name: String,
    id: String,
    description: String,
    value: u16,
    weight: f32,
    harvest_chance: f32,
    effects: [Option<Effect>; 4],
}

struct MyApp {
    ingredients: Vec<Ingredient>,
    desired_effects: [Option<Effect>; 4],
    matching_ingredients: Vec<Ingredient>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ingredients: create_ingredients(),
            desired_effects: [None, None, None, None],
            matching_ingredients: Vec::new(),
        }
    }
}

use eframe::{egui, epi};

impl epi::App for MyApp{
    fn name(&self) -> &str {
        "Alchemy Tool"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { ingredients, desired_effects, matching_ingredients } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Alchemy Tool");

            egui::ComboBox::from_label("First Effect")
            .selected_text(format!("{:?}", desired_effects[0]))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut desired_effects[0], None, "None");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Chameleon), "Chameleon");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::CureDisease),"Cure Disease");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::CureParalysis), "Cure Paralysis");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::CurePoison), "Cure Poison");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DetectLife), "Detect Life");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Dispel), "Dispel");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Feather), "Feather");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FireShield), "Fire Shield");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Strength)), "Fortify Strength");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Intelligence)), "Fortify Intelligence");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Willpower)), "Fortify Willpower");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Agility)), "Fortify Agility");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Speed)), "Fortify Speed");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Endurance)), "Fortify Endurance");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Personality)), "Fortify Personality");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyAttribute(Attribute::Luck)), "Fortify Luck");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), "Fortify Health");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), "Fortify Magicka");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), "Fortify Fatigue");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FrostShield), "Frost Shield");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Invisibility), "Invisibility");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Light), "Light");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::NightEye), "Night Eye");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ReflectDamage), "Reflect Damage");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ReflectSpell), "Reflect Spell");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistDisease), "Resist Disease");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistFire), "Resist Fire");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistFrost), "Resist Frost");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistParalysis), "Resist Paralysis");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistPoison), "Resist Poison");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ResistShock), "Resist Shock");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Strength)), "Restore Strength");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Intelligence)), "Restore Intelligence");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Willpower)), "Restore Willpower");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Agility)), "Restore Agility");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Speed)), "Restore Speed");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Endurance)), "Restore Endurance");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Personality)), "Restore Personality");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreAttribute(Attribute::Luck)), "Restore Luck");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), "Restore Health");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), "Restore Magicka");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), "Restore Fatigue");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Shield), "Shield");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ShockShield), "Shock Shield");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::WaterBreathing), "Water Breathing");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::WaterWalking), "Water Walking");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Burden), "Burden");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Strength)), "Damage Strength");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Intelligence)), "Damage Intelligence");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Willpower)), "Damage Willpower");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Agility)), "Damage Agility");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Speed)), "Damage Speed");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Endurance)), "Damage Endurance");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Personality)), "Damage Personality");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageAttribute(Attribute::Luck)), "Damage Luck");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), "Damage Health");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), "Damage Magicka");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), "Damage Fatigue");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Strength)), "Drain Strength");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Intelligence)), "Drain Intelligence");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Willpower)), "Drain Willpower");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Agility)), "Drain Agility");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Speed)), "Drain Speed");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Endurance)), "Drain Endurance");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Personality)), "Drain Personality");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainAttribute(Attribute::Luck)), "Drain Luck");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Health)), "Drain Health");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Magicka)), "Drain Magicka");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Fatigue)), "Drain Fatigue");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FireDamage), "Fire Damage");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::FrostDamage), "Frost Damage");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Paralyze), "Paralyze");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::ShockDamage), "Shock Damage");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::Silence), "Silence");
                ui.selectable_value(&mut desired_effects[0], Some(Effect::WeaknessToFire), "Weakness to Fire");
            });
            ui.end_row();

            egui::ComboBox::from_label("Second Effect")
            .selected_text(format!("{:?}", desired_effects[1]))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut desired_effects[1], None, "None");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Chameleon), "Chameleon");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::CureDisease),"Cure Disease");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::CureParalysis), "Cure Paralysis");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::CurePoison), "Cure Poison");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DetectLife), "Detect Life");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Dispel), "Dispel");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Feather), "Feather");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FireShield), "Fire Shield");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Strength)), "Fortify Strength");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Intelligence)), "Fortify Intelligence");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Willpower)), "Fortify Willpower");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Agility)), "Fortify Agility");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Speed)), "Fortify Speed");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Endurance)), "Fortify Endurance");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Personality)), "Fortify Personality");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyAttribute(Attribute::Luck)), "Fortify Luck");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), "Fortify Health");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), "Fortify Magicka");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), "Fortify Fatigue");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FrostShield), "Frost Shield");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Invisibility), "Invisibility");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Light), "Light");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::NightEye), "Night Eye");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ReflectDamage), "Reflect Damage");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ReflectSpell), "Reflect Spell");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistDisease), "Resist Disease");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistFire), "Resist Fire");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistFrost), "Resist Frost");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistParalysis), "Resist Paralysis");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistPoison), "Resist Poison");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ResistShock), "Resist Shock");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Strength)), "Restore Strength");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Intelligence)), "Restore Intelligence");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Willpower)), "Restore Willpower");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Agility)), "Restore Agility");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Speed)), "Restore Speed");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Endurance)), "Restore Endurance");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Personality)), "Restore Personality");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreAttribute(Attribute::Luck)), "Restore Luck");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), "Restore Health");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), "Restore Magicka");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), "Restore Fatigue");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Shield), "Shield");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ShockShield), "Shock Shield");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::WaterBreathing), "Water Breathing");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::WaterWalking), "Water Walking");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Burden), "Burden");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Strength)), "Damage Strength");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Intelligence)), "Damage Intelligence");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Willpower)), "Damage Willpower");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Agility)), "Damage Agility");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Speed)), "Damage Speed");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Endurance)), "Damage Endurance");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Personality)), "Damage Personality");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageAttribute(Attribute::Luck)), "Damage Luck");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), "Damage Health");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), "Damage Magicka");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), "Damage Fatigue");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Strength)), "Drain Strength");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Intelligence)), "Drain Intelligence");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Willpower)), "Drain Willpower");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Agility)), "Drain Agility");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Speed)), "Drain Speed");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Endurance)), "Drain Endurance");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Personality)), "Drain Personality");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainAttribute(Attribute::Luck)), "Drain Luck");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Health)), "Drain Health");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Magicka)), "Drain Magicka");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::DrainDerivedAttribute(DerivedAttribute::Fatigue)), "Drain Fatigue");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FireDamage), "Fire Damage");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::FrostDamage), "Frost Damage");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Paralyze), "Paralyze");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::ShockDamage), "Shock Damage");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::Silence), "Silence");
                ui.selectable_value(&mut desired_effects[1], Some(Effect::WeaknessToFire), "Weakness to Fire");
            });
            ui.end_row();
            if ui.button("Find Ingredients").clicked() {
                // Find Ingredients
                matching_ingredients = ingredients
                .iter()
                .filter(|ingredient| {
                    for desired_effect in desired_effects {
                        if !ingredient.effects.contains(desired_effect) {
                            return false;
                        }
                    }
                
                    true
                })
                .collect();
            }
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);

    /*let ingredients = create_ingredients();

    //println!("{:#?}", ingredients);

    // Let's get chameleon ingredients
    let chameleon: Vec<&Ingredient> = ingredients
        .iter()
        .filter(|ingredient| ingredient.effects.contains(&Some(Effect::Chameleon)))
        .collect();

    println!("{:?}", chameleon);

    // Let's get chameleon ingredients and filter out everything but the names
    let chameleon: Vec<String> = ingredients
        .iter()
        .filter(|ingredient| ingredient.effects.contains(&Some(Effect::Chameleon)))
        .map(|ingredient| ingredient.name.clone())
        .collect();

    println!("{:#?}", chameleon);

    let health_strength: Vec<String> = ingredients
        .iter()
        .filter(|ingredient| {
            let desired_effects: Vec<&Option<Effect>> = vec![
                &Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)),
                &Some(Effect::FortifyAttribute(Attribute::Strength)),
            ];
            for desired_effect in desired_effects {
                if !ingredient.effects.contains(desired_effect) {
                    return false;
                }
            }

            true
        })
        .map(|ingredient| ingredient.name.clone())
        .collect();

    println!("{:#?}", health_strength); */
}

fn create_ingredients() -> Vec<Ingredient> {
    vec![
        Ingredient {
            name: String::from("Alkanet Flower"),
            id: String::from("0003365C"),
            description: String::from("Harvested from Alkanet, a fairly common flower in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::ResistPoison), Some(Effect::Light), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue))],
        },
        Ingredient {
            name: String::from("Aloe Vera Leaves"),
            id: String::from("000A7924"),
            description: String::from("Harvested from Aloe Vera, a plant that grows primarily in the Gold Coast."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Ambrosia"),
            id: String::from("000704A0"),
            description: String::from("Harvested from reddish-purple colored Mana Bloom, a flower that is normally found only in Mankar Camoran's Paradise."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), None, None, None],
        },
        Ingredient {
            name: String::from("Apple"),
            id: String::from("0003365D"),
            description: String::from("Food, widely found in houses and dining halls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::FortifyAttribute(Attribute::Willpower)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Arrowroot"),
            id: String::from("0003365E"),
            description: String::from("Harvested from Arrowroot Plant, a plant that is found in the Gold Coast and Nibenay Valley."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::FortifyAttribute(Attribute::Strength)), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Beef"),
            id: String::from("0003365F"),
            description: String::from("Food, found in a few houses and dining halls."),
            value: 1u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Shield), Some(Effect::FortifyAttribute(Attribute::Agility)), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("Bergamot Seeds"),
            id: String::from("000A7933"),
            description: String::from("Harvested from Bergamot, a flower, particularly common along the Orange Road and near Bravil."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistDisease), Some(Effect::Dispel), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Blackberry"),
            id: String::from("00033663"),
            description: String::from("Food, found in many houses and dining halls. It can also be harvested from Blackberry Bushes found on some farms and in the wild in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistShock), Some(Effect::FortifyAttribute(Attribute::Endurance)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Bloodgrass"),
            id: String::from("00033664"),
            description: String::from("Harvested from Blood Grass, a common plant in the planes of Oblivion."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::Chameleon), Some(Effect::ResistParalysis), Some(Effect::Burden), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Boar Meat"),
            id: String::from("00033665"),
            description: String::from("Food, found in several houses and dining halls. Can also be collected from dead Boars, which are randomly found in many outdoors regions."),
            value: 20u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Bog Beacon Asco Cap"),
            id: String::from("0008446C"),
            description: String::from("Harvested from Bog Beacon, a plant found in Blackwood."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Shield), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Bonemeal*"),
            id: String::from("0001EBFF"),
            description: String::from("Collected from Bones Undead, i.e., skeletons and liches, which are commonly found in all undead dungeons. There is also a rare variant listed below."),
            value: 5u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistFire), Some(Effect::FortifyAttribute(Attribute::Luck)), Some(Effect::NightEye)],
        },
        Ingredient {
            name: String::from("Bread Loaf"),
            id: String::from("00023D89"),
            description: String::from("Food, widely found in houses and dining halls."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DetectLife), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::DamageAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Cairn Bolete Cap"),
            id: String::from("0006251E"),
            description: String::from("Harvested from Cairn Bolete, a mushroom found in many caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Intelligence)), Some(Effect::ResistParalysis), Some(Effect::ShockDamage)],
        },
        Ingredient {
            name: String::from("Carrot"),
            id: String::from("00033666"),
            description: String::from("Food, commonly found in houses and dining halls. Can also be harvested from Carrot Plants, which are found growing on many farms."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::NightEye), Some(Effect::FortifyAttribute(Attribute::Intelligence)), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Cheese Wedge"),
            id: String::from("00033668"),
            description: String::from("Food, commonly found in houses and dining halls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistFire), Some(Effect::FireShield), Some(Effect::DamageAttribute(Attribute::Agility))],
        },
        Ingredient {
            name: String::from("Cheese Wheel"),
            id: String::from("00033669"),
            description: String::from("Food, less commonly found in houses and dining halls than Cheese Wedges."),
            value: 1u16,
            weight: 3.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistParalysis), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::FortifyAttribute(Attribute::Willpower))],
        },
        Ingredient {
            name: String::from("Cinnabar Polypore Red Cap"),
            id: String::from("0008529C"),
            description: String::from("Harvested from red-colored Cinnabar Polypores, a mushroom that grows sparsely in the West Weald."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::Shield), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Cinnabar Polypore Yellow Cap"),
            id: String::from("0008529B"),
            description: String::from("Harvested from yellow-colored Cinnabar Polypores, a mushroom that grows sparsely in the West Weald."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Endurance)), Some(Effect::FortifyAttribute(Attribute::Endurance)), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::ReflectSpell)],
        },
        Ingredient {
            name: String::from("Clannfear Claws"),
            id: String::from("0003366A"),
            description: String::from("Collected from dead Clannfears, a type of Daedra common in the Planes of Oblivion."),
            value: 50u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::CureDisease), Some(Effect::ResistDisease), Some(Effect::Paralyze), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Clouded Funnel Cap"),
            id: String::from("00084472"),
            description: String::from("Harvested from Clouded Funnel Cap, a mushroom common in mountainous regions (Jerall and Valus Mountains) and Blackwood."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::FortifyAttribute(Attribute::Intelligence)), Some(Effect::DamageAttribute(Attribute::Endurance)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Columbine Root Pulp"),
            id: String::from("000A7925"),
            description: String::from("Harvested from Columbine, a flower that is common in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Personality)), Some(Effect::ResistFrost), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Chameleon)],
        },
        Ingredient {
            name: String::from("Corn"),
            id: String::from("0003366B"),
            description: String::from("Food, commonly found in houses and dining halls. Can also be harvested from Corn Stalks, which are found growing in many farms."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::ShockShield)],
        },
        Ingredient {
            name: String::from("Crab Meat"),
            id: String::from("0003366C"),
            description: String::from("Collected from dead Mud Crabs, a common nuisance creature found along water shores and in many dungeons."),
            value: 1u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Endurance)), Some(Effect::ResistShock), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::FireShield)],
        },
        Ingredient {
            name: String::from("Daedra Heart"),
            id: String::from("0001EC8F"),
            description: String::from("Collected from dead Dremora and Xivilai, both of which are common enemies in the Planes of Oblivion."),
            value: 25u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ShockShield), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Daedra Silk"),
            id: String::from("00033670"),
            description: String::from("Collected from dead Spider Daedra, a type of Daedra common in the Planes of Oblivion."),
            value: 75u16,
            weight: 0.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::Burden), Some(Effect::NightEye), Some(Effect::Chameleon), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Daedra Venin"),
            id: String::from("00033671"),
            description: String::from("Collected from dead Spider Daedra, a type of Daedra common in the Planes of Oblivion."),
            value: 75u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::Paralyze), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ReflectDamage)],
        },
        Ingredient {
            name: String::from("Daedroth Teeth"),
            id: String::from("00033672"),
            description: String::from("Collected from dead Daedroths, a type of Daedra common in the Planes of Oblivion."),
            value: 65u16,
            weight: 0.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::NightEye), Some(Effect::FrostShield), Some(Effect::Burden), Some(Effect::Light)],
        },
        Ingredient {
            name: String::from("Dragon's Tongue"),
            id: String::from("00025039"),
            description: String::from("Harvested from Dragon's Tongue Plants, a flower found in a few clusters south of Bravil and more sparsely throughout the West Weald."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistFire), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FireShield)],
        },
        Ingredient {
            name: String::from("Dreugh Wax"),
            id: String::from("00033673"),
            description: String::from("Collected from dead Land Dreughs, a type of monster found outdoors and in monster dungeons at high levels."),
            value: 70u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistPoison), Some(Effect::WaterBreathing), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Dryad Saddle Polypore Cap"),
            id: String::from("0008529D"),
            description: String::from("Harvested from Dryad's Saddel [sic] Polypore, a rare mushroom that only grows in a few select locations."),
            value: 10u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::ResistFrost), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::FrostDamage)],
        },
        Ingredient {
            name: String::from("Ectoplasm"),
            id: String::from("0001EBFE"),
            description: String::from("Collected from dead Ethereal Undead, such as ghosts and wraiths, which are commonly found in all undead dungeons."),
            value: 20u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::ShockDamage), Some(Effect::Dispel), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Elf Cup Cap"),
            id: String::from("0008529E"),
            description: String::from("Harvested from Elf Cups, a mushroom found growing sparsely in the West Weald."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::CureDisease), Some(Effect::FortifyAttribute(Attribute::Strength)), Some(Effect::DamageAttribute(Attribute::Intelligence))],
        },
        Ingredient {
            name: String::from("Emetic Russula Cap"),
            id: String::from("0008529F"),
            description: String::from("Harvested from Emetic Russula, a mushroom found growing sparsely in the West Weald."),
            value: 4u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::Shield), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Fennel Seeds"),
            id: String::from("000A7926"),
            description: String::from("Harvested from Fennel, a plant found primarily in the northern section of the Gold Coast."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Intelligence)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Paralyze)],
        },
        Ingredient {
            name: String::from("Fire Salts"),
            id: String::from("00033675"),
            description: String::from("Collected from dead Flame Atronachs, a type of Daedra common in the Planes of Oblivion."),
            value: 30u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::FireDamage), Some(Effect::ResistFrost), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FireShield)],
        },
        Ingredient {
            name: String::from("Flax Seeds"),
            id: String::from("000A7927"),
            description: String::from("Harvested from Flax, a common flower found in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Feather), Some(Effect::Shield), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Flour"),
            id: String::from("00033674"),
            description: String::from("Commonly found in grain sacks in kitchens and pantries, but not technically classified as food)."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ReflectDamage)],
        },
        Ingredient {
            name: String::from("Fly Amanita Cap"),
            id: String::from("00084471"),
            description: String::from("Harvested from Fly Amanita, a common mushroom in cities and in the Great Forest."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::Burden), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ShockDamage)],
        },
        Ingredient {
            name: String::from("Foxglove Nectar"),
            id: String::from("00033687"),
            description: String::from("Harvested from Foxglove, a common flower in certain parts of the Nibenay Valley and Nibenay Basin."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistPoison), Some(Effect::ResistParalysis), Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::ResistDisease)],
        },
        Ingredient {
            name: String::from("Frost Salts"),
            id: String::from("00022E5B"),
            description: String::from("Collected from dead Frost Atronachs, a type of Daedra common in the Planes of Oblivion."),
            value: 60u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::FrostDamage), Some(Effect::ResistFire), Some(Effect::Silence), Some(Effect::FrostShield)],
        },
        Ingredient {
            name: String::from("Garlic"),
            id: String::from("00033677"),
            description: String::from("Food, found in several houses and dining halls. It can also be harvested from Garlic Clusters (treated as plants), which are found in some houses' basements (especially in Skingrad)."),
            value: 1u16,
            weight: 0.0f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistDisease), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::FrostShield), Some(Effect::FortifyAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Ginkgo Leaf"),
            id: String::from("00033678"),
            description: String::from("Cannot be harvested or collected. There are a few guaranteed samples, but otherwise it is randomly found on alchemists and in some containers."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Speed)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::ShockDamage)],
        },
        Ingredient {
            name: String::from("Ginseng"),
            id: String::from("00033679"),
            description: String::from("Harvested from Ginseng Plants, flowers which growly sparsely in the Gold Coast and Nibenay Valley."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::CurePoison), Some(Effect::Burden), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Glow Dust"),
            id: String::from("0001EBE8"),
            description: String::from("Collected from dead Will-o-the-Wisps, a type of monster found outdoors and in monster dungeons."),
            value: 40u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Speed)), Some(Effect::Light), Some(Effect::ReflectSpell), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Grapes"),
            id: String::from("0003367B"),
            description: String::from("Harvested from Grape Vines, which are abundant in several vineyards outside of Skingrad. Can also be found as food in several houses and dining halls."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::WaterWalking), Some(Effect::Dispel), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Green Stain Cup Cap"),
            id: String::from("0008446A"),
            description: String::from("Harvested from Green Stain Cups, a mushroom that is common in several regions, but particularly abundant in Blackwood."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::ReflectDamage), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Green Stain Shelf Cap"),
            id: String::from("0008529A"),
            description: String::from("Harvested from Green Stain Shelf plants, a rare mushroom that is hard to distinguish from the more common Cup variety."),
            value: 10u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::FortifyAttribute(Attribute::Luck)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Ham"),
            id: String::from("0003367C"),
            description: String::from("Food, found in several houses and dining halls."),
            value: 1u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageAttribute(Attribute::Luck))],
        },
        Ingredient {
            name: String::from("Harrada"),
            id: String::from("0003367D"),
            description: String::from("Harvested from Harrada Root plants, which are common in the Planes of Oblivion, but also possibly hostile."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Silence), Some(Effect::Paralyze)],
        },
        Ingredient {
            name: String::from("Imp Gall"),
            id: String::from("0002EE72"),
            description: String::from("Collected from dead Imps, a type of monster found outdoors and in monster dungeons at low levels."),
            value: 15u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::FortifyAttribute(Attribute::Personality)), Some(Effect::CureParalysis), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FireDamage)],
        },
        Ingredient {
            name: String::from("Ironwood Nut"),
            id: String::from("0003367E"),
            description: String::from("Cannot be harvested, collected, or even bought from alchemists. There are 26 guaranteed samples; otherwise they are found randomly in some containers."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::ResistFire), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Lady's Mantle Leaves"),
            id: String::from("000A7928"),
            description: String::from("Harvested from Lady's Mantle, a flower found near Gottlesfont Priory and in the Gold Coast."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Endurance)), Some(Effect::NightEye), Some(Effect::Feather)],
        },
        Ingredient {
            name: String::from("Lady's Smock Leaves"),
            id: String::from("000A7929"),
            description: String::from("Harvested from Lady's Smock, a flower common in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::ResistFire), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Lavender Sprig"),
            id: String::from("000A792A"),
            description: String::from("Harvested from Lavender, a flower common in the Nibenay Basin."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Personality)), Some(Effect::FortifyAttribute(Attribute::Willpower)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Luck))],
        },
        Ingredient {
            name: String::from("Leek"),
            id: String::from("00033680"),
            description: String::from("Food, found in several houses and dining halls. It can also be harvested from Leek Plants, which are found growing in a handful of farms."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::FortifyAttribute(Attribute::Agility)), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Lettuce"),
            id: String::from("00033681"),
            description: String::from("Food, widely found in houses and dining halls. It can also be harvested from Lettuce Plants, which are found growing on several farms."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::FireShield), Some(Effect::DamageAttribute(Attribute::Personality))],
        },
        Ingredient {
            name: String::from("Lichor"),
            id: String::from("0007049E"),
            description: String::from("Harvested from bluish-purple colored Mana Bloom, a flower that is normally found only in Mankar Camoran's Paradise."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), None, None, None],
        },
        Ingredient {
            name: String::from("Mandrake Root"),
            id: String::from("00033683"),
            description: String::from("Harvested from Mandrake, a flower found growing in the Colovian Highlands and also near Bravil."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::CureDisease), Some(Effect::ResistPoison), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::FortifyAttribute(Attribute::Willpower))],
        },
        Ingredient {
            name: String::from("Milk Thistle Seeds"),
            id: String::from("000A792C"),
            description: String::from("Harvested from Milk Thistle, a flower found throughout mountainous regions (Jerall and Valus Mountains) and in some clusters in the Gold Coast."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::Light), Some(Effect::FrostDamage), Some(Effect::CureParalysis), Some(Effect::Paralyze)],
        },
        Ingredient {
            name: String::from("Minotaur Horn"),
            id: String::from("00033568"),
            description: String::from("Collected from dead Minotaurs and Minotaur Lords, two types of monster found outdoors and in monster dungeons."),
            value: 55u16,
            weight: 5.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Willpower)), Some(Effect::Burden), Some(Effect::FortifyAttribute(Attribute::Endurance)), Some(Effect::ResistParalysis)],
        },
        Ingredient {
            name: String::from("Monkshood Root Pulp"),
            id: String::from("000A792E"),
            description: String::from("Harvested from Monkshood, a common flower in the Great Forest and Nibenay Basin."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Strength)), Some(Effect::DamageAttribute(Attribute::Intelligence)), Some(Effect::FortifyAttribute(Attribute::Endurance)), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Morning Glory Root Pulp"),
            id: String::from("000A792F"),
            description: String::from("Harvested from Morning Glory, a flowering vine found growing on houses and walls in the West Weald and Heartlands."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::Burden), Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::FrostShield), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Mort Flesh"),
            id: String::from("00033685"),
            description: String::from("Collected from dead zombies, which are commonly found in all undead dungeons."),
            value: 10u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Motherwort Sprig"),
            id: String::from("000A7930"),
            description: String::from("Harvested from Motherwort, a flower common in most regions of Cyrodiil."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistPoison), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Silence), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Mugwort Seeds"),
            id: String::from("000A7931"),
            description: String::from("Cannot be harvested or collected. The only way to obtain them is by purchasing them from alchemy vendors, who may randomly have some in stock."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), None, None, None],
        },
        Ingredient {
            name: String::from("Mutton"),
            id: String::from("00033686"),
            description: String::from("Food, found in several houses and dining halls. Can also be collected from some (but not all) dead sheep."),
            value: 2u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Dispel), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Nightshade"),
            id: String::from("00033688"),
            description: String::from("Harvested from Nightshade Plants, a flower found in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Burden), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Ogre's Teeth"),
            id: String::from("00033689"),
            description: String::from("Collected from dead Ogres, a type of monster found outdoors and in monster dungeons."),
            value: 75u16,
            weight: 3.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Intelligence)), Some(Effect::ResistParalysis), Some(Effect::ShockDamage), Some(Effect::FortifyAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Onion"),
            id: String::from("0003368A"),
            description: String::from("Food, found in many houses and dining halls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::WaterBreathing), Some(Effect::DetectLife), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Orange"),
            id: String::from("0007588E"),
            description: String::from("Food, found in many houses and dining halls."),
            value: 2u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DetectLife), Some(Effect::Burden), Some(Effect::Shield)],
        },
        Ingredient {
            name: String::from("Pear"),
            id: String::from("0003368B"),
            description: String::from("Food, found in many houses and dining halls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::FortifyAttribute(Attribute::Speed)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Peony Seeds"),
            id: String::from("000A7932"),
            description: String::from("Harvested from Peonies, flowers common in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Strength)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue))],
        },
        Ingredient {
            name: String::from("Potato"),
            id: String::from("0003368C"),
            description: String::from("Food, widely found in houses and dining halls. Can also be harvested from Potato Plants, which are found growing on several farms."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Shield), Some(Effect::Burden), Some(Effect::FrostShield)],
        },
        Ingredient {
            name: String::from("Primrose Leaves"),
            id: String::from("000A7934"),
            description: String::from("Harvested from Primroses, which are flowers occasionally found growing outside various houses and inns."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Willpower)), Some(Effect::RestoreAttribute(Attribute::Personality)), Some(Effect::FortifyAttribute(Attribute::Luck)), Some(Effect::DamageAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Pumpkin"),
            id: String::from("0003368D"),
            description: String::from("Food, widely found in houses and dining halls. Can also be harvested from Pumpkin Vines, which are found growing on some farms."),
            value: 2u16,
            weight: 5.0f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DetectLife)],
        },
        Ingredient {
            name: String::from("Purgeblood SaltsVL"),
            id: String::from("xx001372"),
            description: String::from("Mined from Purgeblood Crystal Formations, which are only available as part of the Vile Lair download."),
            value: 0u16,
            weight: 0.4f32,
            harvest_chance: 100.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("Radish"),
            id: String::from("0003368E"),
            description: String::from("Food, found in many houses and dining halls. Can also be harvested from Radish Plants, which are found growing on several farms."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Endurance)), Some(Effect::Chameleon), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Rat Meat"),
            id: String::from("0003368F"),
            description: String::from("Collected from dead rats, a ubiquitous nuisance creature."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DetectLife), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Redwort Flower"),
            id: String::from("0002503A"),
            description: String::from("Harvested from Domica Redwort, a flower found growing in several clusters in Blackwood and sparsely throughout the West Weald."),
            value: 4u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistFrost), Some(Effect::CurePoison), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Rice"),
            id: String::from("00033690"),
            description: String::from("Food, generally found in grain sacks in kitchens and pantries."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Silence), Some(Effect::ShockShield), Some(Effect::DamageAttribute(Attribute::Agility))],
        },
        Ingredient {
            name: String::from("Root Pulp"),
            id: String::from("00033691"),
            description: String::from("Cannot be harvested or collected. Six guaranteed samples exist; otherwise found randomly in some containers."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::CureDisease), Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::FortifyAttribute(Attribute::Strength)), Some(Effect::DamageAttribute(Attribute::Intelligence))],
        },
        Ingredient {
            name: String::from("Sacred Lotus Seeds"),
            id: String::from("000A7936"),
            description: String::from("Harvested from Sacred Lotus plants, which are flowers that grow in shallow water."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistFrost), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Feather), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("Scales"),
            id: String::from("00033692"),
            description: String::from("Collected from dead slaughterfish, an animal found in deep water, including pools in dungeons."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::WaterBreathing), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::WaterWalking)],
        },
        Ingredient {
            name: String::from("Scamp Skin"),
            id: String::from("00033693"),
            description: String::from("Collected from dead Scamps, a type of Daedra common in the Planes of Oblivion."),
            value: 10u16,
            weight: 1.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::ResistShock), Some(Effect::ReflectDamage), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Shepherd's Pie"),
            id: String::from("000B97E9"),
            description: String::from("Food, found in a few houses and dining halls. Also available from Eyja in Rosethorn Hall."),
            value: 0u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::CureDisease), Some(Effect::Shield), Some(Effect::FortifyAttribute(Attribute::Agility)), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("S'jirra's Famous Potato Bread"),
            id: String::from("00177A29"),
            description: String::from("Only available from S'jirra in Faregyl Inn after you have completed The Potato Snatcher. You receive from one to five loaves (based on level) upon completing the quest. You can also return to S'jirra repeatedly to buy more bread, which can then be used for alchemy."),
            value: 30u16,
            weight: 0.5f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DetectLife), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::DamageAttribute(Attribute::Strength))],
        },
        Ingredient {
            name: String::from("Somnalius Frond"),
            id: String::from("00033696"),
            description: String::from("Harvested from Somnalius Plants, which are found growing in the Great Forest."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Speed)), Some(Effect::DamageAttribute(Attribute::Endurance)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Feather)],
        },
        Ingredient {
            name: String::from("Spiddal Stick"),
            id: String::from("00033697"),
            description: String::from("Harvested from Spiddal Stick plants, which are common in the Planes of Oblivion, but also tend to be hostile."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FireDamage), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue))],
        },
        Ingredient {
            name: String::from("St. Jahn's Wort Nectar"),
            id: String::from("000A7939"),
            description: String::from("Harvested from St. Jahn's Wort, a flower most commonly found in the Great Forest."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistShock), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::CurePoison), Some(Effect::Chameleon)],
        },
        Ingredient {
            name: String::from("Steel-Blue Entoloma Cap"),
            id: String::from("0008446B"),
            description: String::from("Harvested from Steel-Blue Entoloma, a mushroom that is common in the Great Forest."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FireDamage), Some(Effect::ResistFrost), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Stinkhorn Cap"),
            id: String::from("0008446D"),
            description: String::from("Harvested from Stinkhorn, a mushroom that is found in Blackwood."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::WaterWalking), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Strawberry"),
            id: String::from("00033699"),
            description: String::from("Food, widely found in houses and dining halls. It can also be harvested from Strawberry Bushes found on some farms and in the wild in the West Weald."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::CurePoison), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ReflectDamage)],
        },
        Ingredient {
            name: String::from("Summer Bolete Cap"),
            id: String::from("00084470"),
            description: String::from("Harvested from Summer Bolete, a mushroom found in the Great Forest and Nibenay Valley."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::Shield), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Sweetcake"),
            id: String::from("0003369A"),
            description: String::from("Food, found in a few houses and dining halls. Sweetcakes are less common than Sweetrolls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Feather), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Sweetroll"),
            id: String::from("0003369B"),
            description: String::from("Food, found in several houses and dining halls."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistDisease), Some(Effect::DamageAttribute(Attribute::Personality)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Taproot"),
            id: String::from("000AF06E"),
            description: String::from("Collected from dead Spriggans, a type of monster found outdoors and in monster dungeons."),
            value: 1u16,
            weight: 0.3f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::DamageAttribute(Attribute::Endurance)), Some(Effect::ResistPoison), Some(Effect::ShockShield)],
        },
        Ingredient {
            name: String::from("Tiger Lily Nectar"),
            id: String::from("000A792B"),
            description: String::from("Harvested from both Tiger Lilies and Lily of the Valleys, two types of flower that are most commonly found in the Great Forest."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Endurance)), Some(Effect::DamageAttribute(Attribute::Strength)), Some(Effect::WaterWalking), Some(Effect::DamageAttribute(Attribute::Willpower))],
        },
        Ingredient {
            name: String::from("Tinder Polypore Cap"),
            id: String::from("0008446F"),
            description: String::from("Harvested from Tinder Polypore, a mushroom common in the Valus Mountains."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Willpower)), Some(Effect::ResistDisease), Some(Effect::Invisibility), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Tobacco"),
            id: String::from("0003369D"),
            description: String::from("Technically treated as a food and can be found in a few houses. It can also be harvested from Tobacco Plants, which are found on several farms."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::ResistParalysis), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("Tomato"),
            id: String::from("0003369E"),
            description: String::from("Food, widely found in houses and dining halls. It can also be harvested from Tomato Plants, which are found growing on many farms."),
            value: 2u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DetectLife), Some(Effect::Burden), Some(Effect::Shield)],
        },
        Ingredient {
            name: String::from("Troll Fat"),
            id: String::from("00026B5C"),
            description: String::from("Collected from dead Trolls, a type of monster found outdoors and in monster dungeons."),
            value: 25u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Agility)), Some(Effect::FortifyAttribute(Attribute::Personality)), Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Vampire Dust*"),
            id: String::from("00009182"),
            description: String::from("Collected from dead Vampires, found in a range of vampire dungeons. There is also a rare variant listed below."),
            value: 50u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::Silence), Some(Effect::ResistDisease), Some(Effect::FrostDamage), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Venison"),
            id: String::from("0002229B"),
            description: String::from("Food, found in several houses and dining halls. It can also be collected from dead deer, which are animals found outdoors throughout Cyrodiil."),
            value: 1u16,
            weight: 2.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Feather), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Chameleon)],
        },
        Ingredient {
            name: String::from("Viper's Bugloss Leaves"),
            id: String::from("000A793B"),
            description: String::from("Harvested from Viper's Bugloss, a flower which is abundant in the Great Forest."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::ResistParalysis), Some(Effect::NightEye), Some(Effect::Burden), Some(Effect::CureParalysis)],
        },
        Ingredient {
            name: String::from("Void Salts"),
            id: String::from("0003369F"),
            description: String::from("Collected from dead Storm Atronachs, a type of Daedra common in the Planes of Oblivion."),
            value: 30u16,
            weight: 0.4f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Dispel)],
        },
        Ingredient {
            name: String::from("Water Hyacinth Nectar"),
            id: String::from("000A793C"),
            description: String::from("Harvested from Water Hyacinth, a flower found growing in shallow water."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Watermelon"),
            id: String::from("000336A0"),
            description: String::from("Food, found in several houses and dining halls. It can also be harvested from Watermelon Vines, which grow on a few farms."),
            value: 2u16,
            weight: 5.0f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Light), Some(Effect::Burden), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Wheat Grain"),
            id: String::from("000336A1"),
            description: String::from("Food, found in grain sacks in kitchens and pantries. It can also be harvested from Wheat Stalks, which grow on a few farms."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Personality))],
        },
        Ingredient {
            name: String::from("White Seed Pod"),
            id: String::from("000336A2"),
            description: String::from("Harvested from Goldenrod, a flower found in the Gold Coast."),
            value: 5u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Strength)), Some(Effect::WaterBreathing), Some(Effect::Silence), Some(Effect::Light)],
        },
        Ingredient {
            name: String::from("Wisp Stalk Caps"),
            id: String::from("0006251F"),
            description: String::from("Harvested from Wisp Stalks, a mushroom that is only found growing underground (i.e., in caves and mines)."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Willpower)), Some(Effect::DamageAttribute(Attribute::Intelligence)), Some(Effect::FortifyAttribute(Attribute::Speed))],
        },
        Ingredient {
            name: String::from("Wormwood Leaves"),
            id: String::from("000A793E"),
            description: String::from("Harvested from Wormwood, a plant found growing in the Jerall Mountains."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Invisibility), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        // Shivering Isles
        Ingredient {
            name: String::from("Alocasia Fruit"),
            id: String::from("0001AE7A"),
            description: String::from("Harvested from Alocasia plants, found in Mania."),
            value: 1u16,
            weight: 0.25f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Light), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Ashen Remains"),
            id: String::from("00081A68"),
            description: String::from("Created from bones using the Crematorium in Ebrocca."),
            value: 5u16,
            weight: 0.8f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::Silence), Some(Effect::WeaknessToFire)],
        },
        Ingredient {
            name: String::from("Aster Bloom Core"),
            id: String::from("0001B035"),
            description: String::from("Harvested from Aster Bloom plants, found in Mania."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Agility)), Some(Effect::Dispel), Some(Effect::Shield), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Black Tar"),
            id: String::from("0003BABD"),
            description: String::from("Harvested from Dementia variety Mushroom Tree Saplings and from Black Tar plants."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageAttribute(Attribute::Speed)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ShockDamage)],
        },
        Ingredient {
            name: String::from("Blister Pod Cap"),
            id: String::from("0001F3AA"),
            description: String::from("Harvested from Blister Pod plants, which are found in Dementia."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 66.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::NightEye), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Bone Marrow"),
            id: String::from("00044E61"),
            description: String::from("Found on Shambles (50% chance they may yield Bone Shard instead). Also called \"Shambles Marrow.\""),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FrostDamage), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Paralyze)],
        },
        Ingredient {
            name: String::from("Bone Shard"),
            id: String::from("00052443"),
            description: String::from("Found on Shambles (50% chance they may yield Bone Marrow instead)."),
            value: 1u16,
            weight: 0.3f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Willpower)), Some(Effect::FrostShield), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageAttribute(Attribute::Luck))],
        },
        Ingredient {
            name: String::from("Congealed Putrescence"),
            id: String::from("000402E9"),
            description: String::from("Harvested from Putrid Gigantea plants, found in Dementia."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 66.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Strength)), Some(Effect::FireDamage), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Elytra Ichor"),
            id: String::from("00044E5D"),
            description: String::from("Found on Elytras."),
            value: 3u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Burden), Some(Effect::Chameleon), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Flame Stalk 0001B6AE /"),
            id: String::from("00097613"),
            description: String::from("Harvested from Flame Stalk plants, found in Mania caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FireDamage), Some(Effect::FrostShield), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Fungus Stalk"),
            id: String::from("00019529"),
            description: String::from("Harvested from Fungus Stalk plants, found in Dementia."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Strength)), Some(Effect::WaterWalking), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Gnarl Bark"),
            id: String::from("00044E5E"),
            description: String::from("Found on Gnarls."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Endurance)), Some(Effect::Shield), Some(Effect::FireShield), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Grummite Eggs"),
            id: String::from("0001AE82"),
            description: String::from("Harvested from Grummite Egg Mounds and Grummite Egg Sacs, found primarily in Dementia. May also be found on Grummites (25% chance)."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Dispel), Some(Effect::Chameleon), Some(Effect::Silence)],
        },
        Ingredient {
            name: String::from("Hound Tooth"),
            id: String::from("00044E62"),
            description: String::from("Found on Skinned Hounds."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::CurePoison), Some(Effect::DetectLife), Some(Effect::Burden), Some(Effect::Invisibility)],
        },
        Ingredient {
            name: String::from("Hunger Tongue"),
            id: String::from("00044E60"),
            description: String::from("Found on Hungers."),
            value: 4u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::CurePoison), Some(Effect::CureDisease), Some(Effect::FireDamage), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Hydnum Azure Giant Spore"),
            id: String::from("0003BAD3"),
            description: String::from("Harvested from Hydnum Azure plants, found in Mania."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 66.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Endurance)), Some(Effect::DetectLife), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FrostShield)],
        },
        Ingredient {
            name: String::from("Letifer Orca Digestive Slime"),
            id: String::from("0003E632"),
            description: String::from("Harvested from Letifer Orca Planta, found in Dementia. Also called simply \"Digestive Slime.\""),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 66.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue))],
        },
        Ingredient {
            name: String::from("Red Kelp Gas Bladder"),
            id: String::from("0001A16E"),
            description: String::from("Harvested from Red Kelp, found in Mania. Also called simply \"Gas Bladder.\""),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 50.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Speed)), Some(Effect::WaterBreathing), Some(Effect::CureDisease), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka))],
        },
        Ingredient {
            name: String::from("Rot Scale"),
            id: String::from("0001B6B2"),
            description: String::from("Harvested from Rot Scale, found in Dementia caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::Burden), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::Silence), Some(Effect::Paralyze)],
        },
        Ingredient {
            name: String::from("Scalon Fin"),
            id: String::from("00044E5F"),
            description: String::from("Found on Scalons."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::WaterBreathing), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ShockDamage), Some(Effect::Burden)],
        },
        Ingredient {
            name: String::from("Screaming Maw"),
            id: String::from("0001B6B0"),
            description: String::from("Harvested from Screaming Maw plants, found in Mania caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 66.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Willpower)), Some(Effect::DetectLife), Some(Effect::Chameleon), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Smoked Baliwog Leg"),
            id: String::from("0007D048"),
            description: String::from("Found as food in shops and houses."),
            value: 2u16,
            weight: 1.0f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Feather), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageDerivedAttribute(DerivedAttribute::Fatigue))],
        },
        Ingredient {
            name: String::from("Swamp Tentacle"),
            id: String::from("000413DC"),
            description: String::from("Harvested from Swamp Tentacle plants, found in Dementia."),
            value: 2u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Personality)), Some(Effect::WaterBreathing), Some(Effect::WaterWalking), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Thorn Hook"),
            id: String::from("0001B6B1"),
            description: String::from("Harvested from Thorn Hook plants, found in Dementia caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::DamageDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DamageAttribute(Attribute::Luck)), Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health))],
        },
        Ingredient {
            name: String::from("Unrefined Greenmote"),
            id: String::from("00077AD9"),
            description: String::from("Harvested from Mania variety Mushroom Tree Saplings."),
            value: 1u16,
            weight: 0.5f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::DrainDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::DrainDerivedAttribute(DerivedAttribute::Health)), Some(Effect::DrainDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::DrainAttribute(Attribute::Intelligence))],
        },
        Ingredient {
            name: String::from("Void Essence"),
            id: String::from("00044E5C"),
            description: String::from("Found on Flesh Atronachs."),
            value: 50u16,
            weight: 0.2f32,
            harvest_chance: 0.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Health)), Some(Effect::FortifyAttribute(Attribute::Strength)), Some(Effect::FortifyAttribute(Attribute::Endurance))],
        },
        Ingredient {
            name: String::from("Watcher's Eye"),
            id: String::from("0001B6AF"),
            description: String::from("Harvested from Watcher's Eye plants, which grow in Dementia caves."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Light), Some(Effect::ReflectSpell)],
        },
        Ingredient {
            name: String::from("Water Root Pod Pit"),
            id: String::from("00040CAF"),
            description: String::from("Harvested from Water Root Pod plants, which grow in Dementia. Also called simply \"Pod Pit.\""),
            value: 3u16,
            weight: 1.0f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Health)), Some(Effect::ResistFire), Some(Effect::FireShield), Some(Effect::WaterBreathing)],
        },
        Ingredient {
            name: String::from("Wisp Core"),
            id: String::from("0007D04E"),
            description: String::from("Harvested from Root Stalks, which grow in Dementia and Mania caves."),
            value: 2u16,
            weight: 0.1f32,
            harvest_chance: 25.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Intelligence)), Some(Effect::Burden), Some(Effect::Light), Some(Effect::Chameleon)],
        },
        Ingredient {
            name: String::from("Withering Moon"),
            id: String::from("0001B6B3"),
            description: String::from("Harvested from Withering Moon plants, which grow in Dementia caves."),
            value: 1u16,
            weight: 0.1f32,
            harvest_chance: 33.0f32,
        effects: [Some(Effect::RestoreDerivedAttribute(DerivedAttribute::Magicka)), Some(Effect::Shield), Some(Effect::CureDisease), Some(Effect::ReflectSpell)],
        },
        Ingredient {
            name: String::from("Worm's Head Cap"),
            id: String::from("0001B5AA"),
            description: String::from("Harvested from Worm's Head plants, which grow in Mania."),
            value: 1u16,
            weight: 0.2f32,
            harvest_chance: 80.0f32,
        effects: [Some(Effect::RestoreAttribute(Attribute::Luck)), Some(Effect::NightEye), Some(Effect::FortifyDerivedAttribute(DerivedAttribute::Fatigue)), Some(Effect::Paralyze)],
        },
    ]
}
