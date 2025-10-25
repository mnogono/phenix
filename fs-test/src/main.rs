use data::analysis_source_repository::AnalysisSourceRepository;

fn main() {
    let repository = AnalysisSourceRepository {
        dir: String::from("/ProgramData/s8/data/sources"),
    };
    let _list = repository.find_all();
    for it in _list {
        println!("{it:?}");
    }
}
