/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::mem::swap;

fn sort<T: PartialOrd + Clone>(array: &mut [T]){
	//TODO
    fn quick_sort<T: PartialOrd + Clone>(arr: &mut [T], l: usize, r: usize) -> &mut [T] {
        if r - l <= 1 {
            return arr;
        }
        let mid = arr[l].clone();
        let mut i = l;
        let mut j = r - 1;
        while i < j {
            while i < j {
                if arr[j] <= mid {
                    let tmpi = arr[i].clone();
                    let tmpj = arr[j].clone();
                    arr[i] = tmpj;
                    arr[j] = tmpi;
                    break;
                }
                j -= 1;
            }
            while i < j {
                if arr[i] > mid {
                    let tmpi = arr[i].clone();
                    let tmpj = arr[j].clone();
                    arr[i] = tmpj;
                    arr[j] = tmpi;
                    break;
                }
                i += 1;
            }
        }
        arr[i] = mid;
        let arr = quick_sort(arr, l, i);
        let arr = quick_sort(arr, i + 1, r);
        arr
    }
    let n = array.len();
    quick_sort(array, 0, n);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}