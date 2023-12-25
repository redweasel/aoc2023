/*
--- Day 24: Never Tell Me The Odds ---

It seems like something is going wrong with the snow-making process. Instead of forming snow, the water that's been absorbed into the air seems to be forming hail!

Maybe there's something you can do to break up the hailstones?

Due to strong, probably-magical winds, the hailstones are all flying through the air in perfectly linear trajectories. You make a note of each hailstone's position and velocity (your puzzle input). For example:

19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3

Each line of text corresponds to the position and velocity of a single hailstone. The positions indicate where the hailstones are right now (at time 0). The velocities are constant and indicate exactly how far each hailstone will move in one nanosecond.

Each line of text uses the format px py pz @ vx vy vz. For instance, the hailstone specified by 20, 19, 15 @ 1, -5, -3 has initial X position 20, Y position 19, Z position 15, X velocity 1, Y velocity -5, and Z velocity -3. After one nanosecond, the hailstone would be at 21, 14, 12.

Perhaps you won't have to do anything. How likely are the hailstones to collide with each other and smash into tiny ice crystals?

To estimate this, consider only the X and Y axes; ignore the Z axis. Looking forward in time, how many of the hailstones' paths will intersect within a test area? (The hailstones themselves don't have to collide, just test for intersections between the paths they will trace.)

In this example, look for intersections that happen with an X and Y position each at least 7 and at most 27; in your actual data, you'll need to check a much larger test area. Comparing all pairs of hailstones' future paths produces the following results:

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 18, 19, 22 @ -1, -1, -2
Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone A.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths are parallel; they never intersect.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-6, y=-5).

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-2, y=3).

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone B.

Hailstone A: 12, 31, 28 @ -1, -2, -1
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

So, in this example, 2 hailstones' future paths cross inside the boundaries of the test area.

However, you'll need to search a much larger test area if you want to see if any hailstones might collide. Look for intersections that happen with an X and Y position each at least 200000000000000 and at most 400000000000000. Disregard the Z axis entirely.

Considering only the X and Y axes, check all pairs of hailstones' future paths for intersections. How many of these intersections occur within the test area?
*/

type N = i128;


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Vec3(pub N, pub N, pub N);

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl std::ops::Mul<N> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: N) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.1)
    }
}
impl std::ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.1)
    }
}
impl Vec3 {
    pub fn det_xy(&self, rhs: &Vec3) -> N {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    pub fn diff(&self) -> (N, N) {
        (self.1 - self.0, self.2 - self.0)
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(self.1 * rhs.2 - self.2 * rhs.1, self.2 * rhs.0 - self.0 * rhs.2, self.0 * rhs.1 - self.1 * rhs.0)
    }
}

pub struct Ray {
    pos: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Ray { pos, dir }
    }
}

impl TryFrom<&str> for Ray {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (pos, dir) = value.split_once('@').ok_or(())?;
        let mut pos = pos.split(',').filter_map(|num| num.trim().parse().ok());
        let pos = Vec3(pos.next().ok_or(())?, pos.next().ok_or(())?, pos.next().ok_or(())?);
        let mut dir = dir.split(',').filter_map(|num| num.trim().parse().ok());
        let dir = Vec3(dir.next().ok_or(())?, dir.next().ok_or(())?, dir.next().ok_or(())?);
        Ok(Ray::new(pos, dir))
    }
}

/// find the integer intersection of two rays.
/// returns the intersection point x with the condition
/// x <= intersection < x+1
/// and two bools (2d) for x == intersection
/// if there is no intersection in the integer range, return None
pub fn intersect_2d(a: &Ray, b: &Ray) -> Option<(Vec3, (bool, bool))> {
    // solve set of linear equations
    //    a.pos + x*a.dir = b.pos + y*b.dir
    // -> x*a.dir - y*b.dir = b.pos - a.pos
    let ab = b.pos - a.pos;
    let det = -a.dir.det_xy(&b.dir);
    if det == 0 {
        return None; // could also be infinitely many intersections!
    }
    // NOTE: the following line can cause panics!
    let xydet = Vec3(-b.dir.1, -a.dir.1, 0)*ab.0 + Vec3(b.dir.0, a.dir.0, 0)*ab.1;
    // solution is xy = xydet/det
    // however calculating that will round!
    // -> just use the sign of det for now
    // if any of the collisions happen in the past, that's not a collision
    let xydet = xydet * det.signum();
    let det = det.abs();
    if xydet.0 < 0 || xydet.1 < 0 {
        return None; // crossed in the past for one of the rays
    }
    // the resulting point is a.pos + x*a.dir
    // -> make sure the Vec3 that is returned,
    // represents the square that the intersection resides in.
    // q <= x/det < q+1  ->  det > 0: q*det <= x, det < 0: q*det >= x
    // There will still be the cornercase of q*det == x! so check that and return it as well.
    let p = Vec3((xydet.0 * a.dir.0).div_euclid(det),
                  (xydet.0 * a.dir.1).div_euclid(det),
                  (xydet.0 * a.dir.2).div_euclid(det));
    Some((a.pos + p,
        (p.0 * det == xydet.0 * a.dir.0,
         p.1 * det == xydet.0 * a.dir.1)))
}

