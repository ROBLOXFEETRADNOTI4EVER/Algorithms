pub async fn insertion_sort(array: &mut Vec<i32, 999>) {
    let n = array.len();

    for i in 1..n {
        let temp = array[i];
        let mut j = i - 1;

        while j as i32 >= 0 && array[j] > temp {
            // CHANGE ONLY <temp for max-min and >temp to min-max
            array[j + 1] = array[j];
            j -= 1;
        }
        array[j + 1] = temp;
    }
}
