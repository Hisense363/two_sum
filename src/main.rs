use std::collections::HashMap;

fn main() {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut checked: HashMap<i32, usize> = HashMap::new();
        let mut result = vec![0, 0];
        let mut counter = 0;
        while counter < nums.len() {
            let checker = target - nums[counter];
            if checked.contains_key(&checker){
                result[0] = checked[&checker] as i32;
                result[1] = counter as i32;
                break;
            }
            checked.insert(nums[counter], counter);
            counter += 1;
        }
        result
    }
}