#[test]
pub fn part1() {
    let mut rays: Vec<Ray> = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        rays.push(input.try_into().expect("failed to parse line"));
    }

    let min = 200000000000000i64 as N;
    let max = 400000000000000i64 as N;
    //let min = 7i64;
    //let max = 27i64;

    // test all combinations
    let mut count = 0;
    for (i, a) in rays.iter().enumerate() {
        for b in &rays[i+1..] {
            if let Some((x, eq)) = intersect_2d(a, b) {
                if min <= x.0 && (x.0 < max || x.0 == max && eq.0)
                && min <= x.1 && (x.1 < max || x.1 == max && eq.1) {
                    //println!("intersection at {}, {}", x.0, x.1);
                    count += 1;
                }
            }
        }
    }
    println!("found {count} intersections");
}

/*
--- Part Two ---

Upon further analysis, it doesn't seem like any hailstones will naturally collide. It's up to you to fix that!

You find a rock on the ground nearby. While it seems extremely unlikely, if you throw it just right, you should be able to hit every hailstone in a single throw!

You can use the probably-magical winds to reach any integer position you like and to propel the rock at any integer velocity. Now including the Z axis in your calculations, if you throw the rock at time 0, where do you need to be so that the rock perfectly collides with every hailstone? Due to probably-magical inertia, the rock won't slow down or change direction when it collides with a hailstone.

In the example above, you can achieve this by moving to position 24, 13, 10 and throwing the rock at velocity -3, 1, 2. If you do this, you will hit every hailstone as follows:

Hailstone: 19, 13, 30 @ -2, 1, -2
Collision time: 5
Collision position: 9, 18, 20

Hailstone: 18, 19, 22 @ -1, -1, -2
Collision time: 3
Collision position: 15, 16, 16

Hailstone: 20, 25, 34 @ -2, -2, -4
Collision time: 4
Collision position: 12, 17, 18

Hailstone: 12, 31, 28 @ -1, -2, -1
Collision time: 6
Collision position: 6, 19, 22

Hailstone: 20, 19, 15 @ 1, -5, -3
Collision time: 1
Collision position: 21, 14, 12

Above, each hailstone is identified by its initial position and its velocity. Then, the time and position of that hailstone's collision with your rock are given.

After 1 nanosecond, the rock has exactly the same position as one of the hailstones, obliterating it into ice dust! Another hailstone is smashed to bits two nanoseconds after that. After a total of 6 nanoseconds, all of the hailstones have been destroyed.

So, at time 0, the rock needs to be at X position 24, Y position 13, and Z position 10. Adding these three coordinates together produces 47. (Don't add any coordinates from the rock's velocity.)

Determine the exact position and velocity the rock needs to have at time 0 so that it perfectly collides with every hailstone. What do you get if you add up the X, Y, and Z coordinates of that initial position?
*/

