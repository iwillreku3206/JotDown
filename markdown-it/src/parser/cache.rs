use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Cache {
    cache: HashMap<String, (String, usize)>,
    live_length: usize,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            live_length: 0,
        }
    }

    pub fn get_or_insert(
        &mut self,
        plugin_name: &str,
        key: &str,
        render_function: fn(&str) -> String,
    ) -> String {
        self.cache
            .entry(format!("{}_{}", plugin_name, key))
            .and_modify(|(_, lifetime)| {
                *lifetime += 1;
            })
            .or_insert_with(|| (render_function(key), self.live_length))
            .0
            .to_string()
    }

    pub fn clean(&mut self) {
        self.cache.retain(|_, v| {
            v.1 -= 1;
            v.1 > 0
        });
    }
}
