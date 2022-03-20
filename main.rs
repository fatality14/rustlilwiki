//comment for pure vision, compiler doesnt show all warnings
fn main() {
    // fn f1(i: i32){}
    // fn f2(i: &i32){}
    // fn f3(i: &mut i32){}
    // fn f4(i: &mut &i32){}
    // fn f5(i: *mut i32){}
    // fn f6(i: *mut &i32){}
    // fn f7(i: *const i32){}
    // fn f8(i: *const &i32){}

    // let v2: &i32 = &1;
    // f1(v2);f1(v2);
    // f2(v2);f2(v2);
    // f3(v2);f3(v2);
    // f4(v2);f4(v2);
    // f5(v2);f5(v2);
    // f6(v2);f6(v2);

    // f7(v2);f7(v2);
    // f8(v2);f8(v2);

    // let a = &mut *mut 3i32;

    // let b: *mut i32 = &mut 1;
    // unsafe{ *b; <*const i32>::as_ref(b) };

    //0
    //variable casts to type while writing (&), (&mut) etc near
    //also it might be casted in the same way using (as) especially when we cast to a different type
    //we can cast pointer types back with (*T)
    //we can cast raw pointer types back with unsafe (*T) or methor as_ref

    //1
    //variable must be the same type as the value while assign
    //there is pointer part of type and variable part of type
    //they must be exact the same expect some cases below

    //2
    //type could not be casted using (*mut) or (*const)
    //but might be casted with (&mut) or (&) or just () and other combinations

    //3
    //(*mut T) might be assigned with (&mut T)
    //(*mut &T) might be assigned with (&mut &T)
    //(&T) might be assigned with (&mut T | &mut &T)
    //(*const) have the same behavior as (*mut)
    //some more info(https://doc.rust-lang.org/reference/type-coercions.html)


    // let x = 5;
    // // x = 6;
    // let mut xm = x;
    // xm = 6;

    // let mut a = 1;
    // let b: *mut i32 = &mut a;

    // let mut c = a;
    // let d = &mut c;

    //4.1
    //a variable must be (mut) if it could change
    //else it is constant
    //4.2
    //we can redeclare non(mut) variable as (mut) if there is no other borrowing operations

    //5
    //(&mut) type variable could only be assigned if assignable variable is marked (mut)
    //rules 1,2,3 are still working while assign
    

    // struct Point {
    //     mut x: i32,
    //     mut y: i32, // Nope.
    // }

    //6
    //there is no (mut) fields


    // fn foo(a: &mut &i32, b: &mut &i32){}

    // let mut s = &3;
    
    // let r1 = &mut s;
    // let r2 = &mut s;

    // foo(r1,r2);

    // fn foo1(a: *mut &i32, b: *mut &i32){}
    
    // let mut s = &3;
    
    // let r1: *mut &i32 = &mut s;
    // let r2: *mut &i32 = &mut s;

    // foo1(r1,r2);

    //7.1
    //it must be one (&mut T | &mut &T) at a time if the variable is planned to be used
    //or any of (&T) at a time
    //if no (&) or (&mut) sepcified the move operation occurs
    //the move operation steals a pointer from variable
    //note that (*mut T | *mut &T) allows you to obey this rule

    
    // let s = "123".to_string();
    // let s1 = "123";
    // let s2 = s + s1;

    //n
    //TODO add rule here about move in operators


    // struct Test(i32);
    // fn f(s : Test){}

    // let s1 = Test(1);
    // let s2 = s1;

    // f(s1);

    //8
    //the types of s1 and s2 are the same, so we can assing them according to rules 1,2,3
    //we treat s1 and s2 content as a pointer
    //so the content of struct pointer s1 moves to s2 while assing according to rule 7
    //we should make a deep copy of s1 to fix this problem or else use s2 instead


    // let mut s = "sdsd".to_string();
    // let b = s[..2];

    //9
    //slice is always a pointer to a part of an array
    //so move operation of rule 7 is not allowed
    //instead allowed all other operations according to rules 1,2,3


    // let v1: &mut &mut i32;
    // let mut v2: &mut i32 = &mut 2;
    // let v3 = &mut v2;
    // v1 = v3;

    //10
    //there is a pointer to pointer syntax allowed using (&mut &mut T) type or simmilar


    // trait SampleTrait<V> {
    //     fn foo<T: Sized>(&self, _p : T) {
    //         println!("{}", std::any::type_name::<T>());
    //         println!("{}", std::any::type_name::<V>());
    //     }
    // }

    // struct Test();
    // impl SampleTrait<Test> for Test {}

    // let a = Test();
    // a.foo(&&&&&a);

    //11
    //generics could not be typed with (&), (&mut) etc
    //instead they inherit type of given value
    //it is allowed to use pointer to pointer in generics using rule 10

    //12
    //calling method could borrow a type which it is called from


    // #[derive(Copy, Clone)]
    // struct Test;

    // fn foo(t : Test){}
    
    // let t = Test;
    // let t1 = t;
    // foo(t);

    //13
    //functions must have Sized arguments cause of asm needs
    //they use the same assign rules as other varibales
    
    //14
    //structs must implement (Clone) and (Copy) traits to avoid move described in rule 7
    //it is possible to implement a single (Clone) trait to clone contents of pointers by hand


    // trait Trait /*: Sized*/ {}
    // struct Test1();
    // struct Test2();

    // impl Trait for Test1 {}
    // impl Trait for Test2 {}

    // fn foo(t : dyn Test1){}
    // fn foo(t : dyn Trait){}
    // fn foo(t : *mut dyn Trait){}
    // fn foo(t : &mut dyn Trait){}
    // fn foo(t : &dyn Trait){}
    // fn foo(t : Box<dyn Trait>){}
    
    // foo(&Test1());
    // foo(&Test2());

    // fn static_foo<T:Trait + ?Sized>(b: T) {}
    // fn static_foo<T:Trait>(b: T) {}

    //15.1
    //(dyn Trait | &dyn Trait) declares that trait type must implement (Sized)
    //pointers have a size, so we can borrow the trait at compile time
    //wherein the content of the pointer itself might not be sized
    //15.2
    //(Sized) traits have a size to be used with (dyn Trait), but if so then they aren't obejct-safe
    //(https://doc.rust-lang.org/reference/items/traits.html#object-safety)
    //so we cannot create vtable and that's why it is prohibited to use (dyn Trait)
    //15.3
    //(dyn Trait) behavior implicitly works with all types
    //casue they must have a known size at a compile time according to rule 14
    //else the program could not be compiled due to asm rules
    //15.4
    //the behavior of (dyn Trait) instead might be implemented with generics
    //15.5
    //(&dyn Trait) is using vtables to fetch the varibale type
    //it could be used only with object-safe traits
    

    // trait Trait<'a> where &'a Self: Sized {}

    // trait Trait where Self : Sized + Copy{
    //     fn bar1(&self){}
    //     fn bar2(self){}
    //     fn bar3(&mut self){}

    //     fn foo(&mut self)
    //     {
    //         // Self::bar1();
    //         // bar1();
    //         // self.bar1(&self);
    //         // Self::bar1(Self);

    //         Self::bar1(self);
    //         Self::bar1(&self);
    //         self.bar1();
    //         (&self).bar1();
    //         (*self).bar1();
    //         // (**self).bar1();

    //         // Self::bar2(self);
    //         Self::bar2(*self);
    //         self.bar2();
            
    //         Self::bar3(self);
    //         self.bar3();
    //     }
    // }

    // #[derive(Clone, Copy)]
    // struct Test;
    // impl Trait for Test{}

    // let t: *mut Test = &mut Test;
    // unsafe{(*t).bar1();}
    // unsafe{(*t).bar2();}
    
    // trait UBTrait {
    //     /*unsafe*/ fn bar(self: *const Self);
    // }
    // let foo: *const dyn UBTrait = unsafe { std::mem::transmute([0usize, 0x1000usize]) };
    // foo.bar();

    //n.1
    //(self) is equals to (*this), and call methods with the same pattern
    //(self) is a varibale and keep up all variable rules
    //n.2
    //(Self) is trait or struct type and keep up all type rules except some rules below
    //(Self) could be specified using (where) declaration wherever it needed to
    //n.3
    //struct operator (.) automatically set (self) argument of declared type in method
    //n.4
    //we cannot brake trait vtable using (*const Self) with garbage value
    //because it is prohibited to use raw pointers with Self type
    //n.5
    //(self) with (&mut dyn Trait) must be (&mut)


    // const fn foo2(){}

    // fn foo<F>(f:F)
    // where
    //     F: for<'a> Fn(&'a str, &'a str) -> &'a str
    // {}

    //n.1
    //there is (const) functions that behave like constexpr
    //they are being calculated at compile time
    //there is also ~const experimantal feature const_fn_trait_bound
    //n.2
    //there is the way to declare type lifetime etc using (for) syntax


    // let x = fn a(){};
    // let x = fn(){};
    
    // fn t(a: i32){}
    // fn foo<F,T>(f : /*&*/F, t: T) where F: Fn/*Mut*/(T){
    //     f(t);
    // }
    // foo(/*&*/t, 5);

    // fn foo1(){}
    // let a = foo1;
    // let b: &dyn Fn() = &foo1;
    // b();

    //n.1
    //originally function pointer type is (fn T())
    //n.2
    //function pointer trait is (Fn), (FnOnce) and (FnMut)
    //(Fn) is for (&self) borrowing
    //(FnOnce) is for (self) argument which does move operation
    //(FnMut) is for (&mut self) borrowing
    //we could implement these traits by overloading (call) operator
    //n.3
    //there is shortcut for traits as a return function type
    //(impl) allows to dynamically implement trait to empty struct


    // let mut i = "123".to_string();
    // let c = |p: i32|{
    //     i.clear();
    //     // let a = s;
    //     // println!("{}",s);
    // };
    // // c(1);
    // let mut f = c;
    // f(1);

    // fn make_fn() -> impl Fn(){
    //     let s = "123".to_string();
    //     move || {
    //         let v = &s;
    //     }
    // }

    //n.1
    //closure is just a function with current scope visibility
    //it do not copy variables implicitly, so all variable assign rules works the same
    //arguments of closure works the same as (fn) arguments except (self) which is no allowed?
    //n.2
    //it do implement one of (Fn*) traits implicitly according to borrowing rules
    //it is (Fn) by default and if using (&) borrowing
    //it become (FnOnce) if there is move occurs in body
    //it become (FnMut) if using (&mut) borrowing
    //(FnMut) closure itself must be a (mut) variable if intended to be called
    //n.3
    //if closure is declared with (move) it is trying to move all scope variable in
    //but closure still might be (FnMut) and (Fn)


    //n
    //some magick with smart pointers here


    //n
    //learn lifetime erasure rules first (https://linkhere)
    //some magic with variance
    //https://doc.rust-lang.org/reference/subtyping.html



    // trait Trait<T: ?Sized> {
    //     fn foo(){
    //         std::mem::size_of::<T>();
    //     }
    // }

    //13.1
    //? means that T be not sized, but it is not restricted
    //TODO write where to use it

    // struct Test<const T: usize>{}

    // trait Trait<const T: usize> {
    //     fn foo(){
    //         print!("{}",T);
    //     }
    // }

    //13.2
    //there is kinda variable template in here

    // struct Test();    
    // impl Test {
    //     fn Test(self) -> Test{Test()}
    // }

    //13.3
    //there is no constructor, new and method overloading
    //so no standart for implementing it


    //n
    //some magic about macro here

    
    //n
    //macro and generics cooperation
}