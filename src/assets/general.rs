#[derive(Clone)]
pub struct ValueWithBaggage {
    pub value: f32,
    pub baggage: Vec<f32>,
}

pub fn merge_sort(input: Vec<ValueWithBaggage>) -> Vec<ValueWithBaggage> {
    let n = input.len();
    if n <= 1 {
        return input;
    }

    // let mut l1: Vec<ValueWithBaggage> = vec![];
    // let mut l2: Vec<ValueWithBaggage> = vec![];

    // for i in 0..n {
    //     if i <= (n as f32 / 2 as f32).ceil() as usize {
    //         l1.push(input[i].clone());
    //     } else {
    //         l2.push(input[i].clone());
    //     }
    // }
    let mid = n / 2;

    let mut l1: Vec<ValueWithBaggage> = Vec::with_capacity(mid);
    let mut l2: Vec<ValueWithBaggage> = Vec::with_capacity(n - mid);

    for (i, item) in input.into_iter().enumerate() {
        if i < mid {
            l1.push(item);
        } else {
            l2.push(item);
        }
    }

    l1 = merge_sort(l1);
    l2 = merge_sort(l2);

    return merge(l1, l2);
}

fn merge(mut a: Vec<ValueWithBaggage>, mut b: Vec<ValueWithBaggage>) -> Vec<ValueWithBaggage> {
    let mut c: Vec<ValueWithBaggage> = vec![];

    while a.len() > 0 && b.len() > 0 {
        if a[0].value > b[0].value {
            c.push(b[0].clone());
            b.remove(0);
        } else {
            c.push(a[0].clone());
            a.remove(0);
        }
    }

    while a.len() > 0 {
        c.push(a[0].clone());
        a.remove(0);
    }

    while b.len() > 0 {
        c.push(b[0].clone());
        b.remove(0);
    }

    return c;
}
