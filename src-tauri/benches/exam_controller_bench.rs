use criterion::{criterion_group, criterion_main, Criterion};
use apallzac_tools_lib::exam_controller::ExamController;
use apallzac_tools_lib::belt_promotion_exam::{belts::BELTS, candidate::Candidate};

fn get_templates_path() -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{}/templates", manifest_dir)
}

fn create_test_candidate(name: &str, belt: BELTS) -> Candidate {
    Candidate {
        school: Some("Test School".to_string()),
        name: name.to_string(),
        trainer: "Test Trainer".to_string(),
        belt,
        belt_size: "CH".to_string(),
    }
}

fn bench_create_single_exam_page(c: &mut Criterion) {
    let candidate = create_test_candidate("John Doe", BELTS::AMARILLO);
    let templates_path = get_templates_path();
    
    c.bench_function("create_single_exam_page", |b| {
        b.iter(|| {
            let mut temp_controller = ExamController::new("19/04/2026", &templates_path);
            temp_controller.create_exam_page(std::hint::black_box(&candidate), std::hint::black_box("yellow.pdf"))
        })
    });
}

fn bench_create_multiple_exam_pages(c: &mut Criterion) {
    let candidates = vec![
        create_test_candidate("Candidate 1", BELTS::AMARILLO),
        create_test_candidate("Candidate 2", BELTS::CAFE1),
        create_test_candidate("Candidate 3", BELTS::VERDE),
        create_test_candidate("Candidate 4", BELTS::AZUL),
        create_test_candidate("Candidate 5", BELTS::NARANJA),
    ];
    let templates_path = get_templates_path();
    
    c.bench_function("create_multiple_exam_pages", |b| {
        b.iter(|| {
            let mut controller = ExamController::new("19/04/2026", &templates_path);
            for (i, candidate) in candidates.iter().enumerate() {
                let template = match i {
                    0 => "yellow.pdf",
                    1 => "brown1.pdf", 
                    2 => "green.pdf",
                    3 => "blue.pdf",
                    4 => "orange.pdf",
                    _ => "yellow.pdf",
                };
                controller.create_exam_page(std::hint::black_box(candidate), std::hint::black_box(template)).unwrap();
            }
        })
    });
}

fn bench_large_batch_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_batch_processing");
    let templates_path = get_templates_path();
    
    for &size in &[10, 25, 50] {
        let candidates: Vec<Candidate> = (0..size)
            .map(|i| create_test_candidate(&format!("Candidate {}", i), BELTS::AMARILLO))
            .collect();
            
        group.bench_with_input(format!("batch_{}", size), &size, |b, _| {
            b.iter(|| {
                let mut controller = ExamController::new("19/04/2026", &templates_path);
                for candidate in &candidates {
                    controller.create_exam_page(std::hint::black_box(candidate), std::hint::black_box("yellow.pdf")).unwrap();
                }
            })
        });
    }
    group.finish();
}


criterion_group!(
    benches,
    bench_create_single_exam_page,
    bench_create_multiple_exam_pages,
    bench_large_batch_processing,
);
criterion_main!(benches);
