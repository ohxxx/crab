pub fn halo() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "halo world";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
