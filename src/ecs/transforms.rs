use crate::ecs::constants::NUMENTS;

pub struct Transforms {
    localpositions: [[f32; 2]; NUMENTS],
    localangles: [f32; NUMENTS],
    localscales: [[f32; 2]; NUMENTS],
    parents: [usize; NUMENTS],
    globalpositions: [[f32; 2]; NUMENTS],
    globalangles: [f32; NUMENTS],
    globalscales: [[f32; 2]; NUMENTS],
    occupied: [bool; NUMENTS],
    lastguid: usize,
}
impl Transforms {
    pub fn new() -> Transforms {
        let mut t = Transforms {
            localpositions: [[0.0, 0.0]; NUMENTS],
            localangles: [0.0; NUMENTS],
            localscales: [[0.0, 0.0]; NUMENTS],
            parents: [0; NUMENTS],
            globalpositions: [[0.0, 0.0]; NUMENTS],
            globalangles: [0.0; NUMENTS],
            globalscales: [[0.0, 0.0]; NUMENTS],
            occupied: [false; NUMENTS], lastguid: 0
        };
        t.occupied[0] = true; // first transform is root
        t
    }
    pub fn addnewparent(&mut self, initpos: [f32; 2], initangle: f32, initscale: [f32; 2], parent: usize) -> usize {
        let mut free = false;
        for i in 1..NUMENTS {
            if self.occupied[i] == false {
                self.lastguid = i;
                free = true;
                break;
            }
        }
        if free == false {
            println!("No more transform slots left!");
            return 0;
        }
        self.occupied[self.lastguid] = true;
        self.localpositions[self.lastguid] = initpos;
        self.localangles[self.lastguid] = initangle;
        self.localscales[self.lastguid] = initscale;
        self.set_parent(self.lastguid, parent);
        self.lastguid
    }
    pub fn addnew(&mut self, initpos: [f32; 2], initangle: f32, initscale: [f32; 2]) -> usize {
        self.addnewparent(initpos, initangle, initscale, 0)
    }
    pub fn moveto(&mut self, guid: usize, newpos: [f32; 2]) {
        if guid == 0 { return; }
        self.localpositions[guid] = newpos;
    }
    pub fn moveby(&mut self, guid: usize, delta: [f32; 2]) {
        if guid == 0 { return; }
        self.localpositions[guid][0] += delta[0];
        self.localpositions[guid][1] += delta[0];
    }
    pub fn set_parent(&mut self, guidc: usize, guidp: usize) {
        if self.occupied[guidp] == false {
            println!("No transform at {}", guidp);
            return;
        }
        if self.occupied[guidc] == false {
            println!("No transform at {}", guidc);
            return;
        }
        if guidp == guidc {
            println!("Not parenting {} to itself.", guidp);
            return;
        }
        if self.parents[guidp] == guidc {
            println!("Parenting {} to {} would cause a cyclical parent. Not doing.", guidc, guidp);
            return;
        }
        self.parents[guidc] = guidp;
    }
    pub fn update_transforms(&mut self, guid: usize) {
        self.globalpositions[guid] = self.localpositions[guid];
        self.globalangles[guid] = self.localangles[guid];
        self.globalscales[guid] = self.localscales[guid];
        if self.parents[guid] != 0 {
            self.globalpositions[guid][0] += self.globalpositions[self.parents[guid]][0];
            self.globalpositions[guid][1] += self.globalpositions[self.parents[guid]][1];
            self.globalscales[guid][0] *= self.globalscales[self.parents[guid]][0];
            self.globalscales[guid][1] *= self.globalscales[self.parents[guid]][1];
            self.globalangles[guid] += self.globalangles[self.parents[guid]];
        }
        for i in 1..NUMENTS {
            if self.parents[i] == guid {
                self.update_transforms(i);
            }
        }
    }
}