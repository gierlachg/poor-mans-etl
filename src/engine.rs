use crate::reporter::Reporter;
use crate::extractor::Extractor;
use crate::loader::Loader;
use crate::record::Record;
use crate::transformer::Transformer;

pub struct Engine {}

impl Engine {
    pub fn etl<R: Record>(extractor: &mut dyn Extractor<R>,
                          transformer: &dyn Transformer<R>,
                          reporter: &dyn Reporter,
                          loader: &mut dyn Loader) {
        extractor.into_iter()
            .map(|record| transformer.transform(record))
            .for_each(|order| {
                match order {
                    Ok(order) => if let Err(discarded_order) = loader.load(order) {
                        reporter.report_order(discarded_order);
                    },
                    Err(discarded_record) => reporter.report_record(discarded_record),
                }
            });
    }
}
