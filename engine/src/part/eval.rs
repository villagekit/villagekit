use std::path::PathBuf;

use serde_json::Value;

use super::PartRender;

trait PartEvaluator {
    async fn load(path: PathBuf);

    fn evaluate(params: Value) -> PartRender;
}
