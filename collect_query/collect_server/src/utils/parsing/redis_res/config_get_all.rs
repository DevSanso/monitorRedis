use std::error::Error;



#[derive(Default, Clone)]
pub struct ConfigTuple {
    pub name : String,
    pub value : String
}

pub fn parsing_config_get_all(res : String) -> Result<Vec<ConfigTuple>, Box<dyn Error>> {
    let str_p = res.as_str();

    let mut v = Vec::new();
    let mut is_even = false;
    let mut index = 1;
    let mut temp = ConfigTuple::default();
    let str_iter = str_p.split("\n");

    for raw_data in str_iter {
        is_even = index % 2 == 0;

        if is_even {
            temp.value = String::from(raw_data);
            v.push(temp.clone());

            temp.value.clear();
            temp.name.clear();
        }else {
            temp.name = String::from(raw_data);
        }

        index += 1;
    }
    
    Ok(v)
}