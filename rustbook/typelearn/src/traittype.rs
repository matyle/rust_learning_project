// pub struct Pig;
pub struct Duck {
    pub action: String,
}
pub struct Pig {
    pub action: String,
}

pub trait Fly {
    fn fly(&self) -> bool;
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

pub fn fly_static<T: Fly>(s: T) -> bool {
    return s.fly();
}

pub fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}
