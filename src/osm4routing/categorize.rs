// UNKNOWN accessiblity
const UNKNOWN: i8 = -1;

// FOOT_FORBIDDEN that no pedestrian is allowed
const FOOT_FORBIDDEN: i8 = 0;
// FOOT_ALLOWED pedestrians are allowed in both directions
const FOOT_ALLOWED: i8 = 1;

// CAR_FORBIDDEN no car is allowed
const CAR_FORBIDDEN: i8 = 0;
// CAR_RESIDENTIAL http://wiki.openstreetmap.org/wiki/Tag:highway%3Dresidential
const CAR_RESIDENTIAL: i8 = 1;
// CAR_TERTIARY http://wiki.openstreetmap.org/wiki/Tag:highway%3Dtertiary
const CAR_TERTIARY: i8 = 2;
// CAR_SECONDARY http://wiki.openstreetmap.org/wiki/Tag:highway%3Dsecondary
const CAR_SECONDARY: i8 = 3;
// car_forward http://wiki.http://wiki.openstreetmap.org/wiki/Tag:highway%3Dprimary
const CAR_PRIMARY: i8 = 4;
// CAR_TRUNK http://wiki.openstreetmap.org/wiki/Tag:highway%3Dtrunk
const CAR_TRUNK: i8 = 5;
// CAR_MOTORWAY http://wiki.openstreetmap.org/wiki/Tag:highway%3Dmotorway
const CAR_MOTORWAY: i8 = 6;

// BIKE_FORBIDDEN BIKE_ can not use this edge
const BIKE_FORBIDDEN: i8 = 0;
// BIKE_ALLOWED means that it can be used by a BIKE_, but the traffic might be shared with CAR_s
const BIKE_ALLOWED: i8 = 2;
// BIKE_LANE is a narrow lane dedicated for BIKE_, without physical separation from other traffic
const BIKE_LANE: i8 = 3;
// BIKE_BUSWAY means that BIKE_s are allowed on the bus lane
const BIKE_BUSWAY: i8 = 4;
// BIKE_TRACK is a physically separated for any other traffic
const BIKE_TRACK: i8 = 5;

const IN_NO_DIRECTION: i8 = 0;
const IN_BOTH_DIRECTIONS: i8 = 1;
const IN_POSITIVE_DIRECTION: i8 = 2;
const IN_NEGATIVE_DIRECTION: i8 = 3;

// Edgeself contains what mode can use the edge in each direction
#[derive(Clone, Copy, Default)]
pub struct EdgeProperties {
    pub foot: i8,
    pub car_forward: i8,
    pub car_backward: i8,
    pub bike_forward: i8,
    pub bike_backward: i8,
    pub maxspeed: i32,
    pub positive_speedlimit: i32,
    pub negative_speedlimit: i32,
    pub direction: i8,
}

impl EdgeProperties {
    pub fn default() -> EdgeProperties {
        EdgeProperties {
            foot: UNKNOWN,
            car_forward: UNKNOWN,
            car_backward: UNKNOWN,
            bike_forward: UNKNOWN,
            bike_backward: UNKNOWN,
            maxspeed: -1,
            positive_speedlimit:-1,
            negative_speedlimit:-1,
            direction: IN_BOTH_DIRECTIONS,
        }
    }

