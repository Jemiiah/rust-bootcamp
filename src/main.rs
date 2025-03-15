// // use uuid::Uuid as;

// #[derive(Debug)]
// struct User {
//     // id: Uuid,
//     id: u128,
//     name: String,
//     gender: Sex,
//     marital_status: Status,
// }

// #[derive(Debug)]
// enum Status {
//     Married,
//     Single,
//     Divorce,
//     Widow,
// }

// #[derive(Debug)]
// enum Sex {
//     Male,
//     Female,
// }

// // util function to convert from u128
// fn convert_usize_to_u128(len: usize) -> u128 {
//     len as u128
// }
 
// impl User {
//     fn new_user(users: &Vec<User>, name: String, gender: Sex, marital_status: Status) -> User {
//         User {
//             id: convert_usize_to_u128(users.len()) + 1,
//             name,
//             gender,
//             marital_status,
//         }   
//     }

//     fn update_user_name(name: String, users_data: &mut Vec<User>, id: u128) {
//         let updated_name = users_data.iter_mut().find(|x| x.id == id).unwrap();
//         updated_name.name = name;
//     }
//     fn update_user_sex(gender: Sex, users_data: &mut Vec<User>, id: u128) {
//         let updated_sex = users_data.iter_mut().find(|x| x.id == id).unwrap();
//         updated_sex.gender = gender;
//     }
//     fn update_user_marital_status(status: Status, users_data: &mut Vec<User>, id: u128) {
//         let updated_status = users_data
//             .iter_mut()
//             .find(|x: &&mut User| x.id == id)
//             .unwrap();
//         updated_status.marital_status = status;
//     }

//     // fn find_user(|x: &&mut User)
// }

// fn main() {
//     let mut users_data: Vec<User> = Vec::new();
//     println!("new users here: {:#?}", users_data);
//     users_data.push(User::new_user(
//         &users_data,
//         "yunus".to_string(),
//         Sex::Male,
//         Status::Single,
//     ));
//     println!("new user 1 here: {:#?}", users_data);

//     users_data.push(User::new_user(
//         &users_data,
//         "Titilola".to_string(),
//         Sex::Female,
//         Status::Divorce,
//     ));
//     // users_data.push(User::new_user(
//     //     "yunus".to_string(),
//     //     Sex::Male,
//     //     Status::Married,
//     // ));
//     // users_data.push(User::new_user(
//     //     "funke".to_string(),
//     //     Sex::Female,
//     //     Status::Widow,
//     // ));

//     // users_data.push(User::new_user(
//     //     "funke".to_string(),
//     //     Sex::Female,
//     //     Status::Widow,
//     // ));

//     User::update_user_name("Iliya".to_string(), &mut users_data, 2);

//     // User::update_user_marital_status(Status::Married, &mut users_data);
//     // User::update_user_sex(Sex::Female, &mut users_data, 3);
//     // User::update_user_name("kemi".to_string(), &mut users_data, 2);
//     println!("updated user here: {:#?}", users_data);

//     User::update_user_name("Martins".to_string(), &mut users_data, 21);

//     println!("updated user 3 here: {:#?}", users_data);
// }

