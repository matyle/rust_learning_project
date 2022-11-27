fn main() {
    let mut c = CacherRef::new(|a| a);
    let key = String::from("maty");
    c.value(&key);
    let key2 = String::from("tan");
    let res = c.value(&key2);
    println!("{:?}", res);

    // let mut c1 = Cacher::new(|x| x);
    // c1.value(1);
    // let v2 = c1.value(2);
    // println!("{:?}", v2);
}

//结构体中的闭包
//TODO:思考是否有不使用Copy值特征实现的方法？

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct CacherRef<'a, T, E: 'a>
where
    T: Fn(&'a E) -> &'a E,
{
    query: T,
    value: Option<&'a E>,
}

impl<'a, T, E: 'a> CacherRef<'a, T, E>
where
    T: Fn(&E) -> &E,
{
    fn new(query: T) -> Self {
        CacherRef { query, value: None }
    }

    fn value(&mut self, arg: &'a E) -> &'a E {
        match self.value {
            Some(v) => &v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
