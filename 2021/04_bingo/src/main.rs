mod game;
use game::Game;
fn main() {
    let mut game = Game::new("inputs.txt");
    let mut last_draw = 0;
    while game.count_grids() > 1 {
        while game.has_a_winning_grid().is_none()
        {
            game.draw();
        }
        game.remove_grid(game.has_a_winning_grid().unwrap());
    }
    
    while game.has_a_winning_grid().is_none()
    {
        last_draw = game.draw();
    }
    println!("Loosing score : {}", game.get_score(game.has_a_winning_grid().unwrap(), last_draw));
}
