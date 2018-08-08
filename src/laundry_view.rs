
use super::helper;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct LaundryView {
    #[serde(rename="objects")]
    machines: Vec<LaundryMachine>
}

impl LaundryView{

    pub fn new() -> Result<LaundryView, String>{
        let raw_json = helper::curl_get("https://laundryview.com/api/currentRoomData?location=1364814");

        let view: LaundryView = match serde_json::from_slice(raw_json.as_ref()){
            Ok(view) => view,
            Err(e) => {
                let err = format!("Error creating Laundry View: couldn't parse jsoni {}", e);
                return Err(err);
            }
        };

       Ok(view)
    }

    pub fn update(&mut self) -> Result<(), String>{
        let raw_json = helper::curl_get("https://laundryview.com/api/currentRoomData?location=1364814");
        let new_view:LaundryView = match serde_json::from_slice(raw_json.as_ref()){
            Ok(view) => view,
            Err(e) => {
                let err = format!("Error updating Laundry View: couldn't parse json {}", e);
                return Err(err);
            }
        };
        
        self.machines = new_view.machines;

        Ok(())
    }

    pub fn get_status(&self) -> LaundryStatus {
        let mut washers: u8 = 0;
        let mut dryers: u8 = 0;
        for m in &self.machines {
            match m.which_type(){
                Some(MachineType::Washer) => {
                    washers += m.get_availability();
                },   
                Some(MachineType::Dryer) => {
                    dryers += m.get_availability();
                },
                _ => {}
            }   
        }
        LaundryStatus::new(washers, dryers)
    }
}

enum MachineType {
    Washer, Dryer
}

#[derive(Debug, Deserialize)]
struct LaundryMachine{
    #[serde(skip_serializing_if="Option::is_none")]
    time_left_lite: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    time_left_lite2: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    appliance_type: Option<String>,
}

impl LaundryMachine {
    fn which_type(&self) -> Option<MachineType> {
        match self.appliance_type.as_ref().map(|x| x.as_ref()) {
            Some("W") => Some(MachineType::Washer),
            Some("D") => Some(MachineType::Dryer),
            _ => None,
        } 
    }

    fn get_availability(&self) -> u8 {
        let mut count: u8 = 0;
        let available = Some(String::from("Available"));

        if self.time_left_lite == available {
            count += 1;
        } 

        if self.time_left_lite2 == available {
            count += 1;
        }

        count
    }
}

#[derive(Debug, Serialize)]
pub struct LaundryStatus{
    washers:u8,
    dryers:u8,
}

impl LaundryStatus {
    pub fn new(washers: u8, dryers:u8) -> LaundryStatus {
        LaundryStatus{washers, dryers}
    }

    pub fn to_u8(&self)-> Result<Vec<u8>, &'static str>{
        let serlialized:Vec<u8> = match serde_json::to_vec(self){
            Ok(result) => result,
            Err(_e) => {
                return Err("Error Serializing Laundry Status");
            }
        };

        Ok(serlialized)
    }
}
