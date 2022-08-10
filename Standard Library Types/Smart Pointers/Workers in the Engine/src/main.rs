use workers_in_the_engine::engine::Engine;

fn main() {
    let mut engine = Engine::new();
    for id in 0..3 {
        engine.add_worker(id)
    }

    engine.run();
    println!("Log content:");
    engine.print_log();

    engine.run();
    println!("\nFinal log content:");
    engine.print_log();
}