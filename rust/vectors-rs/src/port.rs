//! The Rust port of `vectors.c`: safe Rust; `lib.rs` exports each function under its C name.
//!
//! The surface geometry helpers from EnergyPlus `Vectors.cc`, with the `Vector3D` operations and
//! `Plane::fromVertices` / `signedDistance` from `Geometry/` they use, in the same operation
//! order: polygon area, the Newell area and normal vectors, azimuth and tilt with the local
//! coordinate system, coplanarity, points in a plane, rounding, normalising. Nothing here panics.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "the exports keep the oracle's plain signatures"
)]

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0; // `Constant::DegToRad`
const SMALL_DISTANCE: f64 = 1.0e-4; // `Constant::SmallDistance`

/// `Vector3D`.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct V {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

const UNIT_X: V = V {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};
const UNIT_Y: V = V {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
const UNIT_Z: V = V {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

impl V {
    fn sub(self, b: V) -> V {
        V {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
        }
    }

    fn add_assign(&mut self, b: V) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    fn dot(self, v: V) -> f64 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    fn cross(self, v: V) -> V {
        V {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    /// `VecSquaredLength`.
    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// `VecLength`.
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// `VecNormalize`: zero for a zero vector.
    fn normalized(self) -> V {
        let len = self.length();
        if len != 0.0 {
            V {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            V::default()
        }
    }

    fn component(self, which: i32) -> f64 {
        match which {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => f64::NAN,
        }
    }
}

/// `AreaPolygon` (`Vectors.cc:100`).
pub fn area_polygon(p: &[V]) -> f64 {
    let nor = p[1].sub(p[0]).cross(p[2].sub(p[0])).normalized();
    let mut csum = V::default();
    for w in p.windows(2) {
        csum.add_assign(w[0].cross(w[1]));
    }
    csum.add_assign(p[p.len() - 1].cross(p[0]));
    0.5 * nor.dot(csum).abs()
}

/// `CreateNewellAreaVector` (`Vectors.cc:281`).
pub fn newell_area(p: &[V]) -> V {
    let mut out = V::default();
    let mut v1 = p[1].sub(p[0]);
    for &q in &p[2..] {
        let v2 = q.sub(p[0]);
        out.add_assign(v1.cross(v2));
        v1 = v2;
    }
    let inv = 1.0 / 2.0; // `operator/=`
    V {
        x: out.x * inv,
        y: out.y * inv,
        z: out.z * inv,
    }
}

/// `CreateNewellSurfaceNormalVector` (`Vectors.cc:306`).
pub fn newell_normal(p: &[V]) -> V {
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
    for (i, &c) in p.iter().enumerate() {
        let n = p[(i + 1) % p.len()];
        x += (c.y - n.y) * (c.z + n.z);
        y += (c.z - n.z) * (c.x + n.x);
        z += (c.x - n.x) * (c.y + n.y);
    }
    V { x, y, z }.normalized()
}

/// `DetermineAzimuthAndTilt` (`Vectors.cc:218`): azimuth, tilt, and lcsx, lcsy, lcsz.
pub fn azimuth_and_tilt(p: &[V], normal: V) -> (f64, f64, [V; 3]) {
    let lcsx = p[2].sub(p[1]).normalized();
    let lcsz = normal;
    let lcsy = lcsz.cross(lcsx);
    let costheta = lcsz.dot(UNIT_Z);
    let rotang_0 = if costheta.abs() < 1.0 - 1.12e-16 {
        let x2 = UNIT_Z.cross(lcsz);
        x2.dot(UNIT_Y).atan2(x2.dot(UNIT_X))
    } else {
        lcsx.dot(UNIT_Y).atan2(lcsx.dot(UNIT_X))
    };
    let mut tlt = normal.z.acos();
    tlt /= DEG_TO_RAD;
    let mut az = rotang_0 / DEG_TO_RAD;
    az = (450.0 - az) % 360.0;
    az += 90.0;
    if az < 0.0 {
        az += 360.0;
    }
    az %= 360.0;
    if (az - 360.0).abs() < 1.0e-3 {
        az = 0.0;
    } else if (az - 180.0).abs() < 1.0e-6 {
        az = 180.0;
    }
    if (tlt - 180.0).abs() < 1.0e-6 {
        tlt = 180.0;
    }
    (az, tlt, [lcsx, lcsy, lcsz])
}

/// `Plane::fromVertices` (`Geometry/Plane.cc:62`): `(a, b, c)` and `d`.
fn plane(p: &[V]) -> (V, f64) {
    let (mut a, mut b, mut c) = (0.0, 0.0, 0.0);
    let mut center = V::default();
    for (i, &v) in p.iter().enumerate() {
        let w = p[(i + 1) % p.len()];
        a += (v.y - w.y) * (v.z + w.z);
        b += (v.z - w.z) * (v.x + w.x);
        c += (v.x - w.x) * (v.y + w.y);
        center.add_assign(v);
    }
    let nrm = V { x: a, y: b, z: c };
    (nrm, -(center.dot(nrm) / p.len() as f64))
}

/// `Plane::signedDistance`, on a plane that is not degenerate.
fn signed_distance((nrm, d): (V, f64), q: V) -> f64 {
    ((nrm.x * q.x) + (nrm.y * q.y) + (nrm.z * q.z) + d) / nrm.length()
}

fn degenerate((nrm, _): (V, f64)) -> bool {
    nrm.dot(nrm) == 0.0 // `normal().length_squared() == 0.0`
}

/// `CalcCoPlanarNess` (`Vectors.cc:382`): coplanar, the largest distance and its vertex.
pub fn coplanarness(p: &[V]) -> (bool, f64, i32) {
    let (mut max_dist, mut error_vertex) = (0.0, 0);
    if p.len() < 3 {
        return (true, max_dist, error_vertex);
    }
    let pl = plane(p);
    if degenerate(pl) {
        return (true, max_dist, error_vertex);
    }
    for (vert, &q) in (1..).zip(p) {
        let dist = signed_distance(pl, q).abs();
        if dist > max_dist {
            max_dist = dist;
            error_vertex = vert;
        }
    }
    (max_dist.abs() <= SMALL_DISTANCE, max_dist, error_vertex)
}

/// `PointsInPlane` (`Vectors.cc:426`): the 1-based query vertices in the base polygon's plane, or
/// `None` (`ErrorFound`) for a degenerate base.
pub fn points_in_plane(base: &[V], query: &[V]) -> Option<Vec<i32>> {
    let pl = plane(base);
    if degenerate(pl) {
        return None;
    }
    Some(
        (1..)
            .zip(query)
            .filter(|&(_, &q)| signed_distance(pl, q).abs() < SMALL_DISTANCE)
            .map(|(i, _)| i)
            .collect(),
    )
}

/// `nint64(v * roundto) / roundto`, or `None` where the cast to int64 is undefined.
fn round(v: f64, roundto: f64) -> Option<f64> {
    let s = v * roundto;
    let r = s + if s >= 0.0 { 0.5 } else { -0.5 };
    (r > -9_223_372_036_854_775_808.0 && r < 9_223_372_036_854_775_808.0)
        .then(|| (r as i64) as f64 / roundto)
}

/// The five vertices and how many of them make the polygon, or `None` outside 3 to 5 (ADR-009).
fn poly(n: i32, c: [f64; 15]) -> Option<([V; 5], usize)> {
    let n = usize::try_from(n).ok().filter(|n| (3..=5).contains(n))?;
    Some((
        std::array::from_fn(|i| V {
            x: c[3 * i],
            y: c[3 * i + 1],
            z: c[3 * i + 2],
        }),
        n,
    ))
}

macro_rules! poly_fn {
    ($(#[$m:meta])* $name:ident($($pre:ident: $pt:ty),*) -> $ret:ty, $nan:expr, |$p:ident, $c:ident| $body:expr) => {
        $(#[$m])*
        pub fn $name(
            $($pre: $pt,)*
            n: i32,
            x1: f64, y1: f64, z1: f64,
            x2: f64, y2: f64, z2: f64,
            x3: f64, y3: f64, z3: f64,
            x4: f64, y4: f64, z4: f64,
            x5: f64, y5: f64, z5: f64,
        ) -> $ret {
            let $c = [x1, y1, z1, x2, y2, z2, x3, y3, z3, x4, y4, z4, x5, y5, z5];
            match poly(n, $c) {
                Some((all, n)) => {
                    let $p = &all[..n];
                    $body
                }
                None => $nan,
            }
        }
    };
}

poly_fn!(VAreaPolygon() -> f64, f64::NAN, |p, _c| area_polygon(p));
poly_fn!(VNewellArea(which: i32) -> f64, f64::NAN, |p, _c| {
    newell_area(p).component(which)
});
poly_fn!(VNewellNormal(which: i32) -> f64, f64::NAN, |p, _c| {
    newell_normal(p).component(which)
});
poly_fn!(
    /// Azimuth (0), tilt (1), then the components of lcsx, lcsy and lcsz (2 to 10).
    VAzimuthTilt(which: i32) -> f64, f64::NAN, |p, _c| {
        let (az, tilt, lcs) = azimuth_and_tilt(p, newell_normal(p));
        match which {
            0 => az,
            1 => tilt,
            2..=10 => lcs[(which as usize - 2) / 3].component((which - 2) % 3),
            _ => f64::NAN,
        }
    }
);
poly_fn!(
    /// Coplanar (1 or 0), the largest distance, or its vertex.
    VCoPlanar(which: i32) -> f64, f64::NAN, |p, _c| {
        let (co, max_dist, vertex) = coplanarness(p);
        match which {
            0 => if co { 1.0 } else { 0.0 },
            1 => max_dist,
            2 => f64::from(vertex),
            _ => f64::NAN,
        }
    }
);
poly_fn!(
    /// ErrorFound (-2), the count (-1), or the which-th vertex in the plane (0 past the end) of all
    /// five vertices against the first n.
    VPointsInPlane(which: i32) -> i32, -1, |p, c| {
        let query = poly(5, c).map(|(all, _)| all).unwrap_or_default();
        let found = points_in_plane(p, &query);
        match which {
            -2 => i32::from(found.is_none()),
            -1 => found.map_or(0, |v| v.len() as i32),
            _ => usize::try_from(which).ok().and_then(|w| found?.get(w).copied()).unwrap_or(0),
        }
    }
);

/// `VecRound`'s component `which`; NaN where `nint64` is undefined.
pub fn VRound(which: i32, x: f64, y: f64, z: f64, roundto: f64) -> f64 {
    match (round(x, roundto), round(y, roundto), round(z, roundto)) {
        (Some(x), Some(y), Some(z)) => V { x, y, z }.component(which),
        _ => f64::NAN,
    }
}

/// `VecNormalize`'s component `which`, `VecLength` (3) or `VecSquaredLength` (4).
pub fn VNormalize(which: i32, x: f64, y: f64, z: f64) -> f64 {
    let v = V { x, y, z };
    match which {
        3 => v.length(),
        4 => v.length_squared(),
        _ => v.normalized().component(which),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wall() -> Vec<V> {
        // A 2 by 1 south wall, counter-clockwise from upper left seen from outside (east is on
        // the right seen from the south).
        vec![
            V {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            V {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            V {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            V {
                x: 2.0,
                y: 0.0,
                z: 1.0,
            },
        ]
    }

    #[test]
    fn south_wall_faces_south_and_stands_upright() {
        let p = wall();
        assert_eq!(area_polygon(&p), 2.0);
        let (az, tilt, _) = azimuth_and_tilt(&p, newell_normal(&p));
        assert!((az - 180.0).abs() < 1e-9 && (tilt - 90.0).abs() < 1e-9);
    }

    #[test]
    fn coplanar_until_a_corner_moves_out() {
        let mut p = wall();
        assert!(coplanarness(&p).0);
        p[3].y = 0.01;
        assert!(!coplanarness(&p).0);
    }
}
