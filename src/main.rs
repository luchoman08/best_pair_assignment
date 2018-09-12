mod pair_assignment;
extern crate lp_modeler;
fn main() {
    use pair_assignment::*;
    pair_assignment::solve();
    let mut skills: Skills = Skills::new();
    let mut skills2: Skills = Skills::new();
    skills.insert(0,100.0);
    skills2.insert(0,12.0);
    let agent: Agent = Agent::new(0, skills);
    let agent2: Agent = Agent::new(1, skills2);
    let agents = vec![&agent, &agent2];
    match Agent::get_standard_agent(&agents) {
        Ok(x) => println!("Habilidad 1 {}", x.get_skills().get(&(0 as u16)).unwrap_or(&(0.0 as SkillValue))),
        Err(_) => println!("Nel mijo")
    }

    println!("Compatibilidad {}", Agent::get_compatibility(&agent, &agent2));
}
