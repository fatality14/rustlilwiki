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

    //0
    //variable casts to type while writing (&), (&mut) etc near
    //also it might be casted in the same way using (as)

    //1
    //variable must be the same type as the value while assign
    //there is pointer part of type and variable part of type
    //they must be exact the same expect some cases below

    //2
    //while assign type could not be (*mut) or (*const)
    //but might be (&mut) or (&) or whatever

    //3
    //(*mut T) might be assigned with (&mut T)
    //(*mut &T) might be assigned with (&mut &T)
    //(&T) might be assigned with (&mut T | &mut &T)
    //(*const) have the same behavior as (*mut)

    // struct Point {
    //     mut x: i32,
    //     mut y: i32, // Nope.
    // }

    // let x = 5;
    // x = 6;

    // let mut a = 1;
    // let b: *mut i32 = &mut a;

    // let mut c = a;
    // let d = &mut c;

    //4
    //there is no (mut) fields

    //5
    //a variable must be (mut) if it could change
    //else it is constant

    //6
    //(&mut) type variable could only be assigned if variable is marked (mut)
    //rules 1,2,3 are still working while assign


    // let mut s = &3;
    
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    //7
    //it must be one (&mut T | &mut &T) at a time if the variable is planned to be used
    //or any of (&T) at a time
    //if no (&) or (&mut) sepcified the move operation occurs
    //the move operation steals a pointer from variable


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
    //there is pointer to pointer syntax allowed using (&mut &mut T) type or simmilar

    
    trait Trait<T: Default>{

    }

    struct Test<&T>{

    }

    // struct Test<const T: usize>{}
}