use termion::color;

pub fn displayworld(world: [[u8; 75]; 75])
{
    for i in 0..74 {
        for j in 0..74 {
            if world[i][j] == 1 
            {
                print!("{red}*", red = color::Fg(color::Red));
            }
            else 
            {
                print!(" ");
            }
        }
        println!("");
    }
}
