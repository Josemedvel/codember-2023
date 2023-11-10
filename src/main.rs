mod challenge_01;
use challenge_01::challenge_01;
fn main() {
    challenge_01(Some("llaveS casa CASA casa llaves".to_string()));
    challenge_01(Some("taza ta za taza".to_string()));
    challenge_01(Some("casas casa casasas".to_string()));
    challenge_01(None);
}
