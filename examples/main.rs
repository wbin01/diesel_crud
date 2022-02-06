use diesel_crud;

fn main() {
	// Create
	let post_white_title = "White";
	let post_white_body = "(255, 255, 255)";
	let post_white = diesel_crud::create_the_draft(post_white_title, post_white_body);
	diesel_crud::create_post_publication(&post_white);
	println!("Created: {}", post_white_title);

	let post_black_title = "Black";
	let post_black_body = "(0, 0, 0)";
	let post_black = diesel_crud::create_the_draft(post_black_title, post_black_body);
	diesel_crud::create_post_publication(&post_black);
	println!("Created: {}", post_black_title);

	println!();

	// Read all
	println!("---------\nShow all posts:\n");

	let all_posts = diesel_crud::read_posts(None);
	for post in &all_posts {
		println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
        println!();
    }

	// Update
	println!("---------\nUpdates\n");

	println!("White -> Blue");
	diesel_crud::update_post(&post_white, "Blue", "(0, 0, 255)");
	
	println!("Black -> Red");
	diesel_crud::update_post(&post_black, "Red", "(255, 0, 0)");

	println!();

	// Read updated
	println!("---------\nShow updated posts:\n");

	let all_posts = diesel_crud::read_posts(None);
	for post in &all_posts {
		println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
        println!();
    }

    // Read Red post
	println!("---------\nShow post: Filter by 'Red':\n");

	let all_posts = diesel_crud::read_posts(Some("Red"));
	for post in &all_posts {
		println!("\tId: {}", post.id);
        println!("\tTitle: {}", post.title);
        println!("\tBody: {}", post.body);
        println!();
    }

	// Delete
	println!("---------\nDeleting all posts:\n");

	let all_posts = diesel_crud::read_posts(None);
	for post in all_posts {
		println!("\tDel post (id: {}, title: {})", post.id, post.title);
		diesel_crud::delete_post(&post);
    }
}