// fn main() {
    // let r;                // ---------+-- 'a
    //                       //          |
    // {                     //          |
    //     let x = 5;        // -+-- 'b  |
    //     r = &x;           //  |       |
    // }                     // -+       |
    //                       //          |
    // println!("r: {r}");   //          |

    // fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x    
    //     } else {
    //         y
    //     }

    // }
    
    //    let v = vec![10, 20, 40, 80, 160, 320, 640, 1280, 2560];
    //    let fourth = &v[4];
    //    for i in &v {
    //     println!("{i}");
    //    }
    //    println!("The fourth element is {fourth}");
       

    //    let sixth = v.get(6);
    //    match sixth {
    //     Some(sixth) => println!("The sixth element is {sixth}"),
    //     None => println!("There is no sixth element"),
           
    //    }

    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();

    
        // let numbers = vec![10, 20, 30, 40, 50];
    
        // for element in numbers {  // No indices, just direct iteration
        //     println!("Number: {}", element);
        // }
        

        // let mut numbers = vec![20, 30, 40, 50, 60, 70, 100];

        // for num in numbers.iter_mut() {
        //     *num += 5;
        // pxrintln!("The list number: {:?}", num);
        
        // }
        // numbers.push(120);

        // println!("{:?}", numbers);


        // let age = vec![22, 30, 42, 53, 20, 65, 72];

        // let ppl: Option<&i32> = age.get(3);
        // println!("fetch the age : {:?}", ppl);

        // for people in age.iter() {
        //     println!("The whole group possess : {:?}", people);
        // }
        // println!(": {:?}", &age);

        // use std::collections::HashMap;

        // fn main() {
            // let text = "Hello world wonderful world";

            // let mut map = HashMap::new();

            // for word in text.split_whitespace()  {
            //     let count = map.entry(word).or_insert(0);
            //     *count += 1;

            // }
            // println!("{:?}", map);



            // fn main() {
            //     let mut vec = vec!(20, 24, 30, 35, 30, 40, 28, 30);
            //     let elements: &[i32] = &[19, 30, 46, 35, 30]; // A slice
                
            //     add_elements(&mut vec, elements);
            //     vec.sort();
            //     assert_eq!(vec, vec![19, 20, 24, 28, 30, 30, 30, 30, 30, 35, 35, 40, 46]);
            
            //     remove_element(&mut vec, 4);
            //     let fourth: Option<&i32> = get_element(&vec, 4);
            //     match fourth {
            //         Some(fourth) => println!("the fourth element is {}", fourth), 
            //         None => println!("age doesn't exist"),
            //     };
            // }

            // pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32]) {
            //     vec.extend_from_slice(elements);           
            //     println!("all the ages are : {:?}", vec);
            // }
            
            // pub fn remove_element(vec: &mut Vec<i32>, index: usize) {    
            //     if index < vec.len() {
            //         vec.remove(index);
            //         println!("update vector after removal: {:?}", vec);
                    
            //     } else {
            //         println!("Index out of bounds: {:?}", vec);
                    
            //     }
            
            // }
            
            // pub fn get_element(vec: &Vec<i32>, index: usize) -> Option<&i32> {
            //     vec.get(index)
             
            // }

        //    use std::collections::HashMap;

        //    fn main() {

        //    let mut scores = HashMap::new(); 

        //    scores.insert(String::from("Blue"), 20);
        //    scores.insert("Red".to_string(), 60);

        //    let team_name = String::from("Blue");
        //    let score = scores.get(&team_name).copied().unwrap_or (0);

        //    let mut orders: HashMap<String, String> = HashMap::new();

        //    orders.insert("Jane".to_string(), "coffee, constraints".to_string());
        //    orders.insert(String::from("Winnie"), String::from("tea, muffins"));
        //    orders.insert(String::from("Steve"), String::from("tea, muffins, mint"));

        //    match orders.get("Winne") {
        //     Some(order) => println!("Alice order is ready: {:?}", order),
        //     None => println!("Order doesn't exist"),
               
        //    }
        //    orders.insert("Jane".to_string(), "coffee, constraints, cofee".to_string());

        //    orders.remove("Steve");

        //    for(customer, order) in &orders {
        //     println!("{} ordered: {}", customer, order);  
        //    }
                   
        //    }

        //    use std::collections::HashMap;

        //    fn main() {
            
        //     let mut student_grades: HashMap<String, Vec<f32>> = HashMap::new(); 
        //     student_grades.insert("david".to_string(), vec![85.5, 90.0, 55.5, 79.5]);
        //     student_grades.insert("Ebube".to_string(), vec![60.5, 75.5, 88.6, 49.9]);
        //     student_grades.insert("Nathaniel".to_string(), vec![30.5, 60.9, 91.5,]);
            

        //     match calculate_nathaniel_grades(&student_grades) {
        //         Some(avg) => println!("Nathaniel's average grade: {:.2}", avg),
        //         None => println!("Unable to calculate Nathaniel's average (no grades or not found)"), 
        //     }

        //     student_grades.insert("Nathaniel".to_string(), vec![]);
        //     match calculate_nathaniel_grades(&student_grades) {
        //         Some(avg) => println!("Nathaniel grades average: {:?}", avg),
        //         None => println!("Unable to calculate Nathaniel's average (no grades or not found)"),
        //     }

        //     student_grades.remove("Nathaniel");
        //     match calculate_nathaniel_grades(&student_grades) {
        //         Some(avg) => println!("Nathaniel's average grade: {:.2}", avg),
        //         None => println!("Unable to calculate Nathaniel's average (no grades or not found)"),   
        //     }
            
        // }

        //    fn calculate_nathaniel_grades(grade: &HashMap<String, Vec<f32>>) -> Option<f32> {
        //       if let Some(nathaniel_grades) = grade.get("Nathaniel") {
        //         if !nathaniel_grades.is_empty() {
        //             let sum:f32 = nathaniel_grades.iter() .sum();
        //             let avg = sum / nathaniel_grades.len() as f32;
        //             Some(avg)
        //         } else {         
        //             None
        //             }

        //         } else {
        //             None
        //         }
        //       } 


        // fn find_needle(vec: Vec<i32>, target: i32 ) -> Option<i32> {
        //     for num in vec {
        //         if num == target {
        //             return Some(num);
        //         }
        //     }
        //     None


        // }
        
    // #[derive(Debug)]
    // pub enum TrafficLight {
    //     Red,
    //     Green,
    //     Yellow,
    //     // Define enum variants here
    // }
    
    // pub fn light_action(light: &TrafficLight) -> &'static str {
    //     // Your code here...
    //     match light {
    //     TrafficLight::Red => "Stop",
    //     TrafficLight::Yellow => "Caution",
    //     TrafficLight::Green => "GO",
        
    // }
    // }
    // fn main() {
    //     let red = TrafficLight::Red;
    //     let yellow = TrafficLight::Yellow;
    //     let green = TrafficLight::Green;
    
    //     println!("Red: {:?}", (red));       
    //     println!("Yellow: {:?}", (yellow)); 
    //     println!("Green: {:?}", (green));   
    
        
    // }

    use std::error;
    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greet_files_result = File::open("hello.txt");

        let greeting_file = match greet_files_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}")
                } 
                
            }
        };

            
    
   
    //  #[derive(Debug)]
    // enum Result<T, E> {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // }





       
               
           


          
            
           
             
          
           

 
          
          
           
          
            
            





            

                        




         
    

        






        







    