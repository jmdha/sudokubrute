use criterion::criterion_group;
use criterion::BenchmarkId;
use criterion::Criterion;
use sudokubrute::board::Board;
use sudokubrute::solve::solve;
use sudokubrute::solve::SolveKind;

const DATA: [&'static str; 5] = [
    "004300209005009001070060043006002087190007400050083000600000105003508690042910300",
    "040100050107003960520008000000000017000906800803050620090060543600080700250097100",
    "600120384008459072000006005000264030070080006940003000310000050089700000502000190",
    "497200000100400005000016098620300040300900000001072600002005870000600004530097061",
    "005910308009403060027500100030000201000820007006007004000080000640150700890000420",
];

fn bench_solve(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");
    for i in DATA.iter() {
        let board = Board::generate(*i).unwrap();
        group.bench_with_input(BenchmarkId::new("Backtracking", i), i, |b, _| {
            b.iter(|| solve(SolveKind::Backtracking, board.clone()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_solve);
