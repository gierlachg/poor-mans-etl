use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use poor_man_etl::csv::extractor::CsvExtractor;
use poor_man_etl::csv::loader::CsvLoader;
use poor_man_etl::engine::Engine;
use poor_man_etl::loader::DiscardedOrder;
use poor_man_etl::reporter::Reporter;
use poor_man_etl::traderjoes::transformer::TraderJoesTransformer;
use poor_man_etl::transformer::DiscardedRecord;
use tempfile::tempfile;

#[test]
fn should_extract_from_traderjoes_csv_and_load_to_csv() {
    let mut extractor = create_extractor();
    let transformer = TraderJoesTransformer::new();
    let reporter = PanickingReporter {};
    let (mut loader, mut target_file) = create_loader();

    Engine::etl(&mut extractor, &transformer, &reporter, &mut loader);

    let mut loaded_content = String::new();
    target_file.seek(SeekFrom::Start(0)).unwrap();
    target_file.read_to_string(&mut loaded_content).unwrap();
    assert_eq!(loaded_content, "Order Id,Date Time,Product Id,Product Name,Quantity,Unit\n\
                    13,2019-08-27,123456789,Nuts,12,KG\n\
                    16,2019-08-28,987654321,Jam,1,KG\n");
}

fn create_extractor() -> CsvExtractor {
    let mut source_file = tempfile().unwrap();
    write!(source_file, "Order Number,Year,Month,Day,Product Number,Product Name,Count\n\
                  13,2019,8,27,123456789,Nuts,12\n\
                  16,2019,8,28,987654321,Jam,1").unwrap();
    source_file.seek(SeekFrom::Start(0)).unwrap();
    CsvExtractor::from(source_file).unwrap()
}

fn create_loader() -> (CsvLoader, File) {
    let target_file = tempfile().unwrap();
    let cloned_target_file = target_file.try_clone().unwrap();
    (CsvLoader::to(target_file).unwrap(), cloned_target_file)
}

struct PanickingReporter {}

impl Reporter for PanickingReporter {
    fn report_record(&self, _discarded_record: DiscardedRecord) {
        panic!("Record discarded - should never happen!");
    }

    fn report_order(&self, _discarded_order: DiscardedOrder) {
        panic!("Order discarded - should never happen!");
    }
}