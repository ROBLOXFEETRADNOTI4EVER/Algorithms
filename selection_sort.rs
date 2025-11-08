pub async fn selection_sort(array: &mut Vec<i32, 999>) {
    let n = array.len() - 1;
    for i in 0..n {
        let mut min = i;
        for mut j in i + 1..array.len() {
            if array[min] > array[j] {
                // > for to start with lowerst < to start with highest
                min = j;
            }
        }
        let temp = array[i];
        array[i] = array[min];
        array[min] = temp;
    }
}
