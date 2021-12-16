use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Index;

pub struct Resource {
  quantity: f64,
  name: String,
}

pub struct Recipe {
  input: Vec<Resource>,
  output: Vec<Resource>,
}

pub struct Recipes<'a> {
  recipes: Vec<Recipe>,
  output_references: HashMap<&'a Resource, Vec<&'a Recipe>>,
}

impl Recipes<'_> {
  pub fn new<'a>() -> Recipes<'a> {
    return Recipes {
      recipes: vec![],
      output_references: HashMap::new()
    }
  }

  pub fn add_recipe(&'static mut self,
                    input: Vec<Resource>,
                    output: Vec<Resource>) -> &Recipe {

    self.recipes.push(Recipe { input, output });
    let recipe = self.recipes.index(self.recipes.len() - 1);

    for o in recipe.output.iter() {
      self.output_references.entry(&o).or_insert(vec![recipe.borrow()]);
    }

    return recipe;
  }

  pub fn calculate(&self, target: &Resource, rate: f64) -> Vec<&Resource> {
    let mut resources: Vec<&Resource> = vec![];

    let recipe = self.first_recipe(target);

    for input in recipe.input.iter() {
      let base_recipe = self.first_recipe(input);
      let found = match base_recipe.output.iter().find(|r| r.name == input.name)
      {
        Some(r) => r,
        None => continue
      };

      let input_rate = (rate * input.quantity) / found.quantity;
      let mut input_input = self.calculate(input, input_rate);
      input_input.push(input);

      for r in input_input {
        match resources.iter_mut().find(|f| f.name == r.name) {
          Some(d) => {
            d.quantity += r.quantity;
          },
          None => resources.push(r)
        };
      }
    }

    return resources;
  }

  fn first_recipe(&self, resource: &Resource) -> &Recipe {
    let elem = match self.output_references.get(resource) {
      Some(resource) => resource,
      None => panic!()
    };

    let recipe = match elem.first() {
      Some(r) => *r,
      None => panic!()
    };

    return recipe;
  }
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
      output,
    };
  }
}

fn main() {
  let r1 = Resource
  {
    name: String::from("Hello"),
    quantity: 1.0,
  };

  let r2 = Resource
  {
    name: String::from("Hello"),
    quantity: 1.0,
  };

}
