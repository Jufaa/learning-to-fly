use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct LayerTopology {
    pub input_neurons: usize,
    pub output_neurons: usize,
}

#[derive(Debug)]
pub struct Network{
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let mut built_layers = Vec::new();
        for adjacent_layers in layers.windows(2) {
            let input_neurons = adjacent_layers[0].output_neurons;
            let output_neurons = adjacent_layers[1].input_neurons;
            built_layers.push(Layer::random(rng, input_neurons, output_neurons));
        }
        
        Self { layers: built_layers }
    }
    pub fn new(layers: Vec<Layer>) -> Self {
       Self { layers }
    }
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut inputs = inputs;

        for layer in &self.layers{
            inputs = layer.propagate(inputs);
        }
        inputs

    }
}




#[derive(Debug)]
struct Layer{
    neurons: Vec<Neuron>,
}

impl Layer{
    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self{
        let mut neurons = Vec::new();

        for _ in 0..output_size{
            neurons.push(Neuron::random(rng, input_size));
        }

        Self {neurons}
    }
    fn propagate(&self, inputs: Vec<f32>)  -> Vec<f32> {
        let mut outputs = Vec::new();

        for neuron in &self.neurons{
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        outputs
    }
}


#[derive(Debug)]
struct Neuron{
    bias: f32,
    weights: Vec<f32>,
    
}

impl Neuron{
    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self{
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..1.0);
        
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self {bias, weights}
    }
    fn propagate(&self, inputs: &[f32]) -> f32{
        assert_eq!(inputs.len(), self.weights.len());
        let mut output = 0.0;

        for i in 0..inputs.len(){
            output += inputs[i] * self.weights[i];
        }
        output += self.bias;
        if output > 0.0{
            return output;
        }else{
            0.0;
        }
        output.max(0.0)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);

     assert_relative_eq!(
        neuron.weights.as_slice(),
        [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
    );
    }
    #[test]
    fn propagate() {
        let neuron = Neuron {
            bias: 0.5,
            weights: vec![-0.3, 0.8],
        };
    
        // Ensures `.max()` (our ReLU) works:
        assert_relative_eq!(
            neuron.propagate(&[-10.0, -10.0]),
            0.0,
        );
    
        // `0.5` and `1.0` chosen by a fair dice roll:
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );
    }
}