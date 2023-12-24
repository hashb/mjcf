# MJCF for Rust #

This is a rust interface to MJCF files. MJCF is the modeling language used by
[MuJoCo](https://mujoco.readthedocs.io/en/stable/overview.html).  
This uses the [serde](https://serde.rs/) library to convert between the
XML-based MJCF files and native rust data-structures.

For more information about the MJCF file format, see the official documentation:  
<https://mujoco.readthedocs.io/en/stable/XMLreference.html>

* [Crates.io](https://crates.io/crates/mjcf)
* [Docs.rs](https://docs.rs/crate/mjcf/latest)


### Installation & Usage ###

1) Install from crates.io:  
   `cargo add mjcf`

2) Read the documentation:  
   `cargo doc --package mjcf --open`  
   [And the corresponding MJCF XML Reference.](https://mujoco.readthedocs.io/en/stable/XMLreference.html)

3) Use the function `mjcf::from_str` to parse an MJCF XML string into the native rust data structures.  
   Use the function `mjcf::to_string` to serialize the rust data structures into an MJCF XML string.  


### Example ###
```rust
let mjcf_string = std::fs::read_to_string("my_mujoco_model.xml").unwrap();

// Deserialize the string into rust struct's.
let mut mujoco = mjcf::from_str(&mjcf_string).unwrap();

// Let's add another body to this model.
let worldbody = mujoco.worldbody.as_mut().unwrap();
let mut new_body = mjcf::Body::default();
new_body.name = "my_body".to_string();
worldbody.body.push(new_body);

// Serialize the modified data into a string.
let mjcf_string = mjcf::to_string(&mujoco).unwrap();

std::fs::write("modified_mujoco_model.xml", mjcf_string).unwrap();
```


### mjcf-fmt ###


### Tests ###

This library is tested using MJCF files which were scraped from the internet. In
order to run the tests you need to clone this GIT repository _and all of its
submodules_ using the following command:  
`git clone --recurse-submodules`

##### Test Asset Sources: #####
* [MuJoCo Menagerie](https://github.com/google-deepmind/mujoco_menagerie)
* [Gymnasium](https://gymnasium.farama.org/environments/mujoco/)
* [dm_control](https://github.com/google-deepmind/dm_control)

