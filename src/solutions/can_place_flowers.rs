pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut planted = 0;
    if flowerbed.len() == 1 {
        if flowerbed[0] == 0 {
            return n <= 1;
        } else {
            return n == 0;
        }
    }
    for i in 0..flowerbed.len() {
        if i == 0 && flowerbed[i] == 0 {
            if flowerbed[i + 1] == 0 {
                planted += 1;
                flowerbed[i] = 1;
            }
        } else if i == flowerbed.len() - 1 && flowerbed[i] == 0 {
            if flowerbed[i - 1] == 0 {
                planted += 1;
                flowerbed[i] = 1;
            }
        } else if flowerbed[i] == 0 && flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
            planted += 1;
            flowerbed[i] = 1;
        }
    }
    planted >= n
}
