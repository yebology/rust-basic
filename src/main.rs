enum Direction {
    Left,
    Right, 
    Up,
    Down
}

struct Person {
    name: String,
    age: i32,
    status: String
}

impl Person {
    fn show_person(person: Person) {
        println!("{}", person.name);
        println!("{}", person.age);
        println!("{}", person.status);
    }
}
#[derive(Debug)]
enum Menu {
    MainMenu,
    Start, 
    Quit
}

#[derive(Debug, Clone, Copy)]
struct Account {
    income: i64,
    outcome: i64
}

struct Survey {
    q1: Option<i32>,
    q2: Option<String>
}

struct Book {
    pages: i32,
    rating: i32
}

fn main() {
    // function
    let x = add(6, 4);
    let y: i64 = add(5, 4);
    let z = x + y;
    println!("The sum of x and y is : {}", z);

    // if else
    if z > 4 {
        println!("z is greater than 4");
    }
    else {
        println!("z is less than 4");
    }

    // loop and mutable variables
    let mut var = 0;
    loop {
        if var == 5 {
            break;
        }
        else {
            print!("{} {}", var, " ");
            var += 1;
        }
    }
    println!(" ");

    // while loop
    let mut second_variable: i32 = 0;
    while second_variable < 5  {
        print!("{} {}", second_variable, " ");
        second_variable += 1;
    }
    println!(" ");

    // match 
    let int_variable : i64 = 2;
    match int_variable {
        1 => println!("It's one"),
        2 => println!("It's two"),
        3 => println!("It's three"),
        _ => println!("It's something else"),
    }

    // enum
    let right: Direction = Direction::Right;
    match right {
        Direction::Right => println!("Right"),
        Direction::Up => println!("Up"),
        Direction::Left => println!("Left"),
        Direction::Down => println!("Down"),
    }

    // struct
    let person = Person {
        name: String::from("Yobel"),
        age: 20,
        status: String::from("Student")
    };
    println!("Hello! My name is {}", person.name);
    println!("My age is {}", person.age);
    println!("My status is {}", person.status);

    let book: Book = Book {
        pages: 100,
        rating: 4  
    };
    let (page, rating) = display_books_detail(book);
    println!("The number of pages is {}", page);
    println!("The rating is {}", rating);

    // tuple
    let (x, y, z) = collection_of_numbers();
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);

    // implementation
    let second_person = Person {
        name: String::from("Agus"),
        age: 35,
        status: String::from("Worker")
    };
    Person::show_person(second_person);

    // vector
    let mut vec_of_numbers = Vec::new();
    vec_of_numbers.push(1);
    vec_of_numbers.push(2);
    vec_of_numbers.push(3);
    vec_of_numbers.len();
    vec_of_numbers.pop();

    // derive (debug, clone, copy)
    let mut original_account = Account {
        income: 100,
        outcome: 50
    };
    let cloned_account: Account = original_account.clone(); // cloning
    let copied_account: Account = original_account; // copy
    original_account.income += 50;
    println!("Original Account Income : {}", original_account.income);
    println!("Cloned Account Income : {}", cloned_account.income);
    println!("Copied Account Income : {}", copied_account.income);

    // option
    let response = Survey {
        q1: None,
        q2: Some("A".to_string())
    };
    match response.q1 {
        Some(q1) => println!("q1 is {}", q1), // kalo ada value e
        None => println!("q1 is None") // kalo gada value
    };

    // result
    let result_menu = pick_menu("MainMenu");
    println!("menu value : {:?}", result_menu);
    
}

fn add(a: i64, b: i64) -> i64 {
    return a + b;
}

fn collection_of_numbers() -> (i64, i64, i64) {
    return (1, 2, 3);
}

fn display_books_detail(book: Book) -> (i32, i32) {
    return (book.pages, book.rating);
}

fn get_choice(choice: &str) -> Result<Menu, String> {
    match choice {
        "MainMenu" => Ok(Menu::MainMenu),
        "Start" => Ok(Menu::Start),
        "Quit" => Ok(Menu::Quit),
        _ => Err("Invalid choice".to_owned())
    }
}

fn print_menu(result_menu: &Menu) {
    println!("You choose : {:?}", result_menu);
}

fn pick_menu(input: &str) -> Result<(), String> {
    let choose = get_choice(input)?;
    print_menu(&choose);
    Ok(())
}