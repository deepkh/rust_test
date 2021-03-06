use libhelper::*;
use libhelper::helper::type_of;

pub fn test() {
    print!("\n------------ {} ------------\n", function!());

    // References for Copy, Move, Clone
    // 0. https://doc.rust-lang.org/std/marker/trait.Copy.html
    // 1. https://www.chainnews.com/zh-hant/articles/734292099188.htm
    // 2. https://zhuanlan.zhihu.com/p/21730929
    // 3. https://juejin.im/post/6862339363229925390

    // Difference between Copy and Clone
    //1. Copies happen implicitly: eg., y = x
    //2. Cloning is an explicit: eg., x.clone()
    //3. Clone is a supertrait of Copy, so everything which is Copy must also implement Clone
    //4. If a type is Copy then its Clone implementation only needs to return *self (see the example above).

    //implicitly: Copy, Move
    {
        /*
        // Copy
        {
            let mut a: i32 = 123;
            let b = a;
            a = 456;
        }
        */

        /*
        // Move 
        {
            let a: String = "AAA".to_string();
            let b = a;
            //a.push_str("BBB"); //error[E0382]: borrow of moved value: `a`
        }
        */

        /*
        // Move: no impl copy
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let a: Data = Data{a:123, b:456};
            let b = a;                              //this is Move
            //a.a = 456;                              //error[E0382]: assign to part of moved value: `a`
            //let c = a.clone();                        //method not found in `copy_move_clone::test::Data`
        }
        */

        /*
        // Copy: impl copy manually 
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            impl Copy for Data {}

            //Clone is a supertrait of Copy, so everything member variables which is Copy must also implement Clone
            //in this case the copy behavior is same as clone
            impl Clone for Data {
                fn clone(&self) -> Data {
                    *self                   //does this means copy ?
                }
            }
            
            let mut a: Data = Data{a:123, b:456};
            let b = a;                              //this is Copy
            a.a = 456;
            a.b = 123;
            let c = a.clone();                      //this same as copy in this case
        }
        */

        /*
        // Copy: impl copy by use derive, Clone  
        {
            //Clone is a supertrait of Copy, so everything member variables which is Copy must also implement Clone
            //in this case the copy behavior is same as clone
            #[derive(Debug, Copy, Clone)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut a: Data = Data{a:123, b:456};
            let b = a;
            a.a = 456;
            a.b = 123;
            let c = a.clone();                      //this same as copy in this case
        }
        */
    }

    // explicit: Clone
    {
        /*
        //Clone:
        {
            let mut a: String = "AAA".to_string();
            let b = a.clone();
            a.push_str("BBB");
        }
        */

        //Clone: impl clone manually
        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
            }
            
            //impl Copy for Data {}             //can't implemnt Copy due to 'a: String' are not implement Copy

            impl Clone for Data {
                fn clone(&self) -> Data {
                    print!("clone occured ");
                    //*self                       //error[E0507]: cannot move out of `*self` which is behind a shared reference
                    Data {
                        a: self.a.clone(),
                        b: self.b,
                    }
                }
            }

            let mut a: Data = Data{a: "AAA".to_string(), b: 123};
            //let b = a;                      //this is move  
            //a.b = 456;                      //error[E0382]: assign to part of moved value: `a
            let mut c = a.clone();
            a.a = "BBB".to_string();
            c.a = "CCC".to_string();
            print!("a {:?} c {:?}\n", a, c);    //clone occured a Data { a: "BBB", b: 123 } c Data { a: "CCC", b: 123 }
        }

        //Clone: imple Clone by use derive
        {
            #[derive(Debug, Clone)]        
            struct Data {
                a: String,
                b: i32,
            }
            
            let mut a: Data = Data{a: "AAA".to_string(), b: 123};
            //let b = a;                      //this is move
            //a.b = 456;                      //error[E0382]: assign to part of moved value: `a
            let mut c = a.clone();
            a.a = "BBB".to_string();
            c.a = "CCC".to_string();
            print!("a {:?} c {:?}\n", a, c); //a Data { a: "BBB", b: 123 } c Data { a: "CCC", b: 123 }
        }
    }
    
    
    // borrow reference & deref
    {
        // Copy
        {
            #[derive(Debug, Copy, Clone)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut p:Data = Data{a:123, b:456};
            let mut p1 = &mut p;
            let mut p2 = *p1;           //this is copy
           
            p1.a = 789;
            p2.a = 369;

            print!("\np1:{:?}  type_of:{}\n", p1, type_of(&p1)); //p1:Data { a: 789, b: 456 }  type_of:&mut others::copy_move_clone::test::Data
            print!("p2:{:?}  type_of:{}\n", p2, type_of(&p2));   //p2:Data { a: 369, b: 456 }  type_of:others::copy_move_clone::test::Data
        }

        /*
        // No impl Copy 
        {
            #[derive(Debug, Clone)]        
            struct Data {
                a: String,
                b: i32,
            }

            let p:Data = Data{a:"123".to_string(), b:456};
            let p1 = &p;
            //let p2 = *p1;         //this is move
                                    //error[E0507]: cannot move out of `*p1` which is behind a shared reference
                                    //move occurs because `*p1` has type `copy_move_clone::test::Data`, which does not implement the `Copy` trait
        }
        */
    }

    //Conclusion:
    // a. Impl copy or not
    //     1. Data type impl copy
    //         1. then the 'let x: Data = y' would be copy
    //     2. Data type not impl copy
    //         1. then the 'let x: Data = y' would be move
    //
    // b. Borrow & deref:
    //      The following 'z = *x' behavior 
    //         let x: Data = &y;
    //         let z = *x;
    //
    //      would be something like as below
    //          let x = z;
    //
    //      move or copy havior would depend on Data impl copy or not
    //      but, if the Data not impl copy then always can't move due to
    //      error[E0507]: can't move a shared references.
    //      this means you can borrow a reference from a variable, but 
    //      you can't move a reference from a borrowed reference.
}

