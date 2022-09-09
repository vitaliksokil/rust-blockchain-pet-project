pub fn pagination(page: Option<u32>) -> (u32, u32) {
    let per_page: u32 = 25;
    match page {
        Some(page) => {
            let from_index = (page * per_page) - per_page;
            (from_index, per_page)
        }
        None => (0, per_page)
    }
}