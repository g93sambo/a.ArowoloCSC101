
fn main() {
    let aps_levels = vec![
        vec!["APS 1-2", "Intern", "Paralegal", "Placement"],
        vec!["APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"],
        vec!["APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"],
        vec!["EL 1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"],
        vec!["EL 2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"],
        vec!["SES", "CEO", "Dean", "Partner", "Principal"],
    ];

    let mut position = String::new();
    println!("Enter the position:");
    std::io::stdin().read_line(&mut position).expect("not a valid position");
    position = position.trim().to_string();

    let mut experience_years = String::new();
    println!("Enter the number of years of experience:");
    std::io::stdin().read_line(&mut experience_years).unwrap();
    let experience_years: u32 = experience_years.trim().parse().unwrap();

    let mut aps_level = "Unknown APS level";

    for i in aps_levels.iter() {
        if i[1..].contains(&position.as_str()) {
            aps_level = i[0];
            break;
        }
    }

    println!("The APS level for {} with {} years of experience is {}", position, experience_years, aps_level);
}