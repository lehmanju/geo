#[macro_use]
extern crate criterion;
extern crate geo;

use crate::geo::relate::Relate;
use criterion::Criterion;
use geo::{LineString, Polygon};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("relate medium.relate(medium)", |bencher| {
        let points = include!("../src/algorithm/test_fixtures/norway_main.rs");
        let sub_polygon = {
            let points = points[0..50].to_vec();
            let mut exterior = LineString::<f32>::from(points);
            exterior.close();
            Polygon::new(exterior, vec![])
        };

        let polygon = {
            let points = points[30..90].to_vec();
            let mut exterior = LineString::<f32>::from(points);
            exterior.close();
            Polygon::new(exterior, vec![])
        };

        bencher.iter(|| {
            criterion::black_box(criterion::black_box(&polygon).relate(&sub_polygon));
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
