pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut end = nums.len() - 1;
    while nums[end] == 0 {
        if end == 0 {
            return;
        }
        end -= 1;
    }
    if end > nums.len() {
        return;
    }
    let mut ind = 0;
    while ind < end {
        if nums[ind] == 0 {
            nums.remove(ind);
            nums.push(0);
            end -= 1;
        } else {
            ind += 1;
        }
    }
}
