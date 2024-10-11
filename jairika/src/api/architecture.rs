use crate::Model;
use crate::{Error, Result};

/// The layer type for a neural network.
pub enum Layer {
    Input,
    Hidden,
    Output,
}

/// The activation function for a layer in a neural network.
/// Activation functions in this library are all element-wise.
pub enum Activation {
    // Non-parametric activation functions
    Linear,      // x
    Exponential, // exp(x)
    Sigmoid,     // 1 / (exp(-x) + 1)
    Softplus,    // log(exp(x) + 1)
    Tanh,        // ((exp(x) - exp(-x)) / (exp(x) + exp(-x)))
    Mish,        // x*tanh(softplus(x)) = x*tanh(log(exp(x) + 1))
    Swish,       // x * sigmoid(x) = x / (exp(-x) + 1) (also known as SiLU)
    Softsign,    // x / (|x| + 1)

    // Parametric activation functions
    Elu {
        alpha: f32,
    }, // x if x > 0 else alpha*exp(x) - 1 where alpha must be > 0 (by default, alpha=1.0)
    ReLU {
        threshold: f32,
        alpha: f32,
        max_value: f32,
    }, // x if threshold < x < max_value, alpha*x if x <= threshold else max_value (by default, threshold=0, alpha=0, and max_value=inf)
    SeLU {
        scale: f32,
        alpha: f32,
    }, // scale * (x if x > 0 else alpha*(exp(x) - 1)) where alpha=1.6732632423543772848170429916717 and scale=1.0507009873554804934193349852946
}

impl Model {
    /// Derives a new variant of the model.
    /// The new variant is basically a new branch starting from the current iteration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jairika as jrk;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     jrk::init("path/to/model/").await.unwrap();
    ///     let mut model = jrk::Model::new("path/to/model/").await.unwrap();
    ///     model.derive("variant").await.unwrap();
    /// }
    /// ```
    pub async fn derive(&mut self, variant: &str) -> Result<()> {
        self.entry
            .query(format!("DEFINE NAMESPACE IF NOT EXISTS {}", variant))
            .await
            .map_err(|e| Error::ModelCreationFailed(e.to_string()))?;

        //TODO: Write the code that copies the current iteration to the new variant.

        Ok(())
    }

    /// Adds a layer to the model.
    /// The layer can be an input, hidden, or output layer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jairika as jrk;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     jrk::init("path/to/model/").await.unwrap();
    ///     let mut model = jrk::Model::new("path/to/model/").await.unwrap();
    ///     model.add_layer(Layer::Input, "input_layer", vec![5], None, None).await.unwrap();
    /// }
    /// ```
    pub async fn add_layer(
        &mut self,
        class: Layer,
        name: &str,
        shape: Vec<usize>,
        activation: Option<Activation>,
        dropout: Option<f32>,
    ) -> Result<()> {
        self.entry
            .query(format!("DEFINE TABLE {} SCHEMAFULL TYPE NORMAL", name))
            .await
            .map_err(|e| Error::LayerAdditionFailed(e.to_string()))?;
        self.entry
            .query("DEFINE FIELD bias ON TABLE input_layer TYPE")
            .await
            .map_err(|e| Error::LayerAdditionFailed(e.to_string()))?;

        Ok(())
    }

    /// Removes a layer from the model.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jairika as jrk;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     jrk::init("path/to/model/").await.unwrap();
    ///     let mut model = jrk::Model::new("path/to/model/").await.unwrap();
    ///     model.remove_layer("input_layer").await.unwrap();
    /// }
    /// ```
    pub async fn remove_layer(&mut self, class: Layer, name: &str) -> Result<()> {
        self.entry
            .query(format!("REMOVE TABLE {}", name))
            .await
            .map_err(|e| Error::LayerRemovalFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_add_layer() {
        use crate::{init, Model};

        init("test_model_add_layer").await.unwrap();
        let mut model = Model::new("test_model_add_layer").await.unwrap();
        model
            .add_layer(Layer::Input, "input_layer", vec![64, 64], None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_model_remove_layer() {
        use crate::{init, Model};

        init("test_model_remove_layer").await.unwrap();
        let mut model = Model::new("test_model_remove_layer").await.unwrap();
        model
            .remove_layer(Layer::Input, "input_layer")
            .await
            .unwrap();
    }
}
