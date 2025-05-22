use anyhow::Result;
use ndarray::Array4;
use ort::{
    execution_providers::CPUExecutionProvider,
    session::{builder::GraphOptimizationLevel, Session},
    value::Tensor,
};
use std::path::PathBuf;

pub struct EmbeddingModel {
    session: Session,
}

impl EmbeddingModel {
    pub async fn new() -> Result<Self> {
        ort::init()
            .with_name("gnedby")
            .with_execution_providers([CPUExecutionProvider::default().build()])
            .commit()?;

        let model_path = Self::get_model_path().await?;
        let session = Session::builder()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .commit_from_file(model_path)?;

        Ok(Self { session })
    }

    async fn get_model_path() -> Result<PathBuf> {
        let config_dir = directories::BaseDirs::new()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .config_dir()
            .join("gnedby");
        let model_path = config_dir.join("model.onnx");
        if !model_path.exists() {
            anyhow::bail!("Model file not found. Please run 'gnedby embed load-model' first.");
        }
        Ok(model_path)
    }

    pub fn generate_embedding(&self, image: &Array4<f32>) -> Result<Vec<f32>> {
        let tensor: Tensor<f32> =
            Tensor::from_array((image.shape().to_vec(), image.as_slice().unwrap().to_vec()))?;
        let value = tensor.into_dyn();
        let outputs = self.session.run(ort::inputs![value]?)?;

        let output = outputs[0].try_extract_tensor::<f32>()?;
        let embedding = output.to_owned().into_raw_vec_and_offset().0;

        Ok(embedding)
    }
}
