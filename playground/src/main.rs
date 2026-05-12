use autodiff::forward::Dual;
use autodiff::reverse::{Graph, Var};
use quaternion::quaternion::Quaternion;
use quaternion::vec3::Vec3;
use std::cell::RefCell;
use std::rc::Rc;

fn func1(x: Dual) -> Dual {
    let two = Dual(2.0, 0.0);
    (x * x) + (two * x)
}

fn func2(x: Dual) -> Dual {
    x.sin() * x.exp()
}

fn func3(x: Dual, y: Dual) -> Dual {
    let quo = x / y;
    (quo.sin() + quo - y.exp()) * (quo - y.exp())
}

fn main() {
    //forward

    // let x1 = Dual(3.0, 1.0);

    // let result1 = func1(x1);

    // println!("f(3) = {}", result1.0); // 15
    // println!("f'(3) = {}", result1.1); // 8

    // let x2 = Dual(std::f32::consts::PI, 1.0);

    // let result2 = func2(x2);

    // println!("f(pi)  = {:.4}", result2.0); // 0.
    // println!("f'(pi) = {:.4}", result2.1); // -23.1407

    let x3 = Dual(1., 1.);
    let y3_const = Dual(2., 0.);
    let x3_const = Dual(1., 0.);
    let y3 = Dual(2., 1.);

    let result3x = func3(x3, y3_const);
    let result3y = func3(x3_const, y3);

    println!("f(1,2) = {:.4}", result3x.0);
    println!("df(1,2)/dx = {:.4}", result3x.1);

    println!("f(1,2) = {:.4}", result3y.0);
    println!("df(1,2)/dy = {:.4}", result3y.1);

    //reverse

    // let mut graph1 = Graph::new();

    // let xg1 = graph1.add_node(std::f32::consts::FRAC_PI_2);
    // let yg1 = graph1.add_node(2.);

    // let sin_x = graph1.sin(xg1);
    // let exp_y = graph1.exp(yg1);

    // let target1 = graph1.mul(sin_x, exp_y);

    // graph1.backward(target1);

    // println!("f(pi/2, 2): {:.4}", graph1.nodes[target1].f);
    // println!("df(pi/2,2)/dx = {:.4}", graph1.nodes[xg1].df);
    // println!("df(pi/2,2)/dy = {:.4}", graph1.nodes[yg1].df);

    // let mut graph2 = Graph::new();

    // let xg2 = graph2.add_node(1.);
    // let yg2 = graph2.add_node(2.);

    // let quo = graph2.div(xg2, yg2);
    // let exp_y = graph2.exp(yg2);
    // let sin_quo = graph2.sin(quo);
    // let right = graph2.sub(quo, exp_y);
    // let left = graph2.add(sin_quo, right);
    // let target2 = graph2.mul(left, right);

    // graph2.backward(target2);

    // println!("f(1, 2): {:.4}", graph2.nodes[target2].f);
    // println!("df(1,2)/dx = {:.4}", graph2.nodes[xg2].df);
    // println!("df(1,2)/dy = {:.4}", graph2.nodes[yg2].df);

    // reverse with Var
    let graph3 = Rc::new(RefCell::new(Graph::new()));

    let x = Var::new(&graph3, 1.);
    let y = Var::new(&graph3, 2.);

    let quo = &x / &y;
    let right = &quo - &y.exp();
    let target3 = &(&quo.sin() + &right) * &right;

    target3.backward();

    println!("f(1, 2): {:.4}", target3.f());
    println!("df(1,2)/dx = {:.4}", x.df());
    println!("df(1,2)/dy = {:.4}", y.df());

    // --- Quaternion Tests ---
    println!("\n--- Quaternion Tests ---");
    let q1 = Quaternion(1.0, 2.0, 3.0, 4.0);
    let q2 = Quaternion(0.0, 1.0, 0.0, 0.0);

    let q3 = q1 + q2;
    println!("q1 + q2 = {:?}", q3);

    match q1.normalize() {
        Some(uq) => println!("Normalized q1: {:?}", uq.inner()),
        None => println!("q1 magnitude was close to or was 0!"),
    }

    // Let's do a real 3D rotation test!
    // We want to rotate the vector (1, 0, 0) by 90 degrees around the Z axis.
    let angle: f32 = std::f32::consts::FRAC_PI_2; // 90 degrees
    let w = (angle / 2.0).cos();
    let z = (angle / 2.0).sin();

    // Create the quaternion representing the rotation
    let rot_q = Quaternion(w, 0.0, 0.0, z);

    // The Typestate Pattern forces us to normalize it before rotating!
    let unit_rot = rot_q.normalize().unwrap();

    let v = Vec3(1.0, 0.0, 0.0);
    let v_rotated = v.rotate(unit_rot);

    println!("Original Vector: {:?}", v);
    // Should be roughly (0, 1, 0)
    println!("Rotated 90 degrees around Z-axis: {:?}", v_rotated);
}
