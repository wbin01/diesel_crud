use diesel_crud;

fn main() {
	let num = 8;

    // Create
    println!("{}\nCreate:", "_".repeat(num));

    let post_white_title = "White";
    let post_white_body = "(255, 255, 255)";
    let post_white = diesel_crud::create_the_draft(post_white_title, post_white_body);
    diesel_crud::create_post_publication(&post_white);
    println!("\tCreated: {}", post_white_title);

    let post_black_title = "Black";
    let post_black_body = "(0, 0, 0)";
    let post_black = diesel_crud::create_the_draft(post_black_title, post_black_body);
    diesel_crud::create_post_publication(&post_black);
    println!("\tCreated: {}", post_black_title);

    // Read all
    println!("{}\nRead all posts:", "_".repeat(num));

    let all_posts = diesel_crud::read_posts(None);
    for post in &all_posts {
    	println!();
        println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
    }

    // Update
    println!("{}\nUpdate:", "_".repeat(num));

    println!("\tWhite -> Blue");
    diesel_crud::update_post(&post_white, "Blue", "(0, 0, 255)");
    
    println!("\tBlack -> Red");
    diesel_crud::update_post(&post_black, "Red", "(255, 0, 0)");

    // Read updated
    println!("{}\nRead updated posts:", "_".repeat(num));

    let all_posts = diesel_crud::read_posts(None);
    for post in &all_posts {
    	println!();
        println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
    }

    // Read Red post
    println!("{}\nRead post filtered by 'Red':", "_".repeat(num));

    let all_posts = diesel_crud::read_posts(Some("Red"));
    for post in &all_posts {
        println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
    }

    // Delete
    println!("{}\nDelete:", "_".repeat(num));

    let all_posts = diesel_crud::read_posts(None);
    for post in all_posts {
        println!("\tDel post (id: {}, title: {})", post.id, post.title);
        diesel_crud::delete_post(&post);
    }
}