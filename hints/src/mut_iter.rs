// fn test() {
//     pub struct Foo ([u32; 10]);

//     struct FooIterMut<'a> {
//         idx: usize,
//         x: &'a mut Foo,
//     }
    
//     impl<'a> FooIterMut<'a> {  
//         fn next<'b>(&'b mut self) -> &'a mut u32 {
//             &mut self.x.0[self.idx] // compile error, потому что можем получить две одинаковые mut ссылки
//         }
//     }

//     let mut foo = Foo([0; 10]);
//     let foo_ref = &mut foo;
//     let next1 = {
//         let mut iter = FooIterMut {
//             idx: 0,
//             x: foo_ref,
//         };
//         let next = iter.next();
//         next
//     };
//     let next2 = {
//         let mut iter = FooIterMut {
//             idx: 0,
//             x: foo_ref,
//         };
//         let next = iter.next();
//         next
//     };

//     // next2 поменяли, а изменился next1
//     *next2 += 1;
//     println!("{next1}");
// }