    // Normalize fills UNKNOWN fields
    pub fn normalize(&mut self) {

        // Speed limits for France see https://wiki.openstreetmap.org/wiki/OSM_tags_for_routing/Maxspeed#France
        let speedlimits: [i32;7] = [-1, 50, 90, 90, 90, 110, 130];

        if self.car_backward == UNKNOWN {
            self.car_backward = self.car_forward;
        }
        if self.bike_backward == UNKNOWN {
            self.bike_backward = self.bike_forward;
        }
        if self.car_forward == UNKNOWN {
            self.car_forward = CAR_FORBIDDEN;
        }
        if self.bike_forward == UNKNOWN {
            self.bike_forward = BIKE_FORBIDDEN;
        }
        if self.car_backward == UNKNOWN {
            self.car_backward = CAR_FORBIDDEN;
        }
        if self.bike_backward == UNKNOWN {
            self.bike_backward = BIKE_FORBIDDEN;
        }

        // Compute car positive_speedlimit
        if self.car_forward != 0 {
            if self.maxspeed != -1 {
                self.positive_speedlimit = self.maxspeed;
            }
            else {
                self.positive_speedlimit = speedlimits[self.car_forward as usize];
            }
        }

        // Compute car negative_speedlimit
        if self.car_backward != 0 {
            if self.maxspeed != -1 {
                self.negative_speedlimit = self.maxspeed;
            }
            else {
                self.negative_speedlimit = speedlimits[self.car_backward as usize];
            }
        }
        
        // Direction
        if self.car_forward == CAR_FORBIDDEN && self.car_backward == CAR_FORBIDDEN {
            self.direction = IN_NO_DIRECTION;
        }
        else if self.car_forward != CAR_FORBIDDEN && self.car_backward == CAR_FORBIDDEN {
            self.direction = IN_POSITIVE_DIRECTION;
        }
        else if self.car_forward == CAR_FORBIDDEN && self.car_backward != CAR_FORBIDDEN {
            self.direction = IN_NEGATIVE_DIRECTION;
        }
        else {
            self.direction = IN_BOTH_DIRECTIONS;
        }
    }

    // Accessible means that at least one mean of transportation can use it in one direction
    // If car_only is set to true then returns only valid roads for car
    pub fn accessible(self, car_only: bool) -> bool {

        if car_only == true {
            !(self.car_forward == CAR_FORBIDDEN && self.car_backward == CAR_FORBIDDEN)
        }
        else {
            self.bike_forward != BIKE_FORBIDDEN
            || self.bike_backward != BIKE_FORBIDDEN
            || self.car_forward != CAR_FORBIDDEN
            || self.car_backward != CAR_FORBIDDEN
            || self.foot != FOOT_FORBIDDEN
        }
        
    }

    pub fn update(&mut self, key_string: String, val_string: String) {
        let key = key_string.as_str();
        let val = val_string.as_str();
        self.update_with_str(key, val);
    }

