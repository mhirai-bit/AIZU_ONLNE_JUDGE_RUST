//Iffucual House

fn main() {
    let mut buildings = vec![];
    buildings.push(Building::new());
    buildings.push(Building::new());
    buildings.push(Building::new());
    buildings.push(Building::new());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let b: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let f: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let r: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: i8 = iter.next().unwrap().parse::<i8>().unwrap();

        buildings[b].floors[f].rooms[r] += v;
    }

    let mut building_count = 0;
    for building in &buildings {
        for floor in &building.floors {
            for room in &floor.rooms{
                print!(" {}", *room);
            }
            println!();
        }
        building_count += 1;
        if building_count < buildings.len(){
            println!("{}", "#".repeat(20));
        }
    }
}

struct Building {
    floors: Vec<Floor>
}

struct Floor {
    rooms: Vec<i8>
}

impl Building {
    fn new() -> Building {
        let mut building = Building {
            floors: vec![]
        };
        // pushing 3 floors
        building.floors.push(Floor::new());
        building.floors.push(Floor::new());
        building.floors.push(Floor::new());
        building
    }
}

impl Floor {
    fn new() -> Floor {
        let mut floor = Floor {
            rooms: vec![]
        };
        // initialize with 0 tenant for each room
        for _ in 0..10 {
            floor.rooms.push(0);
        }
        floor
    }   
}
