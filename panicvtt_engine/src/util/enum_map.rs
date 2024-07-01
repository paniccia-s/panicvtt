use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct EnumMap<E, V> {
    map: Vec<V>, 
    #[serde(skip_serializing, skip_deserializing)]
    _e: std::marker::PhantomData<E>
}

impl<E, V> EnumMap<E, V> where E : EnumCount + IntoEnumIterator, V : Clone { 

    pub fn from_value(initial: V) -> Self {
        let mut v = Vec::with_capacity(E::COUNT);
        for _ in E::iter() {
            v.push(initial.clone());
        }

        Self {
            map: v, 
            _e: Default::default()
        }
    }

    pub fn from_fn<F>(cb: F) -> Self where F : Fn(E) -> V {
        let mut v = Vec::with_capacity(E::COUNT); 
        for e in E::iter() {
            v.push(cb(e));
        }

        Self {
            map: v, 
            _e: Default::default()
        }
    }
}
 
impl<E, V> Index<E> for EnumMap<E, V> where E : Into<usize> {
    type Output = V;

    fn index(&self, index: E) -> &Self::Output {
        &self.map[index.into()]
    }
}

impl<E, V> IndexMut<E> for EnumMap<E, V> where E : Into<usize> {
    fn index_mut(&mut self, index: E) -> &mut Self::Output {
        &mut self.map[index.into()]
    }
}

impl<E, V> IntoIterator for EnumMap<E, V> where E : From<usize> {
    type Item = (E, V);

    type IntoIter = EnumMapIter<E, V>;

    fn into_iter(self) -> Self::IntoIter {
        EnumMapIter::new(self.map)
    }
}

pub struct EnumMapIter<E, V> {
    data: Vec<V>,
    index: usize,
    _e: std::marker::PhantomData<E>
}

impl<E, V> EnumMapIter<E, V> {
    pub fn new(data: Vec<V>) -> Self {
        Self {
            data, 
            index: 0, 
            _e: Default::default()
        }
    }
}

impl<E, V> Iterator for EnumMapIter<E, V> where 
    E : From<usize> {
    type Item = (E, V);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            None
        } else {
            // Take the current index, cast it to the current enum type, and pair it with its value
            let pair = (E::from(self.index), self.data.remove(0));
            self.index += 1;
            Some(pair)
        }
    }

} 


// impl<K, V> IntoIterator for SerdeEnumMap<K, V>
//     where K : enum_map::EnumArray<V>, K : Enum, V : Clone {
//     type Item = (K, V);

//     type IntoIter = IntoIter<K, V>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         self.map.into_iter()
//     }
// }

// use std::ops::{Index, IndexMut};

// use enum_map::{Enum, EnumMap, IntoIter, Iter, IterMut};
// use serde::de::Visitor;
// use serde::{Deserialize, Serialize};
// use serde::ser::SerializeStruct;

// pub struct SerdeEnumMap<K, V> 
//     where K : enum_map::EnumArray<V> + Enum, V : Clone {
//     map: EnumMap<K, V>,
// }

// impl<K, V> SerdeEnumMap<K, V>
//     where K : enum_map::EnumArray<V> + Enum, V : Clone {
//     pub fn from_default_value(v: V) -> Self {
//         Self {
//             map: EnumMap::from_fn(|_| v.clone())
//         }
//     }

//     pub fn from_fn<F>(f: F) -> Self 
//     where F : Fn(K) -> V {
//         Self {
//             map: EnumMap::from_fn(f)
//         }
//     }

//     pub fn iter(&self) -> Iter<K, V> {
//         self.map.iter()
//     }

//     pub fn iter_mut(&mut self) -> IterMut<K, V> {
//         self.map.iter_mut()
//     }
// }

// impl<K, V> Index<K> for SerdeEnumMap<K, V> 
//     where K : enum_map::EnumArray<V>, K : Enum, V : Clone {
//     type Output = V;

//     fn index(&self, index: K) -> &Self::Output {
//         &self.map[index]
//     }
// }

// impl<K, V> IndexMut<K> for SerdeEnumMap<K, V> 
//     where K : enum_map::EnumArray<V>, K : Enum, V : Clone {

//     fn index_mut(&mut self, index: K) -> &mut Self::Output {
//         &mut self.map[index]
//     }
// }

// impl<K, V> IntoIterator for SerdeEnumMap<K, V>
//     where K : enum_map::EnumArray<V>, K : Enum, V : Clone {
//     type Item = (K, V);

//     type IntoIter = IntoIter<K, V>;
    
//     fn into_iter(self) -> Self::IntoIter {
//         self.map.into_iter()
//     }
// }


// impl<K, V> Serialize for SerdeEnumMap<K, V>
//     where K : enum_map::EnumArray<V>, K : Enum, K : Into<&'static str>, V : Clone, V : Serialize {

//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where S: serde::Serializer {
//             let mut state = serializer.serialize_struct("map", self.map.len())?;
            
//             for (k, v) in &self.map {
//                 let key: &'static str = k.into();
//                 state.serialize_field(key, v)?;
//             }

//             state.end()
//         }
// }

// struct DesVisitor<K, V>    
//     where K : enum_map::EnumArray<V>, K : Enum, K : Into<&'static str>, V : Clone, V : Serialize {

//     _k: std::marker::PhantomData<K>,
//     _v: std::marker::PhantomData<V>
// }  

// impl<K, V> DesVisitor<K, V> 
//     where K : enum_map::EnumArray<V>, K : Enum, K : Into<&'static str>, V : Clone, V : Serialize {

//     fn new() -> Self {
//         Self {
//             _k: Default::default(),
//             _v: Default::default()
//         }
//     }
// }

// impl<'de, K, V> Visitor<'de> for DesVisitor<K, V> 
//     where K : enum_map::EnumArray<V>, K : Enum, K : Into<&'static str>, K : Deserialize<'de>, V : Clone, V : Serialize, V : Deserialize<'de>, V: Default {

//     type Value = SerdeEnumMap<K, V>;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("idk")
//     }

//     fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//         where
//             A: serde::de::MapAccess<'de>, {
//         let mut em: SerdeEnumMap<K, V> = SerdeEnumMap::from_default_value(V::default());

//         while let Some(key) = map.next_key::<K>()? {
//             em[key] = map.next_value()?;
//         }

//         Ok(em)
//     }
// }


// impl<'de, K, V> Deserialize<'de> for SerdeEnumMap<K, V>
//     where K : enum_map::EnumArray<V>, K : Enum, V : Clone {

//     fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
//         where D: serde::Deserializer<'de> {
//         let fields: Vec<&str> = std::vec::Vec::new();
//         _deserializer.deserialize_struct("SerdeEnumMap", &fields, DesVisitor::new())
//     }
// }

// #[cfg(test)]
// pub mod tests {
//     use crate::entities::skills::Skill;

//     use super::*;

//     #[test]
//     pub fn test() {
//         let map: SerdeEnumMap<Skill, i32> = SerdeEnumMap::from_default_value(0);
//         let s = serde_yaml::to_string(&map);
//         println!("{}", s.unwrap());
//         panic!("")
//     }
// }