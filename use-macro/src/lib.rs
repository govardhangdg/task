use task::auto_vec;


// #[auto_vec]
// fn hello(_name: Vec<String>) -> Vec<String> {
//     vec!["hello".to_owned()]
// }

#[auto_vec]
fn yay<T: Into<i64>>(
  a: String, 
  b: Vec<i64>, 
  c: T,
) -> u64 {
    45
}



#[cfg(test)]
mod tests {

    use super::*;

    #[auto_vec]
    pub fn foo(arg1: i64, arg2: i32) -> f64 {
        return (arg1 + arg2 as i64) as f64;
    }


    #[test]
    fn first_test1() {
        let a = Vec::<f64>::new();
        assert_eq!(foo_vec(vec![], vec![]), a);
    }
    #[test]
    fn first_test2() {
        let a = vec![45, 54, 72, 98, 21, 105];
        let b = vec![47, 64, 98, 12, 65, 21];
        let c: Vec<f64> = vec![92.0, 118.0, 170.0, 110.0, 86.0, 126.0];
        assert_eq!(foo_vec(a, b), c);
    }
    #[test]
    #[should_panic]
    fn first_test3() {
        foo_vec(vec![1], vec![]);
    }

    #[test]
    fn second_test() {
        #[auto_vec]
        pub fn bar() -> String {
            "hello".to_owned()
        }
    }

    #[test]
    fn fourth_test() {
        #[auto_vec]
        pub fn ha(_a: i64) {
            ()
        }
    }
}