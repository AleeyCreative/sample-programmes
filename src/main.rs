pub mod email_extractor;
pub mod quadratic_eqn;

use email_extractor::email_extractor::mail_extractor;
use quadratic_eqn::quadratic_eqn_solver;

fn main() {
    println!("Testing for mail extractor:");
    mail_extractor("there he sings, ababa baba ali abbaba and anofied@gmail.com baba.ali@flexisaf.com");

    println!("\n Testing for quadratic equation solver:");
    println!("The equation is 4x + 12x*2 + 48");
    quadratic_eqn_solver(4,12,48);
}
