use anyhow::Result;
use image::{DynamicImage, GenericImageView};
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
        // ORT 환경 초기화
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
        // 캐시 디렉토리 사용
        let cache_dir = directories::BaseDirs::new()
            .ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?
            .cache_dir()
            .join("gnedby");

        std::fs::create_dir_all(&cache_dir)?;

        let model_path = cache_dir.join("mobilenetv2-12.onnx");

        if !model_path.exists() {
            println!("Downloading MobileNet model...");
            let model_url =
                "https://github.com/onnx/models/raw/main/validated/vision/classification/mobilenet/model/mobilenetv2-12.onnx";
            let response = reqwest::get(model_url).await?;
            let bytes = response.bytes().await?;
            std::fs::write(&model_path, bytes)?;
        }

        Ok(model_path)
    }

    // 이미지 전처리 함수
    fn preprocess_image(&self, img: &DynamicImage) -> Result<Array4<f32>> {
        // MobileNet 모델에서 사용하는 이미지 크기로 리사이즈
        let resized = img.resize_exact(224, 224, image::imageops::FilterType::Triangle);

        // MobileNet 입력 형식으로 변환 (1, 3, 224, 224)
        let mut array = Array4::<f32>::zeros((1, 3, 224, 224));

        // MobileNet 정규화 상수
        let mean = [0.485, 0.456, 0.406];
        let std = [0.229, 0.224, 0.225];

        // 이미지 데이터를 ndarray로 변환
        for (x, y, rgb) in resized.pixels() {
            let r = ((rgb[0] as f32) / 255.0 - mean[0]) / std[0];
            let g = ((rgb[1] as f32) / 255.0 - mean[1]) / std[1];
            let b = ((rgb[2] as f32) / 255.0 - mean[2]) / std[2];

            // NCHW 형식 (batch, channel, height, width)
            array[[0, 0, y as usize, x as usize]] = r;
            array[[0, 1, y as usize, x as usize]] = g;
            array[[0, 2, y as usize, x as usize]] = b;
        }

        Ok(array)
    }

    // 이미지 파일에서 임베딩 생성
    pub async fn embed_image_from_path(&self, path: &str) -> Result<Vec<f32>> {
        let img = image::open(path)?;
        self.embed_image(&img).await
    }

    // DynamicImage에서 임베딩 생성
    pub async fn embed_image(&self, img: &DynamicImage) -> Result<Vec<f32>> {
        let processed_image = self.preprocess_image(img)?;
        self.generate_embedding(&processed_image)
    }

    // 처리된 이미지 데이터에서 임베딩 생성
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
