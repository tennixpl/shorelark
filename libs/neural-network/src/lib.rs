use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;

pub struct Network {
    layers: Vec<Layer>,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

pub struct LayerTopology {
    pub neurons: usize,
}


impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        let layers = layers
        .windows(2)
        .map(|layers| {
            Layer::random(layers[0].neurons, layers[1].neurons)
        })
        .collect();

    Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        //let mut inputs = inputs;
        //Moved mut to function parameters  

        //for layer in &self.layers {
        //    inputs = layer.propagate(inputs);
        //}
        
        // above == below 

        self.layers
        .iter()
        .fold(inputs, |inputs, layer| layer.propagate(inputs))
        // This also removed need for mut in function paramters 


        //inputs
    }
}

impl Layer {
    //use rand_chacha::ChaCha8Rng;
    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let neurons = (0..output_neurons)
        .map(|_| Neuron::random(&mut rng , input_neurons))
        .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = Vec::with_capacity(self.neurons.len());

        for neuron in &self.neurons {
            let output = neuron.propagate(&inputs);
            outputs.push(output);
        }

        //self.neurons
        //.iter()
        //.map(|neuron| neuron.propagate(&inputs))
        //.collect()


        outputs
    }
}

impl Neuron {

    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        
        //let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);
    
        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
    
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        
        let mut output = 0.0;

        for i in 0..inputs.len() {
            output += inputs[i] * self.weights[i];
        }

        //for (&input, &weight) in inputs.iter().zip(&self.weights) {
        //    output += input * weight;
        //}

        output += self.bias;

        if output > 0.0 {
            output
        } else {
            0.0
        }

    }
    //    fn propagate(&self, inputs: &[f32]) -> f32 {
    //    let output = inputs
    //        .iter()
    //        .zip(&self.weights)
    //        .map(|(input, weight)| input * weight)
    //        .sum::<f32>();
    //
    //    (self.bias + output).max(0.0)
    //}
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn random_test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);
    
        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(neuron.weights, &[0.67383957, 0.8181262, 0.26284897, 0.5238807]);
    }

}


