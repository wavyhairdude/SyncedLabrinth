mod birthday;
mod showroom;

static PROGRAM: u8 = 0;

fn main() {
    if PROGRAM == 0 {
        birthday::cupcake();
    }
    if PROGRAM == 1 {
        showroom::vase();
    }
}
