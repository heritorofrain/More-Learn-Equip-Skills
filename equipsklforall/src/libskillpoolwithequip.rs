
use engage_il2cpp::app::force;
use unity2::prelude::*;
use engage_il2cpp::app::Unit;
use engage_il2cpp::app::IUnit;
use engage_il2cpp::app::IUnitMethods;
use engage_il2cpp::app::GameSound;
use engage_il2cpp::app::ISkillArrayMethods;
use engage_il2cpp::app::ISkillDataMethods;
use engage_il2cpp::app::GameMessage;
use engage_il2cpp::app::UnitGrowSequence;
use engage_il2cpp::app::IUnitGrowSequence;
use engage_il2cpp::app::IJobDataMethods;
use engage_il2cpp::app::IPersonDataMethods;
use engage_il2cpp::combat::Character;
use engage_il2cpp::prelude::FromIlInstance;
use engage_il2cpp::app::SkillData_Categorys;
use engage_il2cpp::app::Mess;
use engage_il2cpp::app::persondata::PersonData;
use engage_il2cpp::app::JobData;
use engage_il2cpp::unity_engine::random::Random;

use std::path::Path;
use std::fs::read_to_string;

const SKILL_LEVEL: [u8; 9] = [0, 5, 10, 15, 20, 25, 30, 35, 40];


#[unity2::hook("App", "UnitGrowSequence", "LevelUp")]
pub fn levelup_checknewequipskills(this: UnitGrowSequence, method_info: OptionalMethod) {
    call_original!(this, method_info);

    let unit = this.m_unit();
    let path = Path::new("sd:/engage/config/moreequipskills/equipskills.txt");
    let new_equip_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = unit.m_job().get_name().to_string();
    if let Some(start_bytes) = new_equip_skills.find(&unit_jid) {
        let start_bytes_0 = start_bytes + unit_jid.len();
        if let Some(end_bytes) = new_equip_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_equip_skills = &new_equip_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.m_level();
            if unit.m_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl{
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_equip_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_equip_skills.find(&skill_end) {
                                let equip_skill_lvl = &job_equip_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if equip_skill_lvl == "" {break 'check_lvl};
                                if unit.m_equip_skill_pool().test(equip_skill_lvl) {break 'check_lvl};
                                println!("try to equip skill");
                                unit.m_equip_skill_pool().add(equip_skill_lvl, SkillData_Categorys::job(), 0);
                                println!("added to pool");
                                unit.add_equip_skill(equip_skill_lvl);
                                learn_message(this, unit, equip_skill_lvl.into());
                            };
                        };
                    };
                };
            };
        };
    };
}

#[unity2::hook("App", "Unit", "CreateImpl1")]
pub fn create_equipskills(this: Unit, person: PersonData, job: JobData, level: i32, random: Random, method_info: OptionalMethod) {
    call_original!(this, person, job, level, random, method_info);
    let path = Path::new("sd:/engage/config/moreequipskills/equipskills.txt");
    let new_equip_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = this.m_job().get_name().to_string();
    if let Some(start_bytes) = new_equip_skills.find(&unit_jid) {
        let start_bytes_0 = start_bytes + unit_jid.len();
        if let Some(end_bytes) = new_equip_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_equip_skills = &new_equip_skills[start_bytes_0..end_bytes_0];
            let mut current_level = this.m_level();
            if this.m_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_equip_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_equip_skills.find(&skill_end) {
                                println!("starting function execution");
                                let equip_skill_lvl = &job_equip_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if equip_skill_lvl == "" {break 'check_lvl};
                                if person.get_common_skills().test("SID_Spawn_Equip") {
                                if this.m_equip_skill_pool().test(equip_skill_lvl) {break 'check_lvl};
                                println!("try to spawn with skill");
                                this.m_equip_skill_pool().add(equip_skill_lvl, SkillData_Categorys::job(), 0);
                                println!("added to pool");
                                this.add_equip_skill(equip_skill_lvl);
                                println!("trying to equip skill");
                            };
                        };
                    };
                };
            };
        };
    };
}
}

pub fn learn_message(this: UnitGrowSequence, this_unit: Unit, sid: &str) {
    println!("get unit name");
    let name = this_unit.get_name().to_string();
    println!("get skill data");
    let current = this_unit.m_equip_skill_pool().find(sid);
    println!("get skill name");
    let current_name = Mess::get(current.get_name()).to_string();

    let message = format!("{name} learnt {current_name}");
    println!("instantiate character");
    let thing = Character::instantiate().unwrap();
    println!("creating event");
    GameSound::post_event("ItemGet_Important", thing);
    GameMessage::create_key_wait(this, message);
}

#[skyline::main(name = "equipskl")]
pub fn main() {
    skyline::install_hooks!(levelup_checknewequipskills, create_equipskills);
}