    pub fn update_with_str(&mut self, key: &str, val: &str) {
        match key {
            "highway" => match val {
                "cycleway" | "path" | "footway" | "steps" | "pedestrian" => {
                    self.bike_forward = BIKE_TRACK;
                    self.foot = FOOT_ALLOWED;
                }
                "primary" | "primary_link" => {
                    self.car_forward = CAR_PRIMARY;
                    self.foot = FOOT_ALLOWED;
                    self.bike_forward = BIKE_ALLOWED;
                }
                "secondary" => {
                    self.car_forward = CAR_SECONDARY;
                    self.foot = FOOT_ALLOWED;
                    self.bike_forward = BIKE_ALLOWED;
                }
                "tertiary" => {
                    self.car_forward = CAR_TERTIARY;
                    self.foot = FOOT_ALLOWED;
                    self.bike_forward = BIKE_ALLOWED;
                }
                "unclassified" | "residential" | "living_street" | "road" | "service" | "track" => {
                    self.car_forward = CAR_RESIDENTIAL;
                    self.foot = FOOT_ALLOWED;
                    self.bike_forward = BIKE_ALLOWED;
                }
                "motorway" | "motorway_link" => {
                    self.car_forward = CAR_MOTORWAY;
                    self.foot = FOOT_FORBIDDEN;
                    self.bike_forward = BIKE_FORBIDDEN;
                }
                "trunk" | "trunk_link" => {
                    self.car_forward = CAR_TRUNK;
                    self.foot = FOOT_FORBIDDEN;
                    self.bike_forward = BIKE_FORBIDDEN;
                }
                _ => {}
            },
            "pedestrian" | "foot" => match val {
                "no" => self.foot = FOOT_FORBIDDEN,
                _ => self.foot = FOOT_ALLOWED,
            },

            // http://wiki.openstreetmap.org/wiki/Cycleway
            // http://wiki.openstreetmap.org/wiki/Map_Features#Cycleway
            "cycleway" => match val {
                "track" => self.bike_forward = BIKE_TRACK,
                "opposite_track" => self.bike_backward = BIKE_TRACK,
                "opposite" => self.bike_backward = BIKE_ALLOWED,
                "share_busway" => self.bike_forward = BIKE_BUSWAY,
                "lane_left" | "opposite_lane" => self.bike_backward = BIKE_LANE,
                _ => self.bike_forward = BIKE_LANE,
            },

            "bicycle" => match val {
                "no" | "false" => self.bike_forward = BIKE_FORBIDDEN,
                _ => self.bike_forward = BIKE_ALLOWED,
            },
            "busway" => match val {
                "opposite_lane" | "opposite_track" => self.bike_backward = BIKE_BUSWAY,
                _ => self.bike_forward = BIKE_BUSWAY,
            },
            "oneway" => match val {
                "yes" | "true" | "1" => {
                    self.car_backward = CAR_FORBIDDEN;
                    if self.bike_backward == UNKNOWN {
                        self.bike_backward = BIKE_FORBIDDEN;
                    }
                }
                _ => {}
            },
            "junction" => {
                if val == "roundabout" {
                    self.car_backward = CAR_FORBIDDEN;
                    if self.bike_backward == UNKNOWN {
                        self.bike_backward = BIKE_FORBIDDEN;
                    }
                }
            },
            "maxspeed" => {
                self.maxspeed = self.maxspeed_to_kmph(&val);
            }
            _ => {}
        }
    }

    /**
     * Convert an OSM speed in <countrycode>:<zone type> format to integer speed
     */
    pub fn cczone_to_speed(&mut self, val: &str) -> i32 {

        match val.to_ascii_uppercase().as_ref() {
            "FR:RURAL" => 80,
            "FR:URBAN" => 50,
            "FR:ZONE30" => 30,
            "FR:WALK" => 5,
            _ => {
                println!("Unknown - val={}", &val);
                -1
            }
        }

    }

    /**
     * Convert an OSM maxspeed of source:maxspeed tag value (string) to a km/h value (i8)
     * The allowed format for input val (see https://wiki.openstreetmap.org/wiki/Key:maxspeed)
     *   - maxspeed=60
     *   - maxspeed=50 mph
     *   - maxspeed=10 knots
     *   - maxspeed=none
     *   - maxspeed=walk
     *   - maxspeed=<countrycode>:<zone type>
     */
    pub fn maxspeed_to_kmph(&mut self,val: &str) -> i32 {

        let mut maxspeed: i32 = 0;

        // Speed mulitplier to convert in km/h
        let mut multiplier: f32 = 1.0;

        // First split into speed and unit
        let vec: Vec<&str> = val.trim().split(' ').collect();

        for (i, value) in vec.iter().enumerate() {
            
            if i == 0 {

                match value.as_ref() {
                    "none" => {
                        maxspeed = -99;
                    },
                    "walk" => {
                        maxspeed = 5;
                    }
                    _ => {

                        maxspeed = match value.parse::<i32>() {

                            // Integer
                            Ok(i) => i,

                            // Not an integer - should be // <countrycode>:<zone type>
                            Err(_e) => {
                                self.cczone_to_speed(&value)
                            }
                        };
                    }
                }

            }

            if i == 1 {

                match value.as_ref() {
                    "knots" => {
                        multiplier = 1.852;
                    },
                    "mph" => {
                        multiplier = 1.60934;
                    },
                    _ => {
                        multiplier = 1.0;
                    }
                }

            }

        }

        ((maxspeed as f32) * multiplier) as i32

    }
}

