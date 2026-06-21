#![feature(lazy_cell, ptr_sub_ptr)]

use unity2::prelude::*;
use engage_il2cpp::app::Unit;
use engage_il2cpp::app::IUnit;
use engage_il2cpp::app::IUnitMethods;
use engage_il2cpp::app::GameSound;
use engage_il2cpp::app::SkillArray;
use engage_il2cpp::app::ISkillArray;
use engage_il2cpp::app::ISkillArrayMethods;
use engage_il2cpp::app::ISkillData;
use engage_il2cpp::app::ISkillDataMethods;
use engage_il2cpp::app::GameMessage;
use engage_il2cpp::app::UnitGrowSequence;
use engage_il2cpp::app::IUnitGrowSequence;
use engage_il2cpp::app::IJobData;
use engage_il2cpp::app::IJobDataMethods;
use engage_il2cpp::app::IPersonDataMethods;
use engage_il2cpp::combat::Character;
use engage_il2cpp::prelude::FromIlInstance;
use engage_il2cpp::app::SkillData_Categorys;
use engage_il2cpp::app::Mess;
use engage_il2cpp::app::PersonData;
use engage_il2cpp::app::JobData;
use engage_il2cpp::unity_engine::Random;
use engage_il2cpp::app::IForceMethods;
use engage_il2cpp::app::GameUserData;
use engage_il2cpp::app::IGameUserData;
use engage_il2cpp::app::Difficulty;


use std::path::Path;
use std::fs::read_to_string;

const SKILL_LEVEL: [u8; 9] = [0, 5, 10, 15, 20, 25, 30, 35, 40];


#[unity2::hook("App", "UnitGrowSequence", "LevelUp")]
pub fn levelup_checknewlearnskills(this: UnitGrowSequence, method_info: OptionalMethod) {
    call_original!(this, method_info);

    let unit = this.m_unit();
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = unit.m_job().get_name().to_string();
    if let Some(start_bytes) = new_learn_skills.find(&unit_jid) {
        let start_bytes_0 = start_bytes + unit_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.m_level();
            if unit.m_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.m_private_skill().test(learn_skill_lvl) {break 'check_lvl};
                                unit.m_private_skill().add(learn_skill_lvl, SkillData_Categorys::job(), 0);
                                learn_message(this, unit, learn_skill_lvl);
                            };
                        };
                    };
                };
            };
        };
    };
}

pub fn learn_message(this: UnitGrowSequence, this_unit: Unit, sid: &str) {
    let name = this_unit.get_name().to_string();
    let current = this_unit.m_private_skill().find(sid);
    let current_name = Mess::get(current.get_name()).to_string();

    let message = format!("{name} learnt {current_name}");
    let thing = Character::instantiate().unwrap();
    GameSound::post_event("ItemGet_Important", thing);
    GameMessage::create_key_wait(this, message);
}

#[unity2::hook("App", "UnitGrowSequence", "ClassChange")]
pub fn classchange_clearandaddlearnskills(this: UnitGrowSequence, method_info: OptionalMethod) {
    let old_jid = this.m_unit().m_job().get_jid().to_string();
    let new_jid = this.m_class_change_job().get_jid().to_string();
    let unit = this.m_unit();
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");
    if let Some(old_start_bytes) = new_learn_skills.find(&old_jid) {
        let start_bytes_0 = old_start_bytes + old_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.m_level();
            if unit.get_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.m_private_skill().test(learn_skill_lvl) {
                                    unit.m_private_skill().remove(learn_skill_lvl);
                                };
                            };
                        };
                    };
                };
            };
        };
    };

    call_original!(this, method_info);

    if let Some(new_start_bytes) = new_learn_skills.find(&new_jid) {
        let start_bytes_0 = new_start_bytes + new_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.m_level();
            if unit.get_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.m_private_skill().test(learn_skill_lvl) {break 'check_lvl};
                                unit.m_private_skill().add(learn_skill_lvl, SkillData_Categorys::job(), 0);
                                learn_message(this, unit, learn_skill_lvl);
                            };
                        };
                    };
                };
            };
        };
    };
}

#[unity2::hook("App", "Unit", "CreateImpl1")]
pub fn create_learnskills(this: Unit, person: PersonData, job: JobData, level: i32, random: Random, method_info: OptionalMethod) {
    call_original!(this, person, job, level, random, method_info);
    
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = this.m_job().get_jid().to_string();
    if let Some(start_bytes) = new_learn_skills.find(&unit_jid) {
        let start_bytes_0 = start_bytes + unit_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = level;
            if this.m_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl.into() {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if person.get_common_skills().test(learn_skill_lvl) {break 'check_lvl};
                                let difficulty = GameUserData::instantiate().expect("REASON").m_difficulty();
                                if difficulty == Difficulty::normal() {
                                if person.get_normal_skills().test(learn_skill_lvl) {break 'check_lvl};
                                };
                                if difficulty == Difficulty::hard() {
                                if person.get_hard_skills().test(learn_skill_lvl) {break 'check_lvl};
                                };
                                if difficulty == Difficulty::lunatic() {
                                if person.get_lunatic_skills().test(learn_skill_lvl) {break 'check_lvl};
                                };
                                this.m_private_skill().add(learn_skill_lvl, SkillData_Categorys::job(), 0);
                            };
                        };
                    };
                };
            };
        };
    };
}

#[skyline::main(name = "learnskl")]
pub fn main() {
    skyline::install_hooks!(levelup_checknewlearnskills, classchange_clearandaddlearnskills, create_learnskills);

}