// greatest common divisor (euclidean algorithm)
pub fn gcd(mut a: N, mut b: N) -> N {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// compute the integer solution to the matrix equation mat * x = b
/// If x is not an integer solution, return None
pub fn linsolve(mut mat: [[N; 9]; 9], mut b: [N; 9]) -> Option<[N; 9]> {
    // simplify all rows (divide by largest common divisor)
    for i in 0..9 {
        let div = gcd(mat[i].iter().copied().reduce(|a, b| gcd(a, b)).unwrap(), b[i]);
        mat[i].iter_mut().for_each(|x| *x /= div);
        b[i] /= div;
    }
    // do pivoted gaussian elimination to turn mat into a diagonal matrix
    for i in 0..9 {
        // for column i:
        // 1. choose the row that will have the diagonal entry (by chosing the smallest non zero value)
        let row = (i..9).filter(|j| mat[*j][i] != 0).min_by_key(|j| mat[*j][i].abs());
        if let Some(row) = row {
            if i != row {
                mat.swap(i, row);
                b.swap(i, row);
            }
        }
        else {
            return None; // not invertible
        }
        // 2. make all others 0
        let d = mat[i][i];
        for j in 0..9 {
            if i == j {
                continue;
            }
            let r = mat[j][i];
            let div = gcd(d, r);
            let mul_d = r / div;
            let mul_r = d / div;
            for k in 0..9 {
                mat[j][k] = mat[j][k] * mul_r - mat[i][k] * mul_d;
            }
            b[j] = b[j] * mul_r - b[i] * mul_d;
        }
        
        // simplify all rows (divide by largest common divisor)
        for i in 0..9 {
            let div = gcd(mat[i].iter().copied().reduce(|a, b| gcd(a, b)).unwrap(), b[i]);
            mat[i].iter_mut().for_each(|x| *x /= div);
            b[i] /= div;
        }
    }
    // if any row has a non 1 diagonal entry, the solution is rational!
    for i in 0..9 {
        if mat[i][i] == -1 {
            b[i] = -b[i];
        }
        else if mat[i][i] != 1 {
            return None;
        }
    }
    Some(b)
}

pub fn hailstone_smasher_throw(a: &Ray, b: &Ray, c: &Ray) -> Ray {
    // a_1.pos + a_1.dir*t_1 = r.pos + r.dir*t_1
    // a_2.pos + a_2.dir*t_2 = r.pos + r.dir*t_2
    // a_3.pos + a_3.dir*t_3 = r.pos + r.dir*t_3
    // is a system of non-linear equations since t_i and r.dir are both variables
    // (a_1.dir - r.dir)*t_1 = r.pos - a_1.pos
    // (a_2.dir - r.dir)*t_2 = r.pos - a_2.pos
    // (a_3.dir - r.dir)*t_3 = r.pos - a_3.pos
    // This system is geometrically moving around two triangles,
    // such that the coordinates of triangle 1 become
    // colinear to the coordinates of triangle 2.

    // NOTE: I went on a full tangent for finding t_1 and t_2 using the Newton Root Finding Method.
    // However I gave up, because it seemed unpractical.
    // the method with the t_1 and t_2 seems unstable and not very feasable with integers
    // -> try a different projection based method that I just came up with!

    // 0 = (r.pos - a_1.pos).cross(r.dir - a_1.dir)
    // 0 = (r.pos - a_2.pos).cross(r.dir - a_2.dir)
    // 0 = (r.pos - a_3.pos).cross(r.dir - a_3.dir)
    // are the equations to solve to get r
    // these are non-linear...
    // I can make these linear by expanding
    // r.pos.cross(r.dir) - a_1.pos.cross(r.dir) - r.pos.cross(a_1.dir) = a_1.dir.cross(a_1.pos)
    // r.pos.cross(r.dir) - a_2.pos.cross(r.dir) - r.pos.cross(a_2.dir) = a_2.dir.cross(a_2.pos)
    // r.pos.cross(r.dir) - a_3.pos.cross(r.dir) - r.pos.cross(a_3.dir) = a_3.dir.cross(a_3.pos)
    // and then renaming r.pos.cross(r.dir) =: rl
    // this way I get a linear system of 9 equations, with a unique solution!

    // The following is the python version of the function I need to implement:
    /*
    import numpy as np
    import scipy

    # 411575003091791, 556707064840497, 360903896339100 @ -249, -477, -135
    # 313152845430161, 438523628309611, 243549441983472 @ -111, 34, -45
    # 298029887761063, 330763685732924, 434735376373798 @ -29, 68, -473
    ap = np.array([411575003091791, 556707064840497, 360903896339100], dtype=np.float128)
    ad = np.array([-249, -477, -135], dtype=np.float128)
    bp = np.array([313152845430161, 438523628309611, 243549441983472], dtype=np.float128)
    bd = np.array([-111, 34, -45], dtype=np.float128)
    cp = np.array([298029887761063, 330763685732924, 434735376373798], dtype=np.float128)
    cd = np.array([-29, 68, -473], dtype=np.float128)

    mat = np.array([
        [0, -ad[2], ad[1],  0, ap[2], -ap[1], 1, 0, 0],
        [ad[2], 0, -ad[0],  -ap[2], 0, ap[0], 0, 1, 0],
        [-ad[1], ad[0], 0,  ap[1], -ap[0], 0, 0, 0, 1],
        [0, -bd[2], bd[1],  0, bp[2], -bp[1], 1, 0, 0],
        [bd[2], 0, -bd[0],  -bp[2], 0, bp[0], 0, 1, 0],
        [-bd[1], bd[0], 0,  bp[1], -bp[0], 0, 0, 0, 1],
        [0, -cd[2], cd[1],  0, cp[2], -cp[1], 1, 0, 0],
        [cd[2], 0, -cd[0],  -cp[2], 0, cp[0], 0, 1, 0],
        [-cd[1], cd[0], 0,  cp[1], -cp[0], 0, 0, 0, 1],
    ])
    b = np.concatenate([np.cross(ad, ap), np.cross(bd, bp), np.cross(cd, cp)])
    rp, rd, rl = np.reshape(scipy.linalg.solve(mat, b), (3, 3))
    print([round(x) for x in rp])
    print(rd)
    print(np.cross(rp, rd) - rl) # should be 0
    print(round(np.sum(rp))) # this is what the advent of code asks for! -> worked!
    */

    // Now for implementing this in Rust without libraries and with i128, I will have to get more creative...
    // I'm implementing my own solve routine for integers to do this.
    // I have no idea how else to solve this matrix... sympy didn't help, as its matrix inversion took too long to execute.

    // The column order is choosen such that the gcd can start to reduce the numbers as early as possible -> no overflow
    let mat = [
        [1, 0, 0,        0,        0, -a.dir.2,  a.pos.2,  a.dir.1, -a.pos.1],
        [0, 1, 0,  a.dir.2, -a.pos.2,        0,        0, -a.dir.0,  a.pos.0],
        [0, 0, 1, -a.dir.1,  a.pos.1,  a.dir.0, -a.pos.0,        0,        0],
        [1, 0, 0,        0,        0, -b.dir.2,  b.pos.2,  b.dir.1, -b.pos.1],
        [0, 1, 0,  b.dir.2, -b.pos.2,        0,        0, -b.dir.0,  b.pos.0],
        [0, 0, 1, -b.dir.1,  b.pos.1,  b.dir.0, -b.pos.0,        0,        0],
        [1, 0, 0,        0,        0, -c.dir.2,  c.pos.2,  c.dir.1, -c.pos.1],
        [0, 1, 0,  c.dir.2, -c.pos.2,        0,        0, -c.dir.0,  c.pos.0],
        [0, 0, 1, -c.dir.1,  c.pos.1,  c.dir.0, -c.pos.0,        0,        0],
    ];
    let ac = a.dir.cross(&a.pos);
    let bc = b.dir.cross(&b.pos);
    let cc = c.dir.cross(&c.pos);
    let b = [ac.0, ac.1, ac.2, bc.0, bc.1, bc.2, cc.0, cc.1, cc.2];
    let r = linsolve(mat, b).expect("failed to compute integer solution");
    let (rl, rp, rd) = (Vec3(r[0], r[1], r[2]), Vec3(r[3], r[5], r[7]), Vec3(r[4], r[6], r[8]));
    let ray = Ray::new(rp, rd);
    assert_eq!(rl, ray.pos.cross(&ray.dir), "the problem didn't have a solution for the given parameters");
    ray
}

#[test]
pub fn part2() {
    // if I understand correctly, the hailstones are guaranteed
    // to collide at integer coordinates and times!
    // That means they are all constructed in a certain way to make this possible.
    // a_i.pos + a_i.dir*t_i = r.pos + r.dir*t_i
    // lets say I would know the collision times t_i, then
    // a_1.pos + a_1.dir*t_1 = r.pos + r.dir*t_1
    // a_2.pos + a_2.dir*t_2 = r.pos + r.dir*t_2
    // -> r is a ray through a_1.pos + a_1.dir*t_1, a_2.pos + a_2.dir*t_2 with the given timing!
    // not knowing t_1, t_2, leaves 2 DoF.
    // a_3.pos + a_3.dir*t_3 = r.pos + r.dir*t_3
    // introduces another DoF, but also has 3 more equations -> "hopefully 0 DoF left"
    // So in principle 3 hailstones are enough to determine the ray.
    // for the rest I will just have to check if it holds true.

    let mut rays: Vec<Ray> = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        rays.push(input.try_into().expect("failed to parse line"));
    }

    // find 3 good ones with small positions
    // -> got lucky and it worked, however it's very much at it's numerical limit here!
    rays.sort_unstable_by_key(|ray| (ray.pos.0.abs() + ray.pos.1.abs() + ray.pos.2.abs()));
    let r = hailstone_smasher_throw(&rays[0], &rays[1], &rays[2]);
    println!("The ray that hits all hailstones is\n{}, {}, {} @ {}, {}, {}", r.pos.0, r.pos.1, r.pos.2, r.dir.0, r.dir.1, r.dir.2);
    println!("The xyz sum is {}", r.pos.0 + r.pos.1 + r.pos.2);
}