#[test]
fn test_accessible() {
    let mut p = EdgeProperties::default();
    p.normalize();
    assert!(!p.accessible());

    p.foot = FOOT_ALLOWED;
    assert!(p.accessible())
}

#[test]
fn test_normalize() {
    let mut p = EdgeProperties::default();
    p.bike_forward = BIKE_LANE;
    p.normalize();
    assert_eq!(BIKE_LANE, p.bike_backward);
    p.bike_forward = BIKE_ALLOWED;
    p.normalize();
    assert_eq!(BIKE_LANE, p.bike_backward);

    p.car_forward = CAR_SECONDARY;
    p.car_backward = UNKNOWN;
    p.normalize();
    assert_eq!(CAR_SECONDARY, p.car_backward)
}

#[test]
fn test_update() {
    let mut p = EdgeProperties::default();
    p.update_with_str("highway", "secondary");
    assert_eq!(CAR_SECONDARY, p.car_forward);

    p.update_with_str("highway", "primary_link");
    assert_eq!(CAR_PRIMARY, p.car_forward);

    p.update_with_str("highway", "motorway");
    assert_eq!(CAR_MOTORWAY, p.car_forward);

    p.update_with_str("highway", "residential");
    assert_eq!(CAR_RESIDENTIAL, p.car_forward);

    p.update_with_str("highway", "tertiary");
    assert_eq!(CAR_TERTIARY, p.car_forward);

    p.update_with_str("highway", "trunk");
    assert_eq!(CAR_TRUNK, p.car_forward);

    p.update_with_str("highway", "cycleway");
    assert_eq!(BIKE_TRACK, p.bike_forward);
    assert_eq!(FOOT_ALLOWED, p.foot);

    p.update_with_str("foot", "designated");
    assert_eq!(FOOT_ALLOWED, p.foot);

    p.update_with_str("foot", "no");
    assert_eq!(FOOT_FORBIDDEN, p.foot);

    p.update_with_str("cycleway", "lane");
    assert_eq!(BIKE_LANE, p.bike_forward);

    p.update_with_str("cycleway", "track");
    assert_eq!(BIKE_TRACK, p.bike_forward);

    p.update_with_str("cycleway", "opposite_lane");
    assert_eq!(BIKE_LANE, p.bike_backward);

    p.update_with_str("cycleway", "opposite_track");
    assert_eq!(BIKE_TRACK, p.bike_backward);

    p.update_with_str("cycleway", "opposite");
    assert_eq!(BIKE_ALLOWED, p.bike_backward);

    p.update_with_str("cycleway", "share_busway");
    assert_eq!(BIKE_BUSWAY, p.bike_forward);

    p.update_with_str("cycleway", "lane_left");
    assert_eq!(BIKE_LANE, p.bike_backward);

    p.update_with_str("bicycle", "yes");
    assert_eq!(BIKE_ALLOWED, p.bike_forward);

    p.update_with_str("bicycle", "no");
    assert_eq!(BIKE_FORBIDDEN, p.bike_forward);

    p.update_with_str("busway", "yes");
    assert_eq!(BIKE_BUSWAY, p.bike_forward);

    p.update_with_str("busway", "opposite_track");
    assert_eq!(BIKE_BUSWAY, p.bike_backward);

    p.update_with_str("oneway", "yes");
    assert_eq!(BIKE_FORBIDDEN, p.car_backward);
    assert!(p.bike_backward != BIKE_FORBIDDEN);

    p.bike_backward = UNKNOWN;
    p.update_with_str("oneway", "yes");
    assert_eq!(BIKE_FORBIDDEN, p.bike_backward);

    p.update_with_str("junction", "roundabout");
    assert_eq!(BIKE_FORBIDDEN, p.car_backward);

    p.bike_backward = UNKNOWN;
    p.update_with_str("junction", "roundabout");
    assert_eq!(BIKE_FORBIDDEN, p.bike_backward);
}
