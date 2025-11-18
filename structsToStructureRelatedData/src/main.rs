use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

/* UserBuilder is a struct for a user builder.
 * @param id - The id of the user
 * @param active - Whether the user is active
 * @param username - The username of the user
 * @param email - The email of the user
 * @param sign_in_count - The number of times the user has signed in
 * @return - A new UserBuilder struct
 * @example
 * let user = UserBuilder::new();
 *
 */
#[derive(Debug, Clone)]
struct UserBuilder {
    id: String,
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/* ClassRoom is a struct for a class room.
 * @param list_user - The list of users in the class room
 * @param class_name - The name of the class room
 * @param class_code - The code of the class room
 * @param class_description - The description of the class room
 * @param class_start_date - The start date of the class room
 * @param class_end_date - The end date of the class room
 * @param class_start_time - The start time of the class room
 * @param class_end_time - The end time of the class room
 * @param class_location - The location of the class room
 * @param class_instructor - The instructor of the class room
 * @param class_instructor_email - The email of the instructor
 * @param class_instructor_phone - The phone of the instructor
 * @return - A new ClassRoom struct
 * @example
 * let class_room = ClassRoom::new();
 */
#[derive(Debug, Clone)]
struct ClassRoom {
    list_user: Vec<UserBuilder>,
    class_name: String,
    class_code: String,
    class_description: String,
    class_start_date: NaiveDate,
    class_end_date: NaiveDate,
    class_location: String,
    class_instructor: String,
    class_instructor_email: String,
    class_instructor_phone: String,
}

//*
/* UserBuilder is a builder pattern for creating UserBuilder structs.
 * It is used to create a new UserBuilder struct with the given parameters.
 * It is used to create a new UserBuilder struct with the given parameters.
 *
 * @param id - The id of the user
 * @param active - Whether the user is active
 * @param username - The username of the user
 * @param email - The email of the user
 * @param sign_in_count - The number of times the user has signed in
 * @return - A new UserBuilder struct
 * @example
 * let user = UserBuilder::new()
 *     .active(true)
 *     .username("John Doe".to_string())
 *     .email("john.doe@example.com".to_string())
 *     .sign_in_count(1)
 *     .get_user();
*/
impl UserBuilder {
    /**
     * Create a new UserBuilder struct with the given parameters.
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new();
     */
    fn new() -> Self {
        UserBuilder {
            id: Uuid::new_v4().to_string(),
            active: false,
            username: String::new(),
            email: String::new(),
            sign_in_count: 0,
        }
    }

    /* Set the active status of the user.
     * @param active - Whether the user is active
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new()
     *     .active(true)
     *     .get_user();
     */
    fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /* Set the username of the user.
     * @param username - The username of the user
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new()
     *     .username("John Doe".to_string())
     *     .get_user();
     */
    fn username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    /* Set the email of the user.
     * @param email - The email of the user
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new()
     *     .email("john.doe@example.com".to_string())
     *     .get_user();
     */
    fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    /* Set the sign in count of the user.
     * @param sign_in_count - The number of times the user has signed in
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new()
     *     .sign_in_count(1)
     *     .get_user();
     */
    fn sign_in_count(mut self, sign_in_count: u64) -> Self {
        self.sign_in_count = sign_in_count;
        self
    }

    /* Get the user.
     * @return - A new UserBuilder struct
     * @example
     * let user = UserBuilder::new()
     *     .get_user();
     */
    fn get_user(self) -> UserBuilder {
        self
    }
}

/* ClassRoom is a struct for a class room.
 * @param list_user - The list of users in the class room
 * @param class_name - The name of the class room
 * @param class_code - The code of the class room
 * @param class_description - The description of the class room
 * @param class_start_date - The start date of the class room
 * @param class_end_date - The end date of the class room
 * @param class_location - The location of the class room
 * @param class_instructor - The instructor of the class room
 * @param class_instructor_email - The email of the instructor
 * @param class_instructor_phone - The phone of the instructor
 * @return - A new ClassRoom struct
 * @example
 * let class_room = ClassRoom::new();
 */
impl ClassRoom {
    /* Create a new ClassRoom struct with the given parameters.
     * @param class_name - The name of the class room
     * @param class_description - The description of the class room
     * @param class_start_date - The start date of the class room
     * @param class_end_date - The end date of the class room
     * @param class_location - The location of the class room
     * @param class_instructor - The instructor of the class room
     * @param class_instructor_email - The email of the instructor
     * @param class_instructor_phone - The phone of the instructor
     * @return - A new ClassRoom struct
     * @example
     * let class_room = ClassRoom::new(
     *     "Class A".to_string(),
     *     "Class A description".to_string(),
     *     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
     *     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
     *     "Location A".to_string(),
     *     "Instructor A".to_string(),
     *     "instructor.a@example.com".to_string(),
     *     "1234567890".to_string(),
     */
    fn new(
        class_name: String,
        class_description: String,
        class_start_date: NaiveDate,
        class_end_date: NaiveDate,
        class_location: String,
        class_instructor: String,
        class_instructor_email: String,
        class_instructor_phone: String,
    ) -> Self {
        ClassRoom {
            list_user: Vec::new(),
            class_name,
            class_code: Uuid::new_v4().to_string(),
            class_description,
            class_start_date,
            class_end_date,
            class_location,
            class_instructor,
            class_instructor_email,
            class_instructor_phone,
        }
    }

