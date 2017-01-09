trait Foo {
    fn foo(&self);

    fn bar(&self) {
        println!("We called bar.");
    }
}


trait FooBar : Foo {
    fn foobar(&self);
}


struct Baz;

impl Foo for Baz {
    fn foo(&self) {
        println!("foo");
    }
}

impl FooBar for Baz {
    fn foobar(&self) {
        println!("foobar");
    }
}

//Drop 提供一个值退出作用域执行代码的功能 只有一个drop(&mut self) 的方法
//Borrow用于创建一个数据结构时把拥有和借用的值看作等同
//AsRef用于在泛型中把一个值转换为引用
//Deref<Target=T>用于把&U类型的值自动转换为&T类型
// Iterator用于集合(collection)和惰性值(lazy value generator)实现迭代器
// Sized用于标记运行时长度固定的类型，而不定长的切片和特性必须放在指针后面使其运行时
// 长度已知,&[T] 和 Box<Trait>.
fn main() {
    let baz : Baz = Baz;
    baz.foobar();
    FooBar::foobar(&baz);
}