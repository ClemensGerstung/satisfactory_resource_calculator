use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Resource {
  quantity: f64,
  name: String,
}

pub struct Recipe {
  input: Vec<Resource>,
  output: Vec<Resource>,
}

impl PartialEq for Resource {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}
impl Eq for Resource {}

impl Hash for Resource {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
  }
}

impl Recipe {
  pub fn default() -> Recipe {
    return Recipe {
      input: vec![],
      output: vec![],
    };
  }

  pub fn new(input: Vec<Resource>,
             output: Vec<Resource>) -> Recipe {
    return Recipe {
      input,
      output
    };
  }

  pub fn calculate(&self, rate: f64) -> Recipe {
    let mut copy = Recipe::default();

    for i in self.input.iter() {
      // copy.input.push(Resource {
      //   name: i.name.clone(),
      //   quantity: (i.quantity * rate),
      // });
    }

    return copy;
  }
}

fn main() {
  let mut r1 = Resource
  {
    name: String::from("Hello"),
    quantity: 1.0,
  };

  let recipe = Recipe::new(vec![r1], vec![]);

  let mut recipes: HashMap<&Resource, Vec<&Recipe>> = HashMap::new();
  recipes.insert(&recipe.input[0], vec![&recipe]);

  let mut rs = recipes.entry(&recipe.input[0]).or_insert(vec![]);
  rs.push(&recipe);
}