    /* Get the class room.
     * @return - A new ClassRoom struct
     * @example
     * let class_room = ClassRoom::new()
     *     .get_class_room();
     */
    fn get_class_room(self) -> ClassRoom {
        ClassRoom {
            list_user: self.list_user,
            class_name: self.class_name,
            class_code: self.class_code,
            class_description: self.class_description,
            class_start_date: self.class_start_date,
            class_end_date: self.class_end_date,
            class_location: self.class_location,
            class_instructor: self.class_instructor,
            class_instructor_email: self.class_instructor_email,
            class_instructor_phone: self.class_instructor_phone,
        }
    }

    /* Get all the users in the class room.
     * @return - A vector of UserBuilder structs
     * @example
     * let users = class_room.get_all_users();
     * println!("Users: {:#?}", users);
     */
    fn get_all_users(self) -> Vec<UserBuilder> {
        self.list_user
    }

    /* Update a user in the class room.
     * @param id - The id of the user
     * @param user - The user to update
     * @return - A new ClassRoom struct
     * @example
     * let class_room = class_room.update_user_in_class_room(
     *     "911c8a9e-d1ba-4076-a4c3-976890a77380",
     *     UserBuilder::new().active(true).username("John Doe".to_string()).email("john.doe@example.com".to_string()).sign_in_count(1).get_user(),
     */
    fn update_user_in_class_room(mut self, id: &str, user: UserBuilder) -> Self {
        if let Some(existing) = self.list_user.iter_mut().find(|u| u.id == id) {
            *existing = user;
        }
        self
    }

    /* Find a user by id in the class room.
     * @param id - The id of the user
     * @return - A new UserBuilder struct
     * @example
     * let user = class_room.find_user_by_id("911c8a9e-d1ba-4076-a4c3-976890a77380");
     * println!("User: {:#?}", user);
     */
    fn find_user_by_id(&self, id: &str) -> Option<&UserBuilder> {
        self.list_user.iter().find(|u| u.id == id)
    }

    /* Delete a user by id in the class room.
     * @param id - The id of the user
     * @return - A new ClassRoom struct
     * @example
     * let class_room = class_room.delete_user_by_id("911c8a9e-d1ba-4076-a4c3-976890a77380");
     * println!("Class Room: {:#?}", class_room);
     */
    fn delete_user_by_id(mut self, id: &str) -> Self {
        if let Some(user) = self.list_user.iter_mut().find(|u| u.id == id) {
            user.active = false;
        }
        self
    }

    /* Add users to the class room.
     * @param users - The users to add
     * @return - A new ClassRoom struct
     * @example
     * let class_room = class_room.add_users(vec![UserBuilder::new().active(true).username("John Doe".to_string()).email("john.doe@example.com".to_string()).sign_in_count(1).get_user()]);
     * println!("Class Room: {:#?}", class_room);
     */
    fn add_users(mut self, users: Vec<UserBuilder>) -> Self {
        self.list_user.extend(users);
        self
    }
}

/* Main function to run the program.
 * @return - A new ClassRoom struct
 * @example
 * let class_room = main();
 * println!("Class Room: {:#?}", class_room);
 */
fn main() {
    let user_a = UserBuilder::new()
        .active(true)
        .username("John Doe".to_string())
        .email("john.doe@example.com".to_string())
        .sign_in_count(1);

    let user_a_id = user_a.id.clone();
    let user_a_clone = user_a.clone();

    let user_b = UserBuilder::new()
        .active(true)
        .username("Jane Doe".to_string())
        .email("jane.doe@example.com".to_string())
        .sign_in_count(1);

    let class_room = ClassRoom::new(
        "Class A".to_string(),
        "Class A description".to_string(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        "Location A".to_string(),
        "Instructor A".to_string(),
        "instructor.a@example.com".to_string(),
        "1234567890".to_string(),
    )
    .add_users(vec![user_a, user_b]);

    let if_user_exists = class_room.find_user_by_id(&user_a_id);
    if if_user_exists.is_some() {
        println!("User exists");
        // delete user_a
        let class_room = class_room
            .delete_user_by_id(&user_a_id)
            .add_users(vec![user_a_clone.clone()]);
        println!("Class Room: {:#?}", class_room);
    } else {
        println!("User does not exist");
        let class_room = class_room.add_users(vec![user_a_clone]);
        println!("Class Room: {:#?}", class_room);
    }
}
