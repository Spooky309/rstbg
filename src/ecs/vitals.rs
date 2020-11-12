use crate::ecs::constants::NUMENTS;

pub struct VitalsComponent {
    healths: [i64; NUMENTS],
    maxhealths: [i64; NUMENTS],
    occupied: [bool; NUMENTS],
}

impl VitalsComponent {
    pub fn new() -> VitalsComponent {
        VitalsComponent {
            healths: [0; NUMENTS],
            maxhealths: [0; NUMENTS],
            occupied: [false; NUMENTS]
        }
    }
    pub fn attachnew(&mut self, guid: usize, maxhp: i64, inithp: i64) {
        if self.occupied[guid] == true {
            eprintln!("Tried to make a new vital, but guid {} already has one! Entities can only have one vital!", guid);
            return
        }
        self.healths[guid] = inithp;
        self.maxhealths[guid] = maxhp;
        self.occupied[guid] = true;
    }
    pub fn set_health(&mut self, guid: usize, newh: i64) -> i64 {
        if guid == 0 { return 0; }
        self.healths[guid] = newh;
        self.clamp_to_max(guid)
    }
    pub fn get_health(&mut self, guid: usize) -> i64 {
        if guid == 0 { return 0; }
        self.healths[guid]
    }
    pub fn modify_health(&mut self, guid: usize, delta: i64) -> i64 {
        if guid == 0 { return 0; }
        self.healths[guid] += delta;
        self.clamp_to_max(guid)
    }
    pub fn set_max_health(&mut self, guid: usize, newh: i64) {
        if guid == 0 { return; }
        self.maxhealths[guid] = newh;
        self.clamp_to_max(guid);
    }
    pub fn get_max_health(&mut self, guid: usize) -> i64 {
        if guid == 0 { return 0; }
        self.maxhealths[guid]
    }
    pub fn modify_max_health(&mut self, guid: usize, delta: i64) -> i64 {
        if guid == 0 { return 0; }
        self.maxhealths[guid] += delta;
        self.clamp_to_max(guid);
        self.maxhealths[guid]
    }
    fn clamp_to_max(&mut self, guid: usize) -> i64 {
        if self.healths[guid] > self.maxhealths[guid] {
            self.healths[guid] = self.maxhealths[guid];
        }
        self.healths[guid]
    }
}