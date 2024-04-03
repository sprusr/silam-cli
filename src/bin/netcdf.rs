use proj::Proj;

fn project_lon_lat(lon: f32, lat: f32) -> (f32, f32) {
    let from = "+proj=longlat";
    let to = "+proj=ob_tran +o_proj=longlat +o_lon_p=0 +o_lat_p=30";
    let projection = Proj::new_known_crs(&from, &to, None).unwrap();
    projection.convert((lon, lat)).unwrap()
}

fn find_closest(vec: &Vec<f32>, target: f32) -> Option<usize> {
    match vec.binary_search_by(|probe| probe.partial_cmp(&target).unwrap()) {
        Ok(index) => Some(index), // Exact match found
        Err(index) => {
            if index == 0 {
                Some(0) // Target is less than all elements, so closest is the first
            } else if index == vec.len() {
                Some(vec.len() - 1) // Target is greater than all elements, so closest is the last
            } else {
                // Check which of the neighbors is closer to the target
                let prev_diff = target - vec[index - 1];
                let next_diff = vec[index] - target;
                if prev_diff > next_diff {
                    Some(index)
                } else {
                    Some(index - 1)
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lat = 0.0;
    let lon = 0.0;

    let file = netcdf::open("silam_europe_pollen_v5_9_best.ncd.nc4")?;

    let rlon_variable = file.variable("rlon").expect("rlon variable missing");
    let rlat_variable = file.variable("rlat").expect("rlat variable missing");
    let poli_variable = file.variable("POLI").expect("POLI variable missing");
    let polisrc_variable = file.variable("POLISRC").expect("POLISRC variable missing");

    let (projected_lon, projected_lat) = project_lon_lat(lon, lat);

    let rlons: Vec<f32> = rlon_variable.get_values(..).unwrap();
    let closest_rlon_index = find_closest(&rlons, projected_lon).unwrap();

    let rlats: Vec<f32> = rlat_variable.get_values(..).unwrap();
    let closest_rlat_index = find_closest(&rlats, projected_lat).unwrap();

    let first_poli_at_coords: f32 =
        poli_variable.get_value((0, closest_rlat_index, closest_rlon_index))?;

    let first_polisrc_at_coords: f32 =
        polisrc_variable.get_value((0, closest_rlat_index, closest_rlon_index))?;

    print!(
        "{:?}",
        (
            closest_rlon_index,
            closest_rlat_index,
            first_poli_at_coords,
            first_polisrc_at_coords
        )
    );

    Ok(())
}
