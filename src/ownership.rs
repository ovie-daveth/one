pub fn owner(){

    // let mut s1 = String::from("Hello");
    // let mut s2 = s1;

    // s2 = s2 + " " + "Yes";

    // println!("Try {}",  s2);

    // let s = String::from("hello");
    // let r = &s; // `r` is an immutable reference to `s`
    // println!("r: {}, {}", r, s); // OK


    // let mut d = String::from("hello");
    // let d = &mut d; // Borrow mutably
    // d.push_str(", world");
    // println!("{}", d); // "hello, world"


    //structs and enums

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    let mut user_1 = User{
        email: String::from("oviedavid@gmail.com"),
        username: String::from("Ovie"),
        sign_in_count: 4,
        active: false
    };

    let name = user_1.username;
    user_1.username = String::from("David");

    fn build_user(email: String, username: String) -> User{
        User {
            email,
            username,
            sign_in_count: 0,
            active: false
        }
    }

    let user2 = build_user(String::from("Dvaid55@gmail.com"), String::from("Emmanuel"));
    let _user3 = User{
        ..user2
    };


    #[derive(Debug)]
    struct Reactangle {
        width: u64,
        height: u64
    }

    let rect = Reactangle{
        width: 34,
        height: 54
    };

    impl Reactangle {
        fn area(&self) -> u64 
        {
            self.height * self.width
        }
    }

    // fn area_rectangle(rect: &Reactangle) -> u64{
    //     let area = rect.width * rect.height;
    //     area
    // }

    // let area = area_rectangle(&rect);

    let area = rect.area();

    println!("The area of the triangle {:#?} {}", rect, area);
}