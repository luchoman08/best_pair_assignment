
use lp_modeler::problem::{LpObjective, LpProblem};
use lp_modeler::operations::{LpOperations};
use lp_modeler::variables::LpInteger;
use lp_modeler::solvers::{SolverTrait, CbcSolver};

use std::collections::HashMap;
use std::cmp::{max, min};
pub const DEFAULT_ID: u32 = 854554;
pub const DEFAULT_SKILL_ID: i8 = -1;
pub const MINIMUM_SKILL_VALUE: u8 = 1;
pub struct Agent {
    id: u32,
    skills: Skills,
}
pub type SkillValue = f32;
pub type Skills = HashMap<u16, SkillValue>;
pub enum ErrorKind{
    ToFewAgents
}
trait Average {
    fn average(&self) -> SkillValue ;
}


impl Average for Vec<SkillValue> {
    fn average(&self) -> SkillValue {
        let mut average: SkillValue = 0.0;
        for  value in self.iter() {
            average +=  value;
        }
        average / self.len() as SkillValue
    }

}

impl Agent {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_skills(&self) -> &Skills {
        &self.skills
    }
    pub fn new(id: u32, skills: Skills) -> Agent {
        Agent {id: id, skills: skills}
    }

    fn skill_average(agents: &Vec<&Agent>, id_skill: &u16) -> SkillValue {
        let mut skill_media_punctuation = 0.0 as SkillValue;
        for agent in agents.iter() {
            skill_media_punctuation += agent.skills.get(id_skill).unwrap_or(&(0.0 as SkillValue));
        }
        skill_media_punctuation
    }
    pub fn get_standard_agent(agents: &Vec<&Agent>) -> Result<Agent, ErrorKind> {
        match agents.len()  {
            0 => Err(ErrorKind::ToFewAgents),
            1 => Ok(Agent::new(DEFAULT_ID, agents[0].skills.clone())),
            _ =>
            {
                let id_skills = agents[0].skills.keys();
                let agent_quantity = agents.len() as SkillValue;
                let mut skills_media = Skills::new();
                for id_skill in id_skills {
                    let skill_media_punctuation: SkillValue = Agent::skill_average(agents, &id_skill);
                    skills_media.insert(id_skill.clone() as u16, skill_media_punctuation / agent_quantity);
                }
                return Ok(Agent::new(DEFAULT_ID, skills_media));
            }
        }
    }
    pub fn get_little_skillfull_agent(sample_skills: &Skills) -> Agent {
        let mut low_skills = Skills::new();
        for id_skill in sample_skills.keys() {
            low_skills.insert(id_skill.clone(), MINIMUM_SKILL_VALUE as SkillValue);
        }
        Agent::new(DEFAULT_ID, low_skills)
    }
    pub fn get_compatibility (agent1: &Agent, agent2: &Agent)-> SkillValue {

        let mut compatibility: SkillValue = 0.0;
        let skills_quantity = agent1.skills.len();
        let importance_percentage_per_skill = 1.0 / skills_quantity as SkillValue;
        let compatibility_func = |skill_value1: &SkillValue, skill_value2: &SkillValue| skill_value1.min(*skill_value2) * 100 as SkillValue / skill_value1.max(*skill_value2);
        for id_skill in agent1.skills.keys() {
            compatibility += importance_percentage_per_skill
                * compatibility_func(
                    agent1.skills.get(id_skill)
                        .unwrap_or(&(0.0 as SkillValue)),
                    agent2.skills.get(id_skill)
                    .unwrap_or(&(0.0 as SkillValue)));
        }
        compatibility
    }
}
pub fn solve() {
    println!("Hello, second");
}


/*
pub fn solve() {


let ref a = LpInteger::new("a");
let ref b = LpInteger::new("b");
let ref c = LpInteger::new("c");

let mut problem = LpProblem::new("One Problem", LpObjective::Maximize);

// Maximize 10*a + 20*b
problem += 10.0 * a + 20.0 * b;

// 500*a + 1200*b + 1500*c <= 10000
problem += (500*a + 1200*b + 1500*c).le(10000);
// a <= b
problem += (a).le(b);

let solver = CbcSolver::new();

match solver.run(&problem) {
Ok((status, res)) => {
    println!("Status {:?}", status);
        for (name, value) in res.iter() {
            println!("value of {} = {}", name, value);
        }
    },
    Err(msg) => println!("{}", msg),
}
}
*/
