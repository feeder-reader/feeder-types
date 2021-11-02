#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
   pub id:      String,
   pub title:   String,
   pub link:    String,
   pub summary: String,
   pub source:  String,
   pub new:     bool,
}
