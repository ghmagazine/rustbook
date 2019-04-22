struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

fn main() {
    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));

    assert_eq!((triangle.1).0, 3);
}
