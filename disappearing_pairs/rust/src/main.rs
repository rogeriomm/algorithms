// https://app.codility.com/programmers/trainings/4/disappearing_pairs/

use dubble::DoubleBuffered;

pub fn solution_disappearing_pairs_optimized(s: &[u8]) -> Vec<u8> {
    solution_disappearing_pairs_inefficient(s)
}

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_code)]
pub fn solution_disappearing_pairs_inefficient(s: &[u8]) -> Vec<u8> {

    // Lambda
    fn met(buffer : & mut DoubleBuffered<Vec<u8>>, occurence: u8) {
        let mut i = 0;

        buffer.update();
        buffer.write().clear();

        while i < buffer.read().len() - 1 {
            let value = buffer.read()[i];

            if ! (value == occurence && buffer.read()[i + 1] == occurence) {
                buffer.write().push(value);
            }
            else {
                i += 1;
            }

            i += 1;
        }

        if i == buffer.read().len() - 1 {
            let value = buffer.read()[i];

            buffer.write().push(value);
        }

    }

    // Buffer
    let mut buffer = DoubleBuffered::construct_with(Vec::<u8>::new);

    // Copy slice to buffer
    buffer.write().extend_from_slice(s);

    met(& mut buffer, b'A');
    met(& mut buffer, b'B');
    met(& mut buffer, b'C');

    buffer.update();

    let mut result = vec![];

    result.extend_from_slice(buffer.read());

    result
}

pub fn solution_disappearing_pairs(s: &[u8]) -> Vec<u8> {
    let inneficient = solution_disappearing_pairs_inefficient(s);
    let efficient  = solution_disappearing_pairs_optimized(s);
    assert_eq!(efficient, inneficient);
    inneficient
}

#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unreachable_code)]
fn main() {
    assert_eq!(solution_disappearing_pairs(b"BABABA"), b"BABABA");
    assert_eq!(solution_disappearing_pairs(b"ACCAABBC"), b"AC");
    assert_eq!(solution_disappearing_pairs(b"ACCAAAABBC"), b"AC");
    assert_eq!(solution_disappearing_pairs(b"ACCAAAABBBBBBBBBBC"), b"AC");
}
