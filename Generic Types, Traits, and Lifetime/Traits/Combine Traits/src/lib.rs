pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

pub struct SomeStruct {
    pub name: String,
}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
pub